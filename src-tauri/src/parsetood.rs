use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub created_at: Option<String>,
    pub title: String,
    pub note: String,
    pub folder: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskItem {
    pub checked: bool,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteWithTasks {
    pub note_id: i64,
    pub title: String,
    pub tasks: Vec<TaskItem>,
}

pub fn parse_tasks_from_html(html: &str) -> Result<Vec<TaskItem>, Box<dyn Error>> {
    let document = Html::parse_fragment(html);

    let li_selector = Selector::parse("li[data-type='taskItem']")?;
    let p_selector = Selector::parse("p")?;

    let tasks: Vec<TaskItem> = document
        .select(&li_selector)
        .map(|li_element| {
            let checked = li_element
                .value()
                .attr("data-checked")
                .and_then(|val| val.parse::<bool>().ok())
                .unwrap_or(false);

            let label = li_element
                .select(&p_selector)
                .next()
                .map(|p| p.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            TaskItem { checked, label }
        })
        .collect();

    Ok(tasks)
}

pub fn get_all_notes_with_tasks(db: &sled::Db) -> Result<Vec<NoteWithTasks>, Box<dyn Error>> {
    let mut notes_with_tasks = Vec::new();

    // Iterate through all notes in the database
    for result in db.iter() {
        let (_, value_bytes) = result?;
        let note: Note = bincode::deserialize(&value_bytes)?;

        // Parse tasks from the note's HTML content
        let tasks = parse_tasks_from_html(&note.note)?;

        // Only include notes that have tasks
        if !tasks.is_empty() {
            notes_with_tasks.push(NoteWithTasks {
                note_id: note.id,
                title: note.title,
                tasks,
            });
        }
    }

    Ok(notes_with_tasks)
}
