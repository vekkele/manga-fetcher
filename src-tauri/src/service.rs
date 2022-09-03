use std::fs;
use std::path::{Path, PathBuf};

use futures::stream::{self, StreamExt};
use log::{debug, error, info};
use thiserror::Error;

use crate::constants::MANGADEX_API;

use crate::model::{
    ApiResponse, AtHomeResponse, ChaptersResponse, FeedData, Manga, MangaData, MangaStatistics,
    MangaView, ResponseError, Result, ServiceError, StatisticsResponse,
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

pub fn fetch_feed(id: &str, lang: &str, limit: u32, offset: u32) -> Result<ChaptersResponse> {
    let feed_url = format!("{MANGADEX_API}/manga/{id}/feed?limit={limit}&offset={offset}&translatedLanguage[]={lang}&includes[]=scanlation_group&order[volume]=asc&order[chapter]=asc");
    let res: ApiResponse<Vec<FeedData>> = reqwest::blocking::get(feed_url)?.json()?;
    let response: ChaptersResponse = res.try_into()?;

    Ok(response)
}

//FIXME: Remove
const CHAPTER_ID: &str = "a54c491c-8e4c-4e97-8873-5b79e59da210";

pub async fn download_chapter() -> Result<()> {
    const CONCURRENT_FRAMES: usize = 30;

    let at_home_url = format!("{MANGADEX_API}/at-home/server/{CHAPTER_ID}");
    let res: AtHomeResponse = reqwest::get(at_home_url).await?.json().await?;

    let base_url = res.base_url;
    let hash = res.chapter.hash;

    let client = reqwest::Client::new();
    let chapter_path_buf = get_chapter_path();
    let chapter_path = chapter_path_buf.as_path();

    fs::create_dir_all(chapter_path)?;

    let stream = stream::iter(res.chapter.data_saver)
        .map(|file_name| download_frame(file_name, &client, &base_url, &hash, chapter_path))
        .buffer_unordered(CONCURRENT_FRAMES);

    stream
        .for_each(|e| async {
            match e {
                Ok(name) => info!("Successfully downloaded {name}"),
                Err(e) => error!("Failed to download {e}"),
            };
        })
        .await;

    Ok(())
}

fn get_chapter_path() -> PathBuf {
    let mut chapter_path_buf =
        tauri::api::path::download_dir().expect("downloads path must be present");
    chapter_path_buf.push("chapter");
    chapter_path_buf.to_owned()
}

async fn download_frame(
    file_name: String,
    client: &reqwest::Client,
    base_url: &str,
    hash: &str,
    chapter_path: &Path,
) -> Result<String> {
    let frame_url = format!("{base_url}/data-saver/{hash}/{file_name}");
    let file_path = chapter_path.join(&file_name);
    let file_data = client.get(frame_url).send().await?.bytes().await?;

    fs::write(file_path, file_data)?;

    Ok(file_name)
}
