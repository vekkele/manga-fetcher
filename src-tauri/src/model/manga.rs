use serde::Serialize;

use super::{MangaData, MangaStatistics, MangaView};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    view: MangaView,
    year: Option<u32>,
    avg_score: Option<f32>,
    author: Option<String>,
}

impl Manga {
    pub fn new(data: &MangaData, stats: &MangaStatistics) -> Self {
        let view = MangaView::from(data);

        let author = data
            .relationships
            .iter()
            .find_map(|rel| match rel.rel_type.as_str() {
                "author" => rel.attributes.as_ref()?.name.to_owned(),
                _ => None,
            });

        Manga {
            view,
            year: data.attributes.year,
            author,
            avg_score: stats.rating.average,
        }
    }
}
