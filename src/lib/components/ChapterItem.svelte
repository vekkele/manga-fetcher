<script lang="ts">
  import type { Chapter } from "$lib/commands";
  import { selectedChapters } from "$lib/store";

  export let chapter: Chapter;

  $: ({ chapter: chapterNum, title, id } = chapter);
  $: selected = $selectedChapters.includes(id);

  const { toggle } = selectedChapters;
</script>

<label class="label cursot-pointer justify-start">
  <input
    type="checkbox"
    class="checkbox mx-2"
    bind:checked={selected}
    on:change={(e) => toggle(id, e.currentTarget.checked)}
  />
  <span class="label-text">
    {`Chapter ${chapterNum}${title ? ` - ${title}` : ""}`}
  </span>
</label>
