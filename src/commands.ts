import { invoke } from "@tauri-apps/api";
import { error } from 'tauri-plugin-log-api';


export type MangaView = {
  id: string
  title: string
  status: string
}

export async function search(query: string): Promise<MangaView[]> {
  try {
    return invoke('search', { query });
  } catch (e) {
    error(`failed to invoke command "search": ${e}`)
    return [];
  }
}

