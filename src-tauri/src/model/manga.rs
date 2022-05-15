use serde::Serialize;

use super::{MangaData, MangaStatistics, MangaView};

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
