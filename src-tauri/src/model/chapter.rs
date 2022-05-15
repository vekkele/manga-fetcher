use serde::Serialize;

use super::FeedData;

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
