<script lang="ts">
  import type { ScanGroup } from "$lib/commands";
  import Info from "$lib/icons/Info.svelte";
  import Markdown from "../Markdown.svelte";
  import Modal from "../Modal.svelte";

  export let chapterId: string;
  export let scanGroup: ScanGroup;
  export let triggerClass = "";
</script>

<Modal id={`chapter-info-${chapterId}`}>
  <label
    slot="trigger"
    let:id
    for={id}
    class={`btn btn-circle btn-xs text-white ${triggerClass}`}
  >
    <Info class="w-full h-full" />
  </label>

  <h2 class="text-xl mb-4">Scanlation Group info</h2>

  {#if scanGroup.website}
    <a href={scanGroup.website} target="_blank" class="link text-lg"
      >{scanGroup.name}</a
    >
  {:else}
    <h3 class="text-lg">{scanGroup.name}</h3>
  {/if}

  {#if scanGroup.description}
    <Markdown source={scanGroup.description} />
  {/if}
</Modal>
