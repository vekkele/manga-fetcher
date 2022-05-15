use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MangaData {
    pub id: String,
    pub attributes: MangaAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MangaAttributes {
    pub status: String,
    pub title: HashMap<String, String>,
    pub description: HashMap<String, String>,
    #[serde(rename = "availableTranslatedLanguages")]
    pub translations: Vec<String>,
    pub year: Option<u32>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    pub attributes: TagAttributes,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TagAttributes {
    pub group: String,
    pub name: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Relationship {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub attributes: Option<RelationshipAttributes>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipAttributes {
    pub file_name: Option<String>,
    pub name: Option<String>,
}
