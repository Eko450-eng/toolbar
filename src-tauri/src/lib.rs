mod parsetood;
mod sledfunctions;

use parsetood::{get_all_notes_with_tasks, Note, NoteWithTasks};
use sledfunctions::{
    create_empty_note, delete_note, getdb, read_all_notes, search_notes_by_title, update_note, 
};

// Function to edit a note
#[tauri::command]
async fn gettask() -> Result<Vec<NoteWithTasks>, Vec<String>> {
    let db = getdb();
    let tasks = get_all_notes_with_tasks(&db);
    let _ = db.flush();

    Ok(tasks.expect("No tasks"))
}

#[tauri::command]
async fn getnote() -> Result<Vec<Note>, Vec<String>> {
    let db = getdb();

    let notes = read_all_notes(&db);

    let _ = db.flush();
    Ok(notes.expect("Empty"))
}

#[tauri::command]
async fn searchnotebytitle(query: &str) -> Result<Vec<Note>, Vec<String>> {
    let db = getdb();

    let notes = search_notes_by_title(&db, query);

    let _ = db.flush();
    Ok(notes.expect("Empty"))
}

#[tauri::command]
async fn createnote() -> Result<String, String> {
    let db = getdb();
    let _ = create_empty_note(&db, "none".to_string());

    let _ = db.flush();
    Ok("Created".to_string())
}

#[tauri::command]
async fn addnote(note: Note) -> Result<String, String> {
    let db = getdb();
    let _ = update_note(&db, note.id, note.title, note.note, note.folder);

    let _ = db.flush();
    Ok("Created".to_string())
}

#[tauri::command]
async fn deletenote(id: i64) -> Result<String, String> {
    let db = getdb();
    let _ = delete_note(&db, id);

    let _ = db.flush();
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
