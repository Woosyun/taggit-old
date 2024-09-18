use std::{fs, path::Path, time::UNIX_EPOCH};
use rusqlite::{params, params_from_iter, Connection, Result};

fn connect_db(db_path: &tauri::State<crate::DBPath>) -> Result<Connection, String> {
    // println!("(connect_db) db_path: {:?}", &db_path.0);
    
    if let Some(parent) = db_path.0.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    let conn: Connection = Connection::open(&db_path.0).map_err(|e| e.to_string())?;

    conn.execute("CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            last_modified INTEGER NOT NULL
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
    println!("(find_notes_by_tags) received tags: {:?}", &tags);
    
    let conn = connect_db(&db_path)?;
    let placeholder = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");

    println!("(find_notes_by_tags) placeholder: {:?}", &placeholder);
    
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
        println!("(find_notes_by_tags) row: {:?}", &row);
        Ok(types::Note::new(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>>>().map_err(|e| e.to_string())?;

    println!("(find_notes_by_tags) found notes: {:?}", &notes);

    Ok(notes)
}

#[tauri::command]
pub fn insert_notes_with_tags(db_path: tauri::State<crate::DBPath>, files: Vec<String>, tags: Vec<String>) {
    println!("(insert_notes_with_tags) file path array: {:?}", &files);
    println!("(insert_notes_with_tags) tag array: {:?}", &tags);

    let conn = connect_db(&db_path).unwrap();
    
    //1. copy files to objects/year/month
    let dest_dir = {
        use chrono::Utc;
        let now = Utc::now();
        let year = now.format("%Y").to_string();
        let month = now.format("%m").to_string();
        let dest_dir = db_path.0.parent().unwrap().join("objects").join(year).join(month);
        fs::create_dir_all(&dest_dir).expect("Failed to create directory");

        dest_dir
    };

    println!("(insert_notes_with_tags) dest_dir: {:?}", &dest_dir);
    
    let copied_files = files.iter().map(|file| {
        //get file name?
        let file_name = Path::new(file).file_name().unwrap();
        let dest_path = dest_dir.join(file_name);
        fs::copy(file, &dest_path).expect("Failed to copy file");
        dest_path
    });

    println!("(insert_notes_with_tags) copied_files: {:?}", &copied_files);

    //2. get tag ids from db. If not exist, insert tag to db
    let tag_ids: Vec<u32> = tags.iter().map(|tag| {
        let mut stmt = conn.prepare("SELECT id FROM tags WHERE name = ?").unwrap();
        let id = stmt.query_row(params![tag], |row| row.get(0)).ok();
        if id.is_none() {
            conn.execute("INSERT INTO tags (name, file_count) VALUES (?, 1)", params![tag]).unwrap();
            let id = conn.last_insert_rowid() as u32;
            println!("(insert_notes_with_tags) inserted tag: {} with id: {}", tag, id);
            id
        } else {
            id.unwrap()
        }
    }).collect();

    println!("(insert_notes_with_tags) tag_ids: {:?}", &tag_ids);
    
    //3. insert notes to db and get note ids
    let notes_ids = copied_files.map(|file_path| {
        let file_name = file_path.file_name()
            .unwrap()
            .to_str();

        let last_modified = fs::metadata(&file_path)
            .and_then(|m| m.modified())
            .and_then(|m| Ok(m.duration_since(UNIX_EPOCH)))
            .unwrap()
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let mut stmt = conn.prepare("INSERT INTO notes (name, path, last_modified) VALUES (?, ?, ?)").unwrap();
        stmt.execute(params![&file_name, file_path.to_str(), last_modified]).unwrap();
        conn.last_insert_rowid() as u32
    });

    println!("(insert_notes_with_tags) notes_ids: {:?}", &notes_ids);
    
    //4. insert tag_note to db
    for note_id in notes_ids {
        for tag_id in &tag_ids {
            conn.execute("INSERT INTO tag_note (tag_id, note_id) VALUES (?, ?)", params![tag_id, note_id]).unwrap();
        }
    }
}