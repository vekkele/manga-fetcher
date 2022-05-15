use std::result;

use log::{debug, error};
use serde::Serialize;
use thiserror::Error;

use crate::model::Chapter;
use crate::model::{self, Manga};
use crate::service;

#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error("invalid arguments: {}", .0)]
    InvalidArguments(String),
    #[error("internal app error: {}", .0)]
    Internal(String),
}

impl From<model::ServiceError> for CommandError {
    fn from(e: model::ServiceError) -> Self {
        use model::ServiceError::*;

        match e {
            InvalidArguments(e) => Self::InvalidArguments(e),
            e => Self::Internal(e.to_string()),
        }
    }
}

pub type Result<T> = result::Result<T, CommandError>;

#[tauri::command]
pub fn search(query: &str) -> Result<Vec<model::MangaView>> {
    debug!("searching for \"{query}\"");

    Ok(service::search(query)?)
}

#[tauri::command]
pub fn get_manga(id: &str) -> Result<Manga> {
    Ok(service::get_manga(id)?)
}

#[tauri::command]
pub fn get_chapters(manga_id: &str, lang: &str, limit: u32, offset: u32) -> Result<Vec<Chapter>> {
    debug!("getting chapters: {manga_id}");
    Ok(service::fetch_feed(manga_id, lang, limit, offset)?)
}
