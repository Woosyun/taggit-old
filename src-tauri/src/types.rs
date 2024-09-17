use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Note {
    id: String,
    name: String,
    path: String,
    last_modified: String
}
impl Note {
    pub fn new(id: String, name: String, path: String, last_modified: String) -> Note {
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