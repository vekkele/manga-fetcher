import { writable } from "svelte/store";
import { search } from "$lib/commands";
import type { MangaView } from "$lib/commands";

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