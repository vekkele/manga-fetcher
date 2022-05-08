use thiserror::Error;

use crate::model::{ApiResponse, MangaData, MangaView, ResponseError};

const MANGADEX_ROOT: &str = "https://api.mangadex.org";

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

    let search_url = format!("{MANGADEX_ROOT}/manga?title={query}&includes[]=cover_art&limit=5");
    let res: ApiResponse<Vec<MangaData>> = reqwest::blocking::get(search_url)?.json()?;

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
