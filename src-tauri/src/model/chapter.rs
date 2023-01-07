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
    external_url: Option<String>,
}

impl From<&FeedData> for Chapter {
    fn from(data: &FeedData) -> Chapter {
        let scan_group = ScanGroup::try_from(data).ok();

        Chapter {
            id: data.id.to_owned(),
            chapter: data.attributes.chapter.to_owned(),
            volume: data.attributes.volume.to_owned(),
            title: data.attributes.title.to_owned(),
            pages: data.attributes.pages,
            external_url: data.attributes.external_url.to_owned(),
            scan_group,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScanGroup {
    name: String,
    id: String,
    website: Option<String>,
    description: Option<String>,
}

impl TryFrom<&FeedData> for ScanGroup {
    type Error = ();

    fn try_from(data: &FeedData) -> Result<Self, Self::Error> {
        let scan_group = data
            .relationships
            .iter()
            .find_map(|rel| match rel.rel_type.as_str() {
                "scanlation_group" => Some(ScanGroup {
                    id: rel.id.to_owned(),
                    name: rel.attributes.as_ref()?.name.clone()?,
                    website: rel.attributes.as_ref()?.website.clone(),
                    description: rel.attributes.as_ref()?.description.clone(),
                }),
                _ => None,
            });

        scan_group.ok_or(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::{ChapterAttributes, ChapterRelationship, ChapterRelationshipAttributes};

    #[test]
    fn empty_scan_group() {
        let name = "scan_group_name";

        let feed_data = create_scan_group_feed(Some(name.to_string()), None, None);
        let scan_group = ScanGroup::try_from(&feed_data).ok();

        assert!(scan_group.is_some());

        let unwrapped = scan_group.as_ref().unwrap();

        assert_eq!(unwrapped.name, name.to_string());
        assert_eq!(unwrapped.website, None);
        assert_eq!(unwrapped.description, None);
    }

    #[test]
    fn no_scan_group() {
        let feed_data = create_scan_group_feed(None, None, None);
        let scan_group = ScanGroup::try_from(&feed_data).ok();

        assert!(scan_group.is_none());
    }

    #[test]
    fn full_scan_group() {
        let name = "scan_group_name";
        let website = "some_url";
        let description = "scan_group_description";

        let feed_data = create_scan_group_feed(
            Some(name.to_string()),
            Some(description.to_string()),
            Some(website.to_string()),
        );
        let scan_group = ScanGroup::try_from(&feed_data).ok();

        assert!(scan_group.is_some());

        let unwrapped = scan_group.as_ref().unwrap();

        assert_eq!(unwrapped.name, name.to_string());
        assert_eq!(unwrapped.website, Some(website.to_string()));
        assert_eq!(unwrapped.description, Some(description.to_string()));
    }

    fn create_scan_group_feed(
        name: Option<String>,
        description: Option<String>,
        website: Option<String>,
    ) -> FeedData {
        FeedData {
            id: "id".to_string(),
            attributes: ChapterAttributes {
                chapter: "".to_string(),
                pages: 0,
                external_url: None,
                title: None,
                volume: None,
            },
            relationships: vec![ChapterRelationship {
                id: "id".to_string(),
                rel_type: "scanlation_group".to_string(),
                attributes: Some(ChapterRelationshipAttributes {
                    name,
                    description,
                    website,
                }),
            }],
        }
    }
}
