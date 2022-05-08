use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

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
            let mut title_iter = title_map.iter();

            match title_iter.next() {
                Some((_, value)) => value.to_owned(),
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

const MANGADEX_ROOT: &str = "https://api.mangadex.org";

#[derive(Debug, Deserialize)]
struct SearchResponse {
    result: String,
    data: Option<Vec<MangaData>>,
    errors: Option<Vec<ResponseError>>,
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

#[derive(Debug, Deserialize)]
struct MangaData {
    id: String,
    attributes: Attributes,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    status: String,
    title: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("provided query is empty")]
    EmptyQuery,

    #[error("failed to request: {}", .0)]
    HttpError(#[from] reqwest::Error),

    #[error("failed to request: {:#?}", .0)]
    ApiErrors(Vec<ResponseError>),
}

pub fn search(query: &str) -> Result<Vec<MangaView>, SearchError> {
    if query.is_empty() {
        return Err(SearchError::EmptyQuery);
    }

    let search_url = format!("{MANGADEX_ROOT}/manga?title={query}&limit=5");
    let res: SearchResponse = reqwest::blocking::get(search_url)?.json()?;

    let result = match res.result.as_str() {
        "ok" => res.data.unwrap(),
        _ => {
            return Err(SearchError::ApiErrors(
                res.errors.unwrap_or_else(|| vec![ResponseError::default()]),
            ))
        }
    };

    Ok(result.into_iter().map(MangaView::from).collect())
}
