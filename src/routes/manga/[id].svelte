<script lang="ts">
  import { page } from "$app/stores";
  import {
    ChapterPage,
    downloadChapters,
    getChapters,
    getManga,
    type Manga,
  } from "$lib/commands";
  import ChapterItem from "$lib/components/ChapterItem.svelte";
  import ChaptersPagination from "$lib/components/ChaptersPagination.svelte";
  import MangaInfo from "$lib/components/MangaInfo.svelte";
  import VolumeItem from "$lib/components/VolumeItem.svelte";
  import { DownloadGroup, downloadGroup, selectedChapters } from "$lib/store";
  import { onMount } from "svelte";

  $: id = $page.params["id"];

  let manga: Manga | undefined;
  let chapterPage: ChapterPage | undefined;
  let loading = false;
  let pageLoading = false;

  const limit = 10;
  const groupSelectId = "download-group-select";

  onMount(() => {
    fetchData();
  });

  async function fetchData() {
    loading = true;
    const [mangaData, chapters] = await Promise.all([
      getManga(id),
      getChapters({
        mangaId: id,
        lang: "en",
        limit,
        offset: 0,
      }),
    ]);

    manga = mangaData;
    chapterPage = chapters;
    loading = false;
  }

  async function fetchPage(page: number) {
    pageLoading = true;
    const offset = (page - 1) * limit;

    chapterPage = await getChapters({
      mangaId: id,
      lang: "en",
      limit,
      offset: offset,
    });
    pageLoading = false;
  }
</script>

{#if loading}
  <div class="flex justify-center items-center w-screen h-screen">
    Loading...
  </div>
{:else}
  {#if manga}
    <MangaInfo {manga} />
  {/if}

  <section class="px-5 pb-20 relative">
    <div class="form-control w-52 max-w-xs">
      <label for={groupSelectId} class="label">
        <span class="label-text">Group By</span>
      </label>
      <select
        id={groupSelectId}
        class="select select-bordered"
        bind:value={$downloadGroup}
      >
        {#each Object.values(DownloadGroup) as group}
          <option value={group}>{group}</option>
        {/each}
      </select>
    </div>

    <button
      class="btn btn-primary my-4"
      disabled={$selectedChapters.length === 0}
      on:click={downloadChapters}
    >
      download
    </button>

    {#if chapterPage && manga}
      {#if pageLoading}
        <div class="w-full h-96 flex justify-center items-center">
          Loading...
        </div>
      {:else if $downloadGroup === DownloadGroup.chapter}
        {#each chapterPage.chapters as chapter}
          <ChapterItem {chapter} mangaName={manga.view.title} />
        {/each}
      {:else if $downloadGroup === DownloadGroup.volume}
        {#each chapterPage.volumes as volume}
          <VolumeItem {volume} mangaName={manga.view.title} />
        {/each}
      {/if}

      <div
        class="fixed bottom-0 left-0 right-0 flex justify-center py-2 backdrop-blur-sm bg-slate-700/60"
      >
        <ChaptersPagination
          pages={chapterPage.pages}
          currentPage={chapterPage.currentPage}
          {fetchPage}
        />
      </div>
    {/if}
  </section>
{/if}
