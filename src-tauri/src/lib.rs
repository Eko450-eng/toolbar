// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::env::home_dir;
use std::path::PathBuf;
use std::thread;

use serde::Deserialize;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Sqlite, SqlitePool};

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
