use serde::{Deserialize, Serialize};

use crate::model::ResponseError;

use super::{ApiResponse, FeedData, ServiceError};

#[derive(Deserialize)]
pub struct ChapterProps {
    pub id: String,
    pub fullname: String,
}

#[derive(Debug, Serialize)]
pub struct ChaptersResponse {
    chapters: Vec<Chapter>,
    limit: u32,
    offset: u32,
    total: u32,
}

impl TryFrom<ApiResponse<Vec<FeedData>>> for ChaptersResponse {
    type Error = ServiceError;

    fn try_from(value: ApiResponse<Vec<FeedData>>) -> Result<Self, Self::Error> {
        const TAG: &str = "feed";

        let create_errors = |name: &str| ServiceError::ApiError {
            errors: vec![ResponseError::new(
                500,
                "not enough data fetcher",
                format!("{name} was not provided by API").as_str(),
            )],
            tag: TAG.to_owned(),
        };

        let limit = value.limit.ok_or_else(|| create_errors("limit"))?;
        let offset = value.offset.ok_or_else(|| create_errors("offset"))?;
        let total = value.total.ok_or_else(|| create_errors("total"))?;
        let feed_data = value.result(TAG)?;

        Ok(ChaptersResponse {
            chapters: feed_data.iter().map(Chapter::from).collect(),
            limit,
            offset,
            total,
        })
    }
}

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
    fn from(data: &FeedData) -> Chapter {
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
