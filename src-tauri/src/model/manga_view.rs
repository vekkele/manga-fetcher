use serde::Serialize;

use super::MangaData;
use crate::constants::MANGADEX_UPLOADS;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaView {
    id: String,
    title: String,
    status: String,
    cover_url: Option<String>,
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

        MangaView {
            id: manga.id.to_owned(),
            title,
            status: manga.attributes.status.to_owned(),
            cover_url,
        }
    }
}
