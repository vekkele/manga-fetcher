<script lang="ts">
  import MangaEntry from "$lib/components/MangaEntry.svelte";
  import SearchForm from "$lib/components/SearchForm.svelte";
  import { mangaList } from "$lib/store";

  const loading = mangaList.loading;
</script>

<div class="p-5">
  <SearchForm />

  {#if $mangaList}
    {#if $loading}
      <div>Loading...</div>
    {:else}
      <section class="flex flex-wrap gap-4">
        {#if $mangaList.length === 0}
          <h2>Nothing Found</h2>
        {/if}
        {#each $mangaList as manga}
          <MangaEntry {manga} />
        {/each}
      </section>
    {/if}
  {/if}
</div>
