import { writable } from "svelte/store";
import { MangaView, search } from "./commands";

export const selectedManga = writable<string | null>();

function createSearchResults() {
  const { subscribe, set } = writable<MangaView[] | null>();
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

export const searchResults = createSearchResults();