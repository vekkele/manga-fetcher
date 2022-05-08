use log::{debug, error};
use serde::Serialize;
use thiserror::Error;

use crate::model;
use crate::service;

#[derive(Debug, Error, Serialize)]
pub enum SearchError {
    #[error("provided query is empty")]
    EmptyQuery,
    #[error("internal app error")]
    Internal,
}

impl From<service::SearchError> for SearchError {
    fn from(e: service::SearchError) -> Self {
        use service::SearchError::*;

        match e {
            EmptyQuery => Self::EmptyQuery,
            _ => Self::Internal,
        }
    }
}

#[tauri::command]
pub fn search(query: &str) -> Result<Vec<model::MangaView>, SearchError> {
    debug!("searching for \"{query}\"");

    match service::search(query) {
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
