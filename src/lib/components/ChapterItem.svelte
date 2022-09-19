<script lang="ts">
  import type { Chapter } from "$lib/commands";
  import { selectedChapters } from "$lib/store";

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
  class={`block w-full label-text ${typeClass}`}
  target="_blank"
>
  <label class="flex gap-2 label cursor-pointer justify-start">
    <input
      type="checkbox"
      class="checkbox"
      disabled={!canDownload}
      bind:checked={selected}
      on:change={(e) => toggle({ id, fullname }, e.currentTarget.checked)}
    />
    <span>{chapterName}</span>
    {#if !canDownload}
      <svg
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        width="16px"
        height="16px"
      >
        <path
          d="M 3 3 L 3 21 L 21 21 L 21 12 L 19 12 L 19 19 L 5 19 L 5 5 L 12 5 L 12 3 L 3 3 z M 14 3 L 14 5 L 17.585938 5 L 8.2929688 14.292969 L 9.7070312 15.707031 L 19 6.4140625 L 19 10 L 21 10 L 21 3 L 14 3 z"
        />
      </svg>
    {/if}
  </label>
</a>

<style lang="postcss">
  .chapter-link:hover {
    @apply underline;
  }

  .chapter-select:hover {
    @apply bg-slate-600;
    @apply transition-colors;
    @apply rounded-md;
  }
</style>
