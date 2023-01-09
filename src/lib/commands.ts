import { invoke } from "@tauri-apps/api";
import { get } from "svelte/store";
import { debug, error } from 'tauri-plugin-log-api';
import { selectedChapters } from "./store";


export type MangaView = {
  id: string
  title: string
  status: string
  coverUrl?: string
  description?: string
  genres: string[]
}

export async function search(query: string) {
  try {
    const views: MangaView[] = await invoke('search', { query });
    debug(`received search results: ${JSON.stringify(views, null, 2)}`);
    return views;
  } catch (e) {
    error(`failed to invoke command "search": ${JSON.stringify(e, null, 2)}`);
    return [];
  }
}

export type Manga = {
  view: MangaView,
  year?: number,
  avg_score?: number,
  author?: string,
}

export async function getManga(id: string) {
  try {
    const manga: Manga = await invoke('get_manga', { id });
    debug(`received manga: ${JSON.stringify(manga, null, 2)}`);
    return manga;
  } catch (e) {
    error(`failed to invoke command "getManga": ${JSON.stringify(e, null, 2)}`);
    return undefined;
  }

}

export type GetChapterProps = {
  mangaId: string
  lang: string
  limit: number
  offset: number
}

export type ChaptersResponse = {
  chapters: Chapter[]
  limit: number
  offset: number
  total: number
}

export class ChapterPage {
  constructor(
    private chapterResponse: ChaptersResponse
  ) { }

  static readonly noVolumeKey = '__noVolume';

  get chapters(): Chapter[] {
    return this.chapterResponse.chapters;
  }

  get pages(): number {
    return Math.ceil(this.chapterResponse.total / this.chapterResponse.limit);
  }

  get volumes(): Volume[] {
    const volumeMap = this.chapters.reduce((prev, chapter) => {
      const volume = chapter.volume ?? ChapterPage.noVolumeKey;

      return {
        ...prev,
        [volume]: {
          volume,
          chapters: [...(prev[volume]?.chapters ?? []), { ...chapter }]
        }
      }
    }, {} as { [key: string]: Volume });

    return Object.values(volumeMap);
  }
}

export type Chapter = {
  id: string
  chapter: string
  volume?: string
  title?: string
  scanGroup?: ScanGroup
  pages: number
  externalUrl?: string
}

export type Volume = {
  volume: string
  chapters: Chapter[]
}

export type ScanGroup = {
  id: string,
  name: string,
  website?: string,
  description?: string,
}

export async function getChapters(props: GetChapterProps) {
  try {
    const chapters = await invoke<ChaptersResponse>('get_chapters', props);
    debug(`received chapters: ${JSON.stringify(chapters, null, 2)}`);
    return new ChapterPage(chapters);
  } catch (e) {
    error(`${e}`);
    error(`failed to invoke command "getChapters": ${JSON.stringify(e, null, 2)}`);
    return null;
  }
}

export async function downloadChapters() {
  try {
    const chapters = get(selectedChapters).map(ch => ch.asObject());
    await invoke('download', { chapters });
    debug(`file downloaded`);
  } catch (e) {
    error(`failed to invoke command "download": ${JSON.stringify(e, null, 2)}`);
  }
}


export class AggregatedChapters {
  constructor(
    private data: AggregatedData,
  ) { }


  get chapterIds(): string[] {
    const chapters: string[] = [];

    for (const volume of Object.values(this.data.volumes)) {
      for (const chapter of Object.values(volume.chapters)) {
        chapters.push(chapter.id);
      }
    }

    return chapters;
  }
}

type AggregatedData = {
  volumes: { [key: string]: VolumeAggregate },
}

type VolumeAggregate = {
  volume: string,
  chapters: { [key: string]: ChapterAggregate },
}

type ChapterAggregate = {
  chapter: string,
  id: string,
}

export async function aggregate(id: string, lang: string) {
  try {
    const aggregated = await invoke<AggregatedData>('aggregate', { id, lang });
    return new AggregatedChapters(aggregated);
  } catch (e) {
    error(`failed to invoke command "aggregate": ${JSON.stringify(e, null, 2)}`);
  }
}