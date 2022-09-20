use std::result;

use log::{debug, error};
use serde::Serialize;
use thiserror::Error;

use crate::model::{ChapterProps, ChaptersResponse, Manga, MangaView, ServiceError};
use crate::service;

#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error("invalid arguments: {}", .0)]
    InvalidArguments(String),
    #[error("internal app error: {}", .0)]
    Internal(String),
}

impl From<ServiceError> for CommandError {
    fn from(e: ServiceError) -> Self {
        use ServiceError::*;

        match e {
            InvalidArguments(e) => Self::InvalidArguments(e),
            e => Self::Internal(e.to_string()),
        }
    }
}

pub type Result<T> = result::Result<T, CommandError>;

#[tauri::command]
pub async fn search(query: &str) -> Result<Vec<MangaView>> {
    debug!("searching for \"{query}\"");

    Ok(service::search(query).await?)
}

#[tauri::command]
pub async fn get_manga(id: &str) -> Result<Manga> {
    Ok(service::get_manga(id).await?)
}

#[tauri::command]
pub async fn get_chapters(
    manga_id: &str,
    lang: &str,
    limit: u32,
    offset: u32,
) -> Result<ChaptersResponse> {
    debug!("getting chapters: {manga_id}");
    Ok(service::fetch_feed(manga_id, lang, limit, offset).await?)
}

#[tauri::command]
pub async fn download(chapters: Vec<ChapterProps>) -> Result<()> {
    service::download(chapters).await?;
    Ok(())
}
