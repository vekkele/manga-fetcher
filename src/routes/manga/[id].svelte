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
  import { selectedChapters } from "$lib/store";
  import { onMount } from "svelte";

  $: id = $page.params["id"];

  function back() {
    selectedChapters.clear();
    goto("/");
  }

  let manga: Manga | undefined;
  let chapterPage: ChapterPage | undefined;

  const limit = 10;

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

<button
  class="btn btn-primary my-4"
  disabled={$selectedChapters.length === 0}
  on:click={downloadChapters}
>
  download
</button>

{#if chapterPage && manga}
  {#each chapterPage.chapters as chapter}
    <ChapterItem {chapter} mangaName={manga.view.title} />
  {/each}

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
