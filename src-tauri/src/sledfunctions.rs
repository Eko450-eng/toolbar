use std::{error::Error, path::PathBuf};

use crate::parsetood::Note;

pub fn delete_note(db: &sled::Db, id: i64) -> Result<(), Box<dyn Error>> {
    if db.remove(id.to_be_bytes())?.is_some() {
        Ok(())
    } else {
        Err("Note not found".into())
    }
}

fn get_sled_url() -> std::path::PathBuf {
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

pub fn getdb() -> sled::Db {
    let url = get_sled_url();
    sled::open(&url).expect("Cant find file in 35")
}

pub fn create_empty_note(db: &sled::Db, folder: String) -> Result<i64, Box<dyn Error>> {
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

pub fn update_note(
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
pub fn read_all_notes(db: &sled::Db) -> Result<Vec<Note>, Box<dyn Error>> {
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
pub fn search_notes_by_title(
    db: &sled::Db,
    search_term: &str,
) -> Result<Vec<Note>, Box<dyn Error>> {
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
