use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct FeedData {
    pub id: String,
    pub attributes: ChapterAttributes,
    pub relationships: Vec<ChapterRelationship>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAttributes {
    pub volume: Option<String>,
    pub chapter: String,
    pub title: Option<String>,
    pub pages: u32,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterRelationship {
    pub id: String,
    #[serde(rename = "type")]
    pub rel_type: String,
    pub attributes: Option<ChapterRelationshipAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterRelationshipAttributes {
    pub name: Option<String>,
}
