use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MangaView {
    id: String,
    title: String,
    status: String,
}

impl From<MangaData> for MangaView {
    fn from(manga: MangaData) -> Self {
        let mut title_map = manga.attributes.title;

        let title = title_map.remove("en").unwrap_or_else(|| {
            let mut title_iter = title_map.into_iter();

            match title_iter.next() {
                Some((_, value)) => value,
                None => "".to_string(),
            }
        });

        MangaView {
            id: manga.id,
            title,
            status: manga.attributes.status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    id: String,
    attributes: Attributes,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    status: String,
    title: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub result: String,
    pub data: Option<T>,
    pub errors: Option<Vec<ResponseError>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    id: String,
    status: u32,
    title: String,
    detail: String,
}

impl Default for ResponseError {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: 500,
            title: "unknown error".to_owned(),
            detail: "something went wrong".to_owned(),
        }
    }
}
