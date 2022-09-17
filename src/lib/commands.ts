import { invoke } from "@tauri-apps/api";
import { get } from "svelte/store";
import { debug, error } from 'tauri-plugin-log-api';
import { selectedChapters } from "./store";


export type MangaView = {
  id: string
  title: string
  status: string
  coverUrl?: string
  langCodes: string[]
}

export async function search(query: string) {
  try {
    const views: MangaView[] = await invoke('search', { query });
    debug(`received search results: ${JSON.stringify(views, null, 4)}`);
    return views;
  } catch (e) {
    error(`failed to invoke command "search": ${JSON.stringify(e, null, 4)}`);
    return [];
  }
}

export type Manga = {
  view: MangaView
  description?: string,
  genres: string[],
  year?: number,
  avg_score?: number,
  author?: string,
}

export async function getManga(id: string) {
  try {
    const manga: Manga = await invoke('get_manga', { id });
    debug(`received manga: ${JSON.stringify(manga, null, 4)}`);
    return manga;
  } catch (e) {
    error(`failed to invoke command "getManga": ${JSON.stringify(e, null, 4)}`);
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
  chapters: Chapter[]
  limit: number
  offset: number
  total: number

  constructor(res: ChaptersResponse) {
    this.chapters = res.chapters;
    this.limit = res.limit;
    this.offset = res.offset;
    this.total = res.total;
  }

  public get pages(): number {
    return Math.ceil(this.total / this.limit);
  }

  public get currentPage(): number {
    return this.offset / this.limit + 1;
  }
}

export type Chapter = {
  id: string
  chapter: string
  volume?: string
  title?: string
  scanGroup?: ScanGroup
  pages: number
}

export type ScanGroup = {
  name: string,
  id: string,
}

export async function getChapters(props: GetChapterProps) {
  try {
    const chapters: ChaptersResponse = await invoke('get_chapters', props);
    debug(`received chapters: ${JSON.stringify(chapters, null, 4)}`);
    return new ChapterPage(chapters);
  } catch (e) {
    error(`failed to invoke command "getChapters": ${JSON.stringify(e, null, 4)}`);
    return undefined;
  }
}

export async function downloadChapters() {
  try {
    const chapters = get(selectedChapters);
    await invoke('download', { chapters });
    debug(`file downloaded`);
  } catch (e) {
    error(`failed to invoke command "download": ${JSON.stringify(e, null, 4)}`);
  }
}