<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import {
    getChapters,
    getManga,
    type Chapter,
    type Manga,
  } from "$lib/commands";

  $: id = $page.params["id"];

  function back() {
    goto("/");
  }

  let manga: Manga | undefined;
  let chapters: Chapter[];

  async function fetchData() {
    manga = await getManga(id);
    chapters = await getChapters({
      mangaId: id,
      lang: "en",
      limit: 10,
      offset: 0,
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

<h2>Manga</h2>
<pre>{JSON.stringify(manga, null, 2)}</pre>

<h2>Chapters</h2>
<pre>{JSON.stringify(chapters, null, 2)}</pre>
