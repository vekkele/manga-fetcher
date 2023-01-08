<script lang="ts">
  import type { MangaView } from "$lib/commands";
  import Status from "./Status.svelte";

  export let manga: MangaView;
</script>

<div class="manga-entry card card-side bg-base-300 shadow-xl h-36">
  <div class="relative">
    <img
      class="w-24 max-w-none object-cover h-full"
      src={manga.coverUrl}
      alt="manga cover"
    />
    <a class="img-overlay" href={`manga/${manga.id}`}>
      <span class="overlay-text">Open</span>
    </a>
  </div>

  <div class="w-full min-w-0 p-2 overflow-hidden">
    <h2 class="flex card-title justify-between">
      <span class="whitespace-nowrap overflow-hidden text-ellipsis">
        {manga.title}
      </span>
      <Status status={manga.status} />
    </h2>

    <div class="flex flex-wrap overflow-hidden h-5 gap-2 my-1">
      {#each manga.genres as genre}
        <div class="badge bg-amber-400 text-black">
          {genre}
        </div>
      {/each}
    </div>

    <p class="description">{manga.description}</p>
  </div>
</div>

<style lang="postcss">
  .manga-entry {
    min-width: 19rem; /* 304px */
    width: 31rem; /* 496px */
  }

  .img-overlay {
    @apply flex justify-center items-center;
    @apply w-full h-full;
    @apply absolute top-0 left-0;
    @apply hover:bg-primary transition-all cursor-pointer;
  }

  .overlay-text {
    @apply uppercase text-white text-2xl opacity-0 transition-opacity;

    .img-overlay:hover > & {
      @apply opacity-100;
    }
  }

  .description {
    @apply overflow-hidden text-ellipsis;
    @apply text-xs mt-2;
    display: -webkit-box;
    -webkit-line-clamp: 4;
    -webkit-box-orient: vertical;
  }
</style>
