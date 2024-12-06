// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::env::home_dir;
use std::path::PathBuf;
use std::thread;

use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Sqlite, SqlitePool};

#[cfg(target_os = "windows")]
use std::mem;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT};

#[cfg(target_os = "windows")]
fn send_key_event(vk: u16, flags: u32) {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: mem::zeroed(), // Use mem::zeroed() for the union
        };

        *input.u.ki_mut() = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        };

        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(target_os = "windows")]
fn get_vk_code(c: char) -> Option<u16> {
    match c {
        'A'..='Z' => Some(c as u16),
        'a'..='z' => Some(c.to_ascii_uppercase() as u16),
        '0'..='9' => Some(c as u16),
        '\n' => Some(0x0D), // Enter key
        '\r' => Some(0x0D), // Enter key
        ' ' => Some(0x20),  // Space key
        _ => Some(
            Some(c.to_ascii_lowercase() as u16).expect("Could not convert Character to ascii {:?}"),
        ),
    }
}

#[cfg(target_os = "windows")]
fn type_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(Duration::from_millis(delay));
    const KEYEVENTF_KEYDOWN: u32 = 0x0000;
    const KEYEVENTF_KEYUP: u32 = 0x0002;

    for c in text.chars() {
        if let Some(vk) = get_vk_code(c) {
            // Check if the character is uppercase to add shift key
            let needs_shift = c.is_ascii_uppercase();

            if needs_shift {
                // Press Shift key
                send_key_event(0x10, KEYEVENTF_KEYDOWN); // VK_SHIFT
            }

            // Press the key down
            send_key_event(vk, KEYEVENTF_KEYDOWN);

            // Release the key
            send_key_event(vk, KEYEVENTF_KEYUP);

            if needs_shift {
                // Release Shift key
                send_key_event(0x10, KEYEVENTF_KEYUP); // VK_SHIFT
            }

            // Add a small delay between keystrokes
            thread::sleep(Duration::from_millis(50));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn type_text(text: &str, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
    use std::{process::Command, time::Duration};

    thread::sleep(Duration::from_millis(delay));
    // Check if running on Wayland
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();

    if is_wayland {
        // Use wtype for Wayland
        Command::new("wtype")
            .args(&["-d", "50", text])
            .status()
            .expect("Failed to use wtype");
    } else {
        // Fallback to X11 method
        Command::new("xdotool")
            .args(&["type", "--delay", "50", text])
            .status()
            .expect("Failed to use xdotool");
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn type_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(Duration::from_millis(delay));
    // macOS-specific typing method
    // You might use AppleScript or other macOS-specific input simulation
    println!("Typed on macOS: {}", text);
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn typetext(text: &str, delay: u64) {
    // Give time to switch to target window
    type_text(text, delay).expect("Failed to type text");
}

async fn create_schema(url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&url).await?;
    let qry = "
        PRAGMA foreign_keys = ON ; 
        CREATE TABLE IF NOT EXISTS notes (
        id          INTEGER     PRIMARY KEY AUTOINCREMENT,
        note        TEXT        NOT NULL,
        project     TEXT
        );";
    let result = sqlx::query(qry).execute(&pool).await;
    pool.close().await;
    return result;
}

fn get_sqlite_url() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".config/toolbar/notes.db");

    // Ensure the directory exists
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");

    path
}

#[tauri::command]
async fn createdb() {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        Sqlite::create_database(&url).await.unwrap();
        match create_schema(&url).await {
            Ok(_) => println!("Created"),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Debug, FromRow)]
struct Note {
    id: i64,
    note: String,
    project: String,
}

#[tauri::command]
async fn getnote(id: &str) -> Result<Vec<String>, Vec<String>> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let notes: Vec<Note> = sqlx::query_as("SELECT * FROM notes WHERE id = ?")
        .bind(&id.to_string())
        .fetch_all(&instance)
        .await
        .unwrap();

    Ok(vec![
        notes.first().unwrap().note.clone(),
        notes.first().unwrap().project.clone(),
    ])
}

#[tauri::command]
async fn addnote(note: &str, project: &str) -> Result<String, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();
    let query = "
        INSERT INTO notes (
            note,
            project
        ) 
            VALUES($1, $2)
    ";
    let result = sqlx::query(&query)
        .bind(&note.to_string())
        .bind(&project.to_string())
        .execute(&instance)
        .await;

    instance.close().await;

    println!("Inserted -> {:?}", result);

    // let s: String = result.try_into().expect("Could not return string");
    Ok("Created".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            createdb, addnote, typetext, getnote
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
