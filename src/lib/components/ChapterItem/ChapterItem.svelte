<script lang="ts">
  import type { Chapter } from "$lib/commands";
  import Link from "$lib/icons/Link.svelte";
  import { selectedChapters } from "$lib/store";
  import ScanGroupInfoModal from "./ScanGroupInfoModal.svelte";

  export let chapter: Chapter;
  export let mangaName: string;

  $: ({ chapter: chapterNum, title, id } = chapter);
  $: selected = $selectedChapters.some((ch) => ch.id === id);
  $: chapterName = `Chapter ${chapterNum}${title ? ` - ${title}` : ""}`;
  $: fullname = `${mangaName} ${chapterName}`;

  $: canDownload = !chapter.externalUrl;
  $: typeClass = canDownload ? "chapter-select" : "chapter-link";

  const { toggle } = selectedChapters;
</script>

<a
  href={chapter.externalUrl}
  class={`group block w-full label-text ${typeClass}`}
  target="_blank"
>
  <label class="flex gap-2 label cursor-pointer justify-start items-center">
    <input
      type="checkbox"
      class="checkbox"
      disabled={!canDownload}
      bind:checked={selected}
      on:change={(e) => toggle({ id, fullname }, e.currentTarget.checked)}
    />
    <span>{chapterName}</span>
    {#if chapter.scanGroup && canDownload}
      <div class="ml-auto">
        <ScanGroupInfoModal
          triggerClass="opacity-0 group-hover:opacity-100 transition-all"
          chapterId={id}
          scanGroup={chapter.scanGroup}
        />
      </div>
    {/if}
    {#if !canDownload}
      <Link class="w-4 h-4" />
    {/if}
  </label>
</a>

<style lang="postcss">
  .chapter-link:hover {
    @apply link;
  }

  .chapter-select:hover {
    @apply bg-slate-600;
    @apply transition-colors;
    @apply rounded-md;
  }
</style>
