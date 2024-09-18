use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Note {
    id: i64,
    name: String,
    path: String,
    last_modified: u64
}
impl Note {
    pub fn new(id: i64, name: String, path: String, last_modified: u64) -> Note {
        Note {
            id,
            name,
            path,
            last_modified
        }
    }
}

#[allow(dead_code)]
pub struct Tag {
    id: String,
    name: String
}

#[allow(dead_code)]
pub struct TagNote {
    item_id: String,
    tag_id: String
}