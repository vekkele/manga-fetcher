use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use async_recursion::async_recursion;
use futures::stream::{self, StreamExt};
use log::{debug, error, info};
use reqwest::{Response, Url};
use serde::Serialize;
use thiserror::Error;
use zip::write::FileOptions;

use crate::constants::{MANGADEX_API, MANGADEX_REPORT_URL, MAX_FRAME_RETRIES};

use crate::model::{
    AggregateResponse, ApiResponse, AtHomeResponse, ChapterProps, ChaptersResponse, FeedData,
    Manga, MangaData, MangaStatistics, MangaView, ResponseError, Result, ServiceError,
    StatisticsResponse,
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

pub async fn search(query: &str) -> Result<Vec<MangaView>> {
    if query.is_empty() {
        return Err(ServiceError::InvalidArguments("query is empty".to_owned()));
    }

    let search_url = format!("{MANGADEX_API}/manga?title={query}&includes[]=cover_art&limit=5");
    let res: ApiResponse<Vec<MangaData>> = reqwest::get(search_url).await?.json().await?;
    let result = res.result("search")?;

    Ok(result.iter().map(MangaView::from).collect())
}

pub async fn get_manga(id: &str) -> Result<Manga> {
    let manga_data = fetch_manga_data(id).await?;
    let stats = fetch_statistitcs(id).await?;

    Ok(Manga::new(&manga_data, &stats))
}

async fn fetch_manga_data(id: &str) -> Result<MangaData> {
    let manga_url = format!("{MANGADEX_API}/manga/{id}?includes[]=author&includes[]=cover_art");
    let manga_res: ApiResponse<MangaData> = reqwest::get(manga_url).await?.json().await?;
    let manga = manga_res.result("fetch_manga")?;

    debug!("Got manga model: {:#?}", manga);

    Ok(manga)
}

async fn fetch_statistitcs(id: &str) -> Result<MangaStatistics> {
    let statistics_url = format!("{MANGADEX_API}/statistics/manga/{id}");
    let mut statistics_res: StatisticsResponse = reqwest::get(statistics_url).await?.json().await?;
    let stats = statistics_res.statistics.remove(id).unwrap();

    debug!("Got statistics for {id}: {:#?}", stats);

    Ok(stats)
}

pub async fn fetch_feed(id: &str, lang: &str, limit: u32, offset: u32) -> Result<ChaptersResponse> {
    let feed_url = format!("{MANGADEX_API}/manga/{id}/feed?limit={limit}&offset={offset}&translatedLanguage[]={lang}&includes[]=scanlation_group&order[volume]=asc&order[chapter]=asc");
    let res: ApiResponse<Vec<FeedData>> = reqwest::get(feed_url).await?.json().await?;
    let response: ChaptersResponse = res.try_into()?;

    Ok(response)
}

pub async fn aggregate(id: &str, lang: &str) -> Result<AggregateResponse> {
    let aggregate_url = format!("{MANGADEX_API}/manga/{id}/aggregate?translatedLanguage[]={lang}");
    let res: AggregateResponse = reqwest::get(aggregate_url).await?.json().await?;

    Ok(res)
}

pub async fn download(chapters: Vec<ChapterProps>) -> Result<()> {
    let stream = stream::iter(chapters)
        .map(download_chapter)
        .buffer_unordered(100);

    stream
        .for_each(|e| async {
            match e {
                Ok(_) => info!("Successfully downloaded some chapter"),
                Err(e) => error!("Failed to download {e}"),
            };
        })
        .await;

    Ok(())
}

pub async fn download_chapter(chapter: ChapterProps) -> Result<()> {
    const CONCURRENT_FRAMES: usize = 30;

    let chapter_id = chapter.id;
    let chapter_name = chapter.fullname;
    let at_home = get_at_home(&chapter_id).await?;

    let base_url = at_home.base_url;
    let hash = at_home.chapter.hash;

    let client = reqwest::Client::new();
    let chapter_path_buf = get_chapter_path(&chapter_name);
    let chapter_path = chapter_path_buf.as_path();

    fs::create_dir_all(chapter_path)?;

    let frame_urls = at_home.chapter.data_saver;
    let total_frames = frame_urls.len();
    let stream = stream::iter(frame_urls)
        .enumerate()
        .map(|(index, file_name)| {
            let frame_url = get_frame_url(&base_url, &hash, &file_name);

            download_frame(
                &chapter_id,
                frame_url,
                file_name,
                &client,
                chapter_path,
                index,
                total_frames,
                MAX_FRAME_RETRIES,
            )
        })
        .buffer_unordered(CONCURRENT_FRAMES);

    stream
        .for_each(|e| async {
            match e {
                Ok(name) => info!("Successfully downloaded {name}"),
                Err(e) => error!("Failed to download {e}"),
            };
        })
        .await;

    write_zip(chapter_path, &chapter_name)?;

    Ok(())
}

fn get_chapter_path(chapter_name: &str) -> PathBuf {
    let mut chapter_path_buf =
        tauri::api::path::download_dir().expect("downloads path must be present");
    chapter_path_buf.push(chapter_name);
    chapter_path_buf.to_owned()
}

fn get_frame_name(file_name: &str, frame_index: usize, total_frames: usize) -> String {
    let ext = Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("jpg");
    let pad_width = get_pad_width(total_frames);

    format!("{:0width$}.{ext}", frame_index, width = pad_width)
}

fn get_pad_width(max_num: usize) -> usize {
    max_num.to_string().len()
}

async fn get_at_home(chapter_id: &str) -> Result<AtHomeResponse> {
    let at_home_url = format!("{MANGADEX_API}/at-home/server/{chapter_id}");

    let res: AtHomeResponse = reqwest::get(at_home_url).await?.json().await?;

    Ok(res)
}

fn get_frame_url(base_url: &str, hash: &str, file_name: &str) -> String {
    format!("{base_url}/data-saver/{hash}/{file_name}")
}

//FIXME: Fix too many args warning
#[async_recursion]
#[allow(clippy::too_many_arguments)]
async fn download_frame(
    chapter_id: &str,
    frame_url: String,
    file_name: String,
    client: &reqwest::Client,
    chapter_path: &Path,
    frame_index: usize,
    total_frames: usize,
    retries_left: u32,
) -> Result<String> {
    let frame_name = get_frame_name(&file_name, frame_index, total_frames);
    let file_path = chapter_path.join(&frame_name);
    let start = Instant::now();
    let response = client.get(frame_url).send().await?;
    let end = Instant::now();
    let duration = end.duration_since(start).as_millis();

    let download_successful = report_frame(client, &response, duration).await?;
    let has_retries_left = retries_left - 1 > 0;
    if !download_successful && has_retries_left {
        let at_home = get_at_home(chapter_id).await?;
        let frame_urls = &at_home.chapter.data_saver;
        let file_name = &frame_urls[frame_index];
        let frame_url = get_frame_url(&at_home.base_url, &at_home.chapter.hash, file_name);

        return download_frame(
            chapter_id,
            frame_url,
            file_name.clone(),
            client,
            chapter_path,
            frame_index,
            total_frames,
            retries_left - 1,
        )
        .await;
    }

    let file_data = response.bytes().await?;

    fs::write(file_path, file_data)?;

    Ok(file_name)
}

#[derive(Serialize)]
struct FrameReport<'a> {
    url: &'a Url,
    success: bool,
    cached: bool,
    bytes: u64,
    duration: u128,
}

async fn report_frame(
    client: &reqwest::Client,
    response: &Response,
    duration: u128,
) -> Result<bool> {
    let url = response.url();
    let status = response.status();
    let success = status.is_success();

    let has_mangadex_domain = match url.host_str() {
        Some(host) => host.contains("mangadex.org"),
        None => false,
    };

    if has_mangadex_domain {
        return Ok(success);
    }

    let bytes = response.content_length().unwrap_or(0);
    let cache = response.headers().get("X-Cache");
    let cached = match cache {
        Some(cache) => cache.to_str().unwrap_or("").contains("HIT"),
        None => false,
    };

    client
        .post(MANGADEX_REPORT_URL)
        .json(&FrameReport {
            url,
            success,
            bytes,
            cached,
            duration,
        })
        .send()
        .await?;

    Ok(success)
}

fn write_zip(chapter_path: &Path, chapter_name: &str) -> Result<()> {
    let zip_path = chapter_path.with_file_name(format!("{chapter_name}.cbz"));
    let zip_file = fs::File::create(zip_path)?;

    let mut writer = zip::ZipWriter::new(zip_file);

    let method = zip::CompressionMethod::Stored;
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in fs::read_dir(chapter_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = path.strip_prefix(chapter_path).unwrap().to_str().unwrap();
        writer.start_file(file_name, options)?;

        let mut f = fs::File::open(path)?;
        f.read_to_end(&mut buffer)?;
        writer.write_all(&*buffer)?;
        buffer.clear()
    }

    writer.finish()?;

    fs::remove_dir_all(chapter_path)?;

    Ok(())
}
