<script lang="ts">
  import { goto } from "$app/navigation";
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

  function back() {
    selectedChapters.clear();
    goto("/");
  }

  let manga: Manga | undefined;
  let chapterPage: ChapterPage | undefined;

  const limit = 10;
  const groupSelectId = "download-group-select";

  onMount(() => {
    fetchData();
  });

  async function fetchData() {
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
  }

  async function fetchPage(page: number) {
    const offset = (page - 1) * limit;

    chapterPage = await getChapters({
      mangaId: id,
      lang: "en",
      limit,
      offset: offset,
    });
  }
</script>

<button class="btn" on:click={back}>To Search</button>

<section class="flex">
  <div>
    <img src="" alt="" />
  </div>
</section>

{#if manga}
  <MangaInfo {manga} />
{/if}

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
  {#if $downloadGroup === DownloadGroup.chapter}
    {#each chapterPage.chapters as chapter}
      <ChapterItem {chapter} mangaName={manga.view.title} />
    {/each}
  {:else if $downloadGroup === DownloadGroup.volume}
    {#each chapterPage.volumes as volume}
      <VolumeItem {volume} mangaName={manga.view.title} />
    {/each}
  {/if}

  <ChaptersPagination
    pages={chapterPage.pages}
    currentPage={chapterPage.currentPage}
    {fetchPage}
  />
{/if}

<h2>Selected Chapters:</h2>
{#each $selectedChapters as ch, i}
  <div>{i}: {ch}</div>
{/each}

<h2>Manga</h2>
<pre>{JSON.stringify(manga, null, 2)}</pre>

<h2>Chapters</h2>
<pre>{JSON.stringify(chapterPage, null, 2)}</pre>

<h2>Volumes</h2>
<pre>{JSON.stringify(chapterPage?.volumes, null, 2)}</pre>
