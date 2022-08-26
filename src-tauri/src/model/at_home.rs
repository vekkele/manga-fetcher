pub use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AtHomeResponse {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: ChapterData,
}

#[derive(Debug, Deserialize)]
pub struct ChapterData {
    pub hash: String,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}
