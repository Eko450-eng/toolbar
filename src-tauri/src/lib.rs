// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::env::home_dir;
use std::path::PathBuf;
use std::thread;

use serde::Deserialize;
use serde::Serialize;
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
        title       TEXT        NOT NULL,
        note        TEXT,
        created_at   DATETIME    DEFAULT     CURRENT_TIMESTAMP,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Note {
    id: Option<i64>,
    created_at: Option<String>,
    title: String,
    note: String,
    project: String,
}

#[tauri::command]
async fn createnote() -> Result<i64, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let query = "
        INSERT INTO notes (
            note,
            project,
            title
        ) 
            VALUES($1, $2, $3)
    ";

    let instance = SqlitePool::connect(&url).await.unwrap();
    let result = sqlx::query(&query)
        .bind("".to_string())
        .bind("".to_string())
        .bind("New Note".to_string())
        .execute(&instance)
        .await
        .map_err(|e| e.to_string())?
        .last_insert_rowid();

    Ok(result)
}

#[tauri::command]
async fn getnote() -> Result<Vec<Note>, Vec<String>> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let notes: Vec<Note> = sqlx::query_as("SELECT * FROM notes")
        .fetch_all(&instance)
        .await
        .unwrap();

    //    println!("{:?}", notes.first().clone().unwrap());

    Ok(notes)
}

#[tauri::command]
async fn deletenote(id: i64) -> Result<String, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let query = "DELETE FROM notes WHERE id = $1;";

    let instance = SqlitePool::connect(&url).await.unwrap();
    let result = sqlx::query(&query).bind(&id).execute(&instance).await;

    instance.close().await;

    println!("Inserted -> {:?}", result);

    // let s: String = result.try_into().expect("Could not return string");
    Ok("Created".to_string())
}

#[tauri::command]
async fn addnote(note: Note) -> Result<String, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let mut query = "
        INSERT INTO notes (
            note,
            project,
            title
        ) 
            VALUES($1, $2, $3)
    ";

    if note.id != Some(0) {
        query = "
        UPDATE notes SET
            note = $1,
            project = $2,
            title = $3
            WHERE id = $4;";
    }

    let instance = SqlitePool::connect(&url).await.unwrap();
    let result = sqlx::query(&query)
        .bind(&note.note.to_string())
        .bind(&note.project.to_string())
        .bind(&note.title.to_string())
        .bind(&note.id)
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
            createdb, addnote, typetext, getnote, deletenote, createnote
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
