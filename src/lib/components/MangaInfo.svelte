<script lang="ts">
  import type { Manga } from "$lib/commands";

  export let manga: Manga;
</script>

<section>
  <header class="flex">
    <img
      src={manga.view.coverUrl}
      alt="manga cover"
      loading="lazy"
      width="200"
    />
    <div class="p-4">
      <h1 class="text-6xl">{manga.view.title}</h1>
      <h3 class="my-2 text-xl">{manga.author}</h3>

      <div class="flex flex-wrap gap-2 my-2">
        {#each manga.genres as genre}
          <div class="px-2 rounded-xl bg-amber-400 text-black">
            {genre}
          </div>
        {/each}
      </div>

      <div>
        <span>Publication: </span>
        <span>{manga.year}</span>
        <span>{manga.view.status}</span>
      </div>
    </div>
  </header>
</section>
