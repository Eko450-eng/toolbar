use regex::Regex;
use std::env;
use std::path::PathBuf;

use dirs_next;
use serde::Deserialize;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Sqlite, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Note {
    id: Option<i64>,
    created_at: Option<String>,
    title: String,
    note: String,
    folder: String,
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
        folder     TEXT
        );";
    let result = sqlx::query(qry).execute(&pool).await;
    pool.close().await;
    return result;
}

fn get_sqlite_url() -> PathBuf {
    if let Some(app_data_dir) = dirs_next::data_dir() {
        let mut path = app_data_dir;
        path.push("toolbar/notes.db");
        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
        path
    } else {
        println!("Could not determine the application data directory.");
        let mut path = PathBuf::from("/");
        path.push("toolbar/notes.db");
        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
        path
    }
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

#[tauri::command]
async fn createnote() -> Result<i64, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let _instance = SqlitePool::connect(&url).await.unwrap();

    let query = "
        INSERT INTO notes (
            note,
            folder,
            title
        ) 
            VALUES($1, $2, $3)
    ";

    let now = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S");

    let instance = SqlitePool::connect(&url).await.unwrap();
    let result = sqlx::query(&query)
        .bind("".to_string())
        .bind("".to_string())
        .bind(now.to_string())
        .execute(&instance)
        .await
        .map_err(|e| e.to_string())?
        .last_insert_rowid();

    Ok(result)
}

#[tauri::command]
async fn searchtext(project: Option<&str>) -> Result<Vec<Note>, Vec<String>> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let query = if let Some(project_string) = project {
        if !project_string.is_empty() {
            format!("SELECT * FROM notes WHERE note LIKE '%{project_string}%'")
        } else {
            "SELECT * FROM notes".to_string()
        }
    } else {
        "SELECT * FROM notes".to_string()
    };

    let notes: Vec<Note> = sqlx::query_as(&query).fetch_all(&instance).await.unwrap();
    println!("{:?}", notes);

    Ok(notes)
}

#[tauri::command]
async fn getnote(project: Option<&str>) -> Result<Vec<Note>, Vec<String>> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let query = if let Some(project_string) = project {
        if !project_string.is_empty() {
            format!("SELECT * FROM notes WHERE note LIKE '%project: {project_string}<%'")
        } else {
            "SELECT * FROM notes".to_string()
        }
    } else {
        "SELECT * FROM notes".to_string()
    };

    let notes: Vec<Note> = sqlx::query_as(&query).fetch_all(&instance).await.unwrap();

    Ok(notes)
}

fn subtractheader(input: &str) -> Option<String> {
    let re = Regex::new(
        r#".prop..tag data-type="property" class="..............">project: (?<project>\w*)"#,
    )
    .unwrap();
    re.captures(input)
        .and_then(|cap| cap.name("project").map(|m| m.as_str().to_string()))
}

#[tauri::command]
async fn getprojects() -> Result<Vec<String>, Vec<String>> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let instance = SqlitePool::connect(&url).await.unwrap();

    let notes: Vec<Note> = sqlx::query_as("SELECT * FROM notes")
        .fetch_all(&instance)
        .await
        .unwrap();

    let mut projects: Vec<String> = vec![];

    for note in notes {
        projects.push(subtractheader(&note.note).expect("Couldn't get project"));
    }

    Ok(projects)
}

#[tauri::command]
async fn deletenote(id: i64) -> Result<String, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let query = "DELETE FROM notes WHERE id = $1;";

    let instance = SqlitePool::connect(&url).await.unwrap();
    let _result = sqlx::query(&query).bind(&id).execute(&instance).await;

    instance.close().await;

    Ok("Created".to_string())
}

#[tauri::command]
async fn addnote(note: Note) -> Result<String, String> {
    let db_path = get_sqlite_url();
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let mut query = "
        INSERT INTO notes (
            note,
            folder,
            title
        ) 
            VALUES($1, $2, $3)
    ";

    if note.id != Some(0) {
        query = "
        UPDATE notes SET
            note = $1,
            folder = $2,
            title = $3
            WHERE id = $4;";
    }

    let instance = SqlitePool::connect(&url).await.unwrap();
    let _result = sqlx::query(&query)
        .bind(&note.note.to_string())
        .bind(&note.folder.to_string())
        .bind(&note.title.to_string())
        .bind(&note.id)
        .execute(&instance)
        .await;

    instance.close().await;

    Ok("Created".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            createdb,
            addnote,
            getnote,
            deletenote,
            createnote,
            getprojects,
            searchtext
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
