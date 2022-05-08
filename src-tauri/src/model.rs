use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::constants::MANGADEX_UPLOADS;

#[derive(Debug, Serialize)]
pub struct MangaView {
    id: String,
    title: String,
    status: String,
    cover_url: Option<String>,
    lang_codes: Vec<String>,
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

        let cover_url: Option<String> = manga
            .relationships
            .into_iter()
            .find(|rel| rel.rel_type == "cover_art")
            .and_then(|rel: Relationship| {
                let file_name = rel.attributes?.file_name?;
                let url = format!(
                    "{MANGADEX_UPLOADS}/covers/{}/{}.512.jpg",
                    manga.id, file_name,
                );

                Some(url)
            });

        let lang_codes = manga
            .attributes
            .translations
            .into_iter()
            .map(|t| t.split('-').next().unwrap().to_string())
            .map(|code| match code.as_str() {
                "en" => "gb".to_string(),
                "zh" => "cn".to_string(),
                c => c.to_string(),
            })
            .collect();

        MangaView {
            id: manga.id,
            title,
            status: manga.attributes.status,
            cover_url,
            lang_codes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    id: String,
    attributes: Attributes,
    relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    status: String,
    title: HashMap<String, String>,
    #[serde(rename = "availableTranslatedLanguages")]
    translations: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Relationship {
    #[serde(rename = "type")]
    rel_type: String,
    attributes: Option<RelationshipAttributes>,
}

#[derive(Debug, Deserialize)]
struct RelationshipAttributes {
    #[serde(rename = "fileName")]
    file_name: Option<String>,
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
