import { writable } from "svelte/store";
import { search, type MangaView } from "$lib/commands";

function createMangaList() {
  const { subscribe, set } = writable<MangaView[] | undefined>();
  const loading = writable(false);

  return {
    subscribe,
    loading,
    search: async (query: string) => {
      if (query === "") return;

      loading.set(true);

      const result = await search(query);
      set(result);
      loading.set(false);
    }
  }
}

export const mangaList = createMangaList();

export type ChapterProps = {
  id: string
  fullname: string
}

function createSelectedChapters() {
  const { subscribe, update, set } = writable<ChapterProps[]>([]);
  const add = (chapter: ChapterProps) => update(prev => ([...prev, chapter]));
  const remove = (id: string) => update(prev => prev.filter(chapter => id !== chapter.id));

  return {
    subscribe,
    toggle: (chapter: ChapterProps, selected: boolean) => {
      if (selected) {
        remove(chapter.id);
      } else {
        add(chapter);
      }
    },
    clear: () => set([]),
  }
}

export const selectedChapters = createSelectedChapters();

export enum DownloadGroup {
  chapter = 'Chapter',
  volume = 'Volume'
}

function createDownloadGroup() {
  const { set, ...state } = writable(DownloadGroup.chapter);

  return {
    ...state,
    set: (value: DownloadGroup) => {
      selectedChapters.clear();
      set(value);
    }
  }
}

export const downloadGroup = createDownloadGroup();