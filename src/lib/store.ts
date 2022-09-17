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

function createSelectedChapters() {
  const { subscribe, update, set } = writable<string[]>([]);
  const add = (id: string) => update(prev => ([...prev, id]));
  const remove = (id: string) => update(prev => prev.filter(chapterId => chapterId !== id));

  return {
    subscribe,
    toggle: (id: string, selected: boolean) => {
      if (selected) {
        remove(id);
      } else {
        add(id);
      }
    },
    clear: () => set([]),
  }
}

export const selectedChapters = createSelectedChapters();