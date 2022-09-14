<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import {
    ChapterPage,
    getChapters,
    getManga,
    type Manga,
  } from "$lib/commands";
  import ChaptersPagination from "$lib/components/ChaptersPagination.svelte";

  $: id = $page.params["id"];

  function back() {
    goto("/");
  }

  let manga: Manga | undefined;
  let chapterPage: ChapterPage | undefined;

  const limit = 10;

  async function fetchData() {
    manga = await getManga(id);
    chapterPage = await getChapters({
      mangaId: id,
      lang: "en",
      limit,
      offset: 0,
    });
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

<h1>Title info {id}</h1>
<button class="btn btn-primary" on:click={fetchData}>Get Manga</button>

{#if chapterPage}
  {#each chapterPage.chapters as chapter}
    <div>
      {`Chapter ${chapter.chapter}${
        chapter.title ? ` - ${chapter.title}` : ""
      }`}
    </div>
  {/each}

  <ChaptersPagination
    pages={chapterPage.pages}
    currentPage={chapterPage.currentPage}
    {fetchPage}
  />
{/if}

<h2>Manga</h2>
<pre>{JSON.stringify(manga, null, 2)}</pre>

<h2>Chapters</h2>
<pre>{JSON.stringify(chapterPage, null, 2)}</pre>
