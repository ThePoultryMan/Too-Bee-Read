use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::path::Path;

use crate::api;

#[derive(Deserialize, Serialize)]
pub struct SaveData {
    books: Option<Vec<BookData>>,
}

#[derive(Deserialize, Serialize)]
struct BookData {
    status: BookStatus,
    title: String,
    volume_info: api::VolumeInfo,
}

#[derive(Deserialize, Serialize)]
enum BookStatus {
    Unread,
    InProgress,
    Read,
}

impl SaveData {
    pub fn write_save(&self) {
        if !Path::new("./data").is_dir() {
            create_dir("./data").expect("Couldn't create data directory.");
        }

        let save_data = File::create("./data/save_data.json").expect("Couldn't create save file.");

        serde_json::to_writer(save_data, self).expect("Couldn't write save data to file.");
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self { books: None }
    }
}
