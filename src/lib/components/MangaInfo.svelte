<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Manga } from "$lib/commands";
  import { selectedChapters } from "$lib/store";
  import Markdown from "./Markdown.svelte";

  export let manga: Manga;

  function back() {
    selectedChapters.clear();
    goto("/");
  }
</script>

<section>
  <header class="flex">
    <div>
      <button
        class="btn btn-xs btn-block btn-primary rounded-none"
        on:click={back}>BACK</button
      >
      <img
        class="rounded-br-md"
        src={manga.view.coverUrl}
        alt="manga cover"
        loading="lazy"
        width="400"
      />
    </div>
    <div class="p-4 max-w-4xl">
      <h1 class="text-6xl">{manga.view.title}</h1>
      <h3 class="my-2 text-xl">{manga.author}</h3>

      <div class="flex flex-wrap gap-2 my-2">
        {#each manga.view.genres as genre}
          <div class="px-2 rounded-xl bg-amber-400 text-black">
            {genre}
          </div>
        {/each}
      </div>

      <div class="font-bold mb-2">
        <span>Publication: </span>
        <span>{manga.year}</span>
        <span>{manga.view.status}</span>
      </div>

      <Markdown source={manga.view.description} />
    </div>
  </header>
</section>
