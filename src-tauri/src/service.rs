use log::{debug, error};
use thiserror::Error;

use crate::constants::MANGADEX_API;
use crate::model::{
    ApiResponse, Chapter, FeedData, Manga, MangaData, MangaStatistics, MangaView, ResponseError,
    Result, ServiceError, StatisticsResponse,
};

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("provided query is empty")]
    EmptyQuery,

    #[error("failed to request: {}", .0)]
    HttpError(#[from] reqwest::Error),

    #[error("failed to request: {:#?}", .0)]
    ApiErrors(Vec<ResponseError>),
}

pub fn search(query: &str) -> Result<Vec<MangaView>> {
    if query.is_empty() {
        return Err(ServiceError::InvalidArguments("query is empty".to_owned()));
    }

    let search_url = format!("{MANGADEX_API}/manga?title={query}&includes[]=cover_art&limit=5");
    let res: ApiResponse<Vec<MangaData>> = reqwest::blocking::get(search_url)?.json()?;
    let result = res.result("search")?;

    Ok(result.iter().map(MangaView::from).collect())
}

pub fn get_manga(id: &str) -> Result<Manga> {
    let manga_data = fetch_manga_data(id)?;
    let stats = fetch_statistitcs(id)?;

    Ok(Manga::new(&manga_data, &stats))
}

fn fetch_manga_data(id: &str) -> Result<MangaData> {
    let manga_url = format!("{MANGADEX_API}/manga/{id}?includes[]=author&includes[]=cover_art");
    let manga_res: ApiResponse<MangaData> = reqwest::blocking::get(manga_url)?.json()?;
    let manga = manga_res.result("fetch_manga")?;

    debug!("Got manga model: {:#?}", manga);

    Ok(manga)
}

fn fetch_statistitcs(id: &str) -> Result<MangaStatistics> {
    let statistics_url = format!("{MANGADEX_API}/statistics/manga/{id}");
    let mut statistics_res: StatisticsResponse = reqwest::blocking::get(statistics_url)?.json()?;
    let stats = statistics_res.statistics.remove(id).unwrap();

    debug!("Got statistics for {id}: {:#?}", stats);

    Ok(stats)
}

pub fn fetch_feed(id: &str, lang: &str, limit: u32, offset: u32) -> Result<Vec<Chapter>> {
    let feed_url = format!("{MANGADEX_API}/manga/{id}/feed?limit={limit}&offset={offset}&translatedLanguage[]={lang}&includes[]=scanlation_group&order[volume]=asc&order[chapter]=asc");
    let res: ApiResponse<Vec<FeedData>> = reqwest::blocking::get(feed_url)?.json()?;
    let feed_data = res.result("feed")?;

    let chapters = feed_data
        .iter()
        .filter_map(|d| match d.attributes.external_url {
            None => Some(Chapter::from(d)),
            Some(_) => None,
        })
        .collect();

    debug!(
        "Got {limit} chapters from {offset}th for {id} in {lang} lang: {:#?}",
        chapters
    );

    Ok(chapters)
}
