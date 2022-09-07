use std::vec;

use reqwest::Error;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Volume {
    kind: String,
    totalItems: i32,
    items: Vec<Item>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct Item {
    kind: String,
    id: String,
    etag: String,
    volumeInfo: VolumeInfo,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    publisher: Option<String>,
    publishedDate: String,
    description: Option<String>,
    readingModes: ReadingModes,
    pageCount: Option<i32>,
    printType: String,
    averageRating: Option<f32>,
    ratingsCount: Option<i32>,
    maturityRating: String,
    contentVersion: String,
    imageLinks: ImageLinks,
    language: String,
    infoLink: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct ReadingModes {
    text: bool,
    image: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct ImageLinks {
    smallThumbnail: String,
    thumbnail: String,
}

impl Volume {
    fn read_error(error: serde_json::Error) -> Self {
        Self {
            kind: "books#volumes".to_owned(),
            totalItems: 1,
            items: vec![Item::read_error(error)],
        }
    }
}

impl Item {
    fn read_error(error: serde_json::Error) -> Self {
        Self {
            kind: "bookss#volume".to_owned(),
            id: "thisisntabook".to_owned(),
            etag: "thisstillisntabook".to_owned(),
            volumeInfo: VolumeInfo::read_error(error),
        }
    }
}

impl VolumeInfo {
    fn read_error(error: serde_json::Error) -> Self {
        Self {
            title: "There was an error :(".to_owned(),
            authors: vec!["ThePoultryMan".to_owned()],
            publisher: Some("none".to_owned()),
            publishedDate: "none".to_owned(),
            description: Some(error.to_string()),
            readingModes: ReadingModes::read_error(),
            pageCount: Some(0),
            printType: "NONE".to_owned(),
            averageRating: Some(5.0),
            ratingsCount: Some(1),
            maturityRating: "NOT_MATURE".to_owned(),
            contentVersion: "1.0.0".to_owned(),
            imageLinks: ImageLinks::default(),
            language: "en".to_owned(),
            infoLink: "https://github.com/ThePoultryMan/Too-Bee-Read".to_owned(),
        }
    }
}

impl ReadingModes {
    fn read_error() -> Self {
        Self {
            text: false,
            image: false,
        }
    }
}

impl Default for ImageLinks {
    fn default() -> Self {
        Self {
            smallThumbnail: String::new(),
            thumbnail: String::new(),
        }
    }
}

pub async fn search_for_book(name: &str) -> Result<(), Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + name)
        .await?
        .text()
        .await?;

    let response = match serde_json::from_str::<Volume>(&body) {
        Ok(volume) => volume,
        Err(err) => Volume::read_error(err),
    };

    println!("{}", response.items[0].volumeInfo.title);

    Ok(())
}
