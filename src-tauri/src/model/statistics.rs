use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatisticsResponse {
    pub statistics: HashMap<String, MangaStatistics>,
}

#[derive(Debug, Deserialize)]
pub struct MangaStatistics {
    pub rating: Rating,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub average: Option<f32>,
}
