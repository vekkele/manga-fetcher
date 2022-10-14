<script lang="ts">
  import type { Volume } from "$lib/commands";
  import ChapterItem from "./ChapterItem.svelte";

  export let volume: Volume;
  export let mangaName: string;

  let checked = false;
</script>

<div
  class="collapse collapse-open border border-base-300 bg-base-100 rounded-box m-2"
>
  <div class="collapse-title text-xl font-medium">
    <label class="volume-select flex items-center">
      <input type="checkbox" class="checkbox m-2" bind:checked />
      <span>Volume {volume.volume}</span>
    </label>
  </div>
  <div class="collapse-content">
    {#each volume.chapters as chapter}
      <ChapterItem {chapter} {mangaName} />
    {/each}
  </div>
</div>

<style lang="postcss">
  .volume-select:hover {
    @apply bg-slate-600;
    @apply transition-colors;
    @apply rounded-md;
    @apply select-none;
  }
</style>
