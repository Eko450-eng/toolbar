use std::error::Error;
use std::path::PathBuf;

mod parsetood;

use dirs_next;
use parsetood::get_all_notes_with_tasks;
use parsetood::Note;
use parsetood::NoteWithTasks;
use serde::Deserialize;
use serde::Serialize;

fn create_empty_note(db: &sled::Db, folder: String) -> Result<i64, Box<dyn Error>> {
    // Get the next available ID
    let next_id = db.len() as i64;

    let note = Note {
        id: next_id,
        created_at: Some(chrono::Utc::now().to_rfc3339()),
        title: String::new(),
        note: String::new(),
        folder,
    };

    // Insert the empty note
    db.insert(next_id.to_be_bytes(), bincode::serialize(&note)?)?;

    Ok(next_id)
}

// Function to edit a note
fn update_note(
    db: &sled::Db,
    id: i64,
    title: String,
    content: String,
    folder: String,
) -> Result<(), Box<dyn Error>> {
    // Get the existing note
    if let Some(existing_note_bytes) = db.get(id.to_be_bytes())? {
        let mut note: Note = bincode::deserialize(&existing_note_bytes)?;

        // Update the fields
        note.title = title;
        note.note = content;
        note.folder = folder;

        // Save the updated note
        db.insert(id.to_be_bytes(), bincode::serialize(&note)?)?;

        Ok(())
    } else {
        Err("Note not found".into())
    }
}

// Function to read all notes
fn read_all_notes(db: &sled::Db) -> Result<Vec<Note>, Box<dyn Error>> {
    let notes: Result<Vec<Note>, Box<dyn Error>> = db
        .iter()
        .map(|result| {
            let (_, value_bytes) = result?;
            let note: Note = bincode::deserialize(&value_bytes)?;
            Ok(note)
        })
        .collect();

    notes
}

// New function to search notes by title
fn search_notes_by_title(db: &sled::Db, search_term: &str) -> Result<Vec<Note>, Box<dyn Error>> {
    let matching_notes: Vec<Note> = db
        .iter()
        .filter_map(|result| {
            result.ok().and_then(|(_, value_bytes)| {
                let note: Note = bincode::deserialize(&value_bytes).ok()?;
                // Case-insensitive search
                if note
                    .title
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
                {
                    Some(note)
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(matching_notes)
}

fn getopentasks(db: &sled::Db) -> Result<Vec<NoteWithTasks>, Box<dyn std::error::Error>> {
    let notes_with_tasks = get_all_notes_with_tasks(&db)?;

    // Print the results
    for note in &notes_with_tasks {
        println!("\nNote: {} - '{}'", note.note_id, note.title);
        for (index, task) in note.tasks.iter().enumerate() {
            println!(
                "  Task {}: [{}] {}",
                index + 1,
                if task.checked { "x" } else { " " },
                task.label
            );
        }
    }

    Ok(notes_with_tasks)
}

#[tauri::command]
async fn gettask() -> Result<Vec<NoteWithTasks>, Vec<String>> {
    let url = get_sled_url();
    let db = sled::open(&url).expect("Cant find file in 35");

    let tasks = getopentasks(&db);

    db.flush();

    Ok(tasks.expect("No tasks"))
}

// New function to delete a note
fn delete_note(db: &sled::Db, id: i64) -> Result<(), Box<dyn Error>> {
    if db.remove(id.to_be_bytes())?.is_some() {
        Ok(())
    } else {
        Err("Note not found".into())
    }
}

fn get_sled_url() -> PathBuf {
    if let Some(app_data_dir) = dirs_next::data_dir() {
        let mut path = app_data_dir;
        path.push("toolbar/notes");
        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
        path
    } else {
        println!("Could not determine the application data directory.");
        let mut path = PathBuf::from("/");
        path.push("toolbar/notes");
        std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
        path
    }
}

#[tauri::command]
async fn getnote() -> Result<Vec<Note>, Vec<String>> {
    let url = get_sled_url();
    let db = sled::open(&url).expect("Cant find file in 35");

    let notes = read_all_notes(&db);

    println!("GetNote: ---- {:?}", notes);

    db.flush();
    Ok(notes.expect("Empty"))
}

#[tauri::command]
async fn searchnotebytitle(query: &str) -> Result<Vec<Note>, Vec<String>> {
    let url = get_sled_url();
    let db = sled::open(&url).expect("Cant find file in 35");

    let notes = search_notes_by_title(&db, query);

    println!("GetNote: ---- {:?}", notes);

    db.flush();
    Ok(notes.expect("Empty"))
}

#[tauri::command]
async fn createnote() -> Result<String, String> {
    let url = get_sled_url();
    let db = sled::open(&url).expect("Cant find file in 56");
    create_empty_note(&db, "none".to_string());

    db.flush();
    Ok("Created".to_string())
}

#[tauri::command]
async fn addnote(note: Note) -> Result<String, String> {
    let url = get_sled_url();

    let db = sled::open(&url).expect("Cant find file in 56");
    update_note(&db, note.id, note.title, note.note, note.folder);

    db.flush();
    Ok("Created".to_string())
}

#[tauri::command]
async fn deletenote(id: i64) -> Result<String, String> {
    let url = get_sled_url();
    let db = sled::open(&url).expect("Cant find file in 56");

    delete_note(&db, id);

    db.flush();
    Ok("Created".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            getnote,
            addnote,
            createnote,
            deletenote,
            searchnotebytitle,
            gettask
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
