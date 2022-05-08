use log::{debug, error};
use serde::Serialize;
use thiserror::Error;

use crate::service::search;

#[derive(Debug, Error, Serialize)]
pub enum SearchError {
    #[error("provided query is empty")]
    EmptyQuery,
    #[error("internal app error")]
    Internal,
}

impl From<search::SearchError> for SearchError {
    fn from(e: search::SearchError) -> Self {
        use search::SearchError::*;

        match e {
            EmptyQuery => Self::EmptyQuery,
            _ => Self::Internal,
        }
    }
}

#[tauri::command]
pub fn search(query: &str) -> Result<Vec<search::MangaView>, SearchError> {
    debug!("searching for \"{query}\"");

    match search::search(query) {
        Ok(res) => {
            debug!("success. Found {} entries", res.len());
            Ok(res)
        }
        Err(e) => {
            error!("failed to search: {e}");
            Err(SearchError::from(e))
        }
    }
}
