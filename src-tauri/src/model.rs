use std::{collections::HashMap, result};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::constants::MANGADEX_UPLOADS;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaView {
    id: String,
    title: String,
    status: String,
    cover_url: Option<String>,
    lang_codes: Vec<String>,
}

impl From<&MangaData> for MangaView {
    fn from(manga: &MangaData) -> Self {
        let mut title_map = manga.attributes.title.clone();

        let title = title_map.remove("en").unwrap_or_else(|| {
            let mut title_iter = title_map.into_iter();

            match title_iter.next() {
                Some((_, value)) => value,
                None => "".to_string(),
            }
        });

        let cover_url: Option<String> = manga
            .relationships
            .iter()
            .find(|rel| rel.rel_type == "cover_art")
            .and_then(|rel| {
                let file_name = rel.attributes.as_ref()?.file_name.as_ref()?;
                let url = format!(
                    "{MANGADEX_UPLOADS}/covers/{}/{}.512.jpg",
                    manga.id, file_name,
                );

                Some(url)
            });

        let lang_codes = manga
            .attributes
            .translations
            .iter()
            .map(|t| t.split('-').next().unwrap().to_string())
            .map(|code| match code.as_str() {
                "en" => "gb".to_string(),
                "zh" => "cn".to_string(),
                c => c.to_string(),
            })
            .collect();

        MangaView {
            id: manga.id.to_owned(),
            title,
            status: manga.attributes.status.to_owned(),
            cover_url,
            lang_codes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    id: String,
    attributes: MangaAttributes,
    relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize, Clone)]
struct MangaAttributes {
    status: String,
    title: HashMap<String, String>,
    description: HashMap<String, String>,
    #[serde(rename = "availableTranslatedLanguages")]
    translations: Vec<String>,
    year: Option<u32>,
    tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Clone)]
struct Tag {
    attributes: TagAttributes,
}

#[derive(Debug, Deserialize, Clone)]
struct TagAttributes {
    group: String,
    name: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Relationship {
    #[serde(rename = "type")]
    rel_type: String,
    attributes: Option<RelationshipAttributes>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RelationshipAttributes {
    file_name: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub result: String,
    pub data: Option<T>,
    pub errors: Option<Vec<ResponseError>>,
}

impl<T> ApiResponse<T> {
    pub fn result(self, tag: &str) -> Result<T> {
        match self.result.as_str() {
            "ok" => Ok(self.data.unwrap()),
            _ => Err(ServiceError::ApiError {
                errors: self
                    .errors
                    .unwrap_or_else(|| vec![ResponseError::default()]),
                tag: tag.to_owned(),
            }),
        }
    }
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    view: MangaView,
    description: Option<String>,
    genres: Vec<String>,
    year: Option<u32>,
    avg_score: Option<f32>,
    author: Option<String>,
}

impl Manga {
    pub fn new(data: &MangaData, stats: &MangaStatistics) -> Self {
        let view = MangaView::from(data);

        let description_map = &data.attributes.description;
        let description = description_map
            .get("en")
            .or_else(|| {
                let mut description_iter = description_map.iter();
                description_iter.next().map(|(_, v)| v)
            })
            .cloned();

        let genres: Vec<String> = data
            .attributes
            .tags
            .iter()
            .filter_map(|t| match t.attributes.group.as_str() {
                "genre" => Some(t.attributes.name.get("en")?.to_owned()),
                _ => None,
            })
            .collect();

        let author = data
            .relationships
            .iter()
            .find_map(|rel| match rel.rel_type.as_str() {
                "author" => rel.attributes.as_ref()?.name.to_owned(),
                _ => None,
            });

        Manga {
            view,
            description,
            year: data.attributes.year,
            genres,
            author,
            avg_score: stats.rating.average,
        }
    }
}

#[derive(Debug, Deserialize)]

pub struct FeedData {
    id: String,
    pub attributes: ChapterAttributes,
    relationships: Vec<ChapterRelationship>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAttributes {
    volume: Option<String>,
    chapter: String,
    title: Option<String>,
    pages: u32,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterRelationship {
    id: String,
    #[serde(rename = "type")]
    rel_type: String,
    attributes: Option<ChapterRelationshipAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterRelationshipAttributes {
    name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    id: String,
    chapter: String,
    volume: Option<String>,
    title: Option<String>,
    scan_group: Option<ScanGroup>,
    pages: u32,
}

impl From<&FeedData> for Chapter {
    fn from(data: &FeedData) -> Self {
        let scan_group = data
            .relationships
            .iter()
            .find_map(|rel| match rel.rel_type.as_str() {
                "scanlation_group" => Some(ScanGroup {
                    id: rel.id.to_owned(),
                    name: rel.attributes.as_ref()?.name.as_ref()?.to_owned(),
                }),
                _ => None,
            });

        Chapter {
            id: data.id.to_owned(),
            chapter: data.attributes.chapter.to_owned(),
            volume: data.attributes.volume.to_owned(),
            title: data.attributes.title.to_owned(),
            pages: data.attributes.pages,
            scan_group,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScanGroup {
    name: String,
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsResponse {
    pub statistics: HashMap<String, MangaStatistics>,
}

#[derive(Debug, Deserialize)]
pub struct MangaStatistics {
    rating: Rating,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    average: Option<f32>,
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid arguments: {}", .0)]
    InvalidArguments(String),

    #[error("failed call to api \"{}\", errors: {:#?}", .tag, .errors)]
    ApiError {
        errors: Vec<ResponseError>,
        tag: String,
    },

    #[error("failed http requests")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = result::Result<T, ServiceError>;
