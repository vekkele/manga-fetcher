import { invoke } from "@tauri-apps/api";

export type TitleView = {
  name: string
  url: string
}

export function search(query: string): Promise<TitleView[]> {
  return invoke('search', { query });
}

