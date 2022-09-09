use serde::{Deserialize, Serialize};
use std::fs::{create_dir, read_to_string, File};
use std::path::Path;

use crate::api;

#[derive(Deserialize, Serialize)]
pub struct SaveData {
    books: Option<Vec<BookData>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BookData {
    status: BookStatus,
    title: String,
    volume_info: api::VolumeInfo,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum BookStatus {
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

    pub fn add_book_data(&mut self, book_data: BookData) {
        let mut new_book_data = self.books.clone().unwrap_or(Vec::new());
        new_book_data.push(book_data);
        self.books = Some(new_book_data);
    }

    pub fn has_books(&self) -> bool {
        self.books.is_some()
    }

    pub fn get_books(&self) -> Vec<BookData> {
        self.books.clone().expect("A severe error has occured.")
    }
}

impl BookData {
    pub fn new(status: BookStatus, title: String, volume_info: api::VolumeInfo) -> Self {
        Self {
            status,
            title,
            volume_info,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_volume_info(&self) -> api::VolumeInfo {
        self.volume_info.clone()
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self { books: None }
    }
}

pub fn get_save() -> SaveData {
    let save_data = read_to_string("./data/save_data.json").expect("Couldn't retrieve save data.");

    serde_json::from_str::<SaveData>(&save_data).expect("Couldn't use save data.")
}
