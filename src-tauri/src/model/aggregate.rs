use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AggregateResponse {
    pub volumes: HashMap<String, VolumeAggregate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeAggregate {
    pub volume: String,
    pub count: u32,
    pub chapters: HashMap<String, ChapterAggregate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterAggregate {
    pub id: String,
    pub chapter: String,
}
