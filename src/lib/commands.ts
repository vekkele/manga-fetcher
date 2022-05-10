import { invoke } from "@tauri-apps/api";
import { debug, error } from 'tauri-plugin-log-api';


export type MangaView = {
  id: string
  title: string
  status: string
  coverUrl?: string
  langCodes: string[]
}

export async function search(query: string): Promise<MangaView[]> {
  try {
    const views: MangaView[] = await invoke('search', { query });
    debug(`received search results: ${JSON.stringify(views, null, 4)}`);
    return views;
  } catch (e) {
    error(`failed to invoke command "search": ${JSON.stringify(e, null, 4)}`);
    return [];
  }
}

