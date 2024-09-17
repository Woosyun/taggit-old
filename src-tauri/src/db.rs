use std::fs;

use rusqlite::{params_from_iter, Connection, Result};


#[allow(dead_code)]
fn connect_db(db_path: tauri::State<crate::DBPath>) -> Result<Connection, String> {
    if let Some(parent) = (&db_path.0).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    let conn: Connection = Connection::open(&db_path.0).map_err(|e| e.to_string())?;

    conn.execute("CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            last_modified TEXT NOT NULL
            )", ()).map_err(|e| e.to_string())?;
    conn.execute("CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            file_count INTEGER NOT NULL
            )", ()).map_err(|e| e.to_string())?;
    conn.execute("CREATE TABLE IF NOT EXISTS tag_note (
            tag_id INTEGER,
            note_id INTEGER,
            FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (tag_id, note_id)
            )", ()).map_err(|e| e.to_string())?;

    Ok(conn)
}

use crate::types;

#[tauri::command]
pub fn find_notes_by_tags(db_path: tauri::State<crate::DBPath>, tags: Vec<String>) -> Result<Vec<types::Note>, String> {
    let conn = connect_db(db_path)?;
    let placeholder = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("
        SELECT n.*
        FROM notes n
        JOIN tag_note tn ON n.id = tn.note_id
        JOIN tags t ON t.id = tn.tag_id
        WHERE t.name IN ({})
        GROUP BY n.id
        HAVING COUNT(DISTINCT t.id) >= ?", placeholder);

    let mut params = tags.iter().map(|tag| tag as &dyn rusqlite::ToSql).collect::<Vec<_>>();
    let len = tags.len().clone() as u32;
    params.push(&len);

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let notes = stmt.query_map(params_from_iter(params), |row| {
        Ok(types::Note::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>>>().map_err(|e| e.to_string())?;

    for note in &notes {
        println!("(find_notes_by_tags)Found note: {:?}", note);
    }

    Ok(notes)
}