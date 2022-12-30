<script lang="ts">
  import { computePageGroups } from "$lib/helpers/pagination";
  import PageButton from "./PageButton.svelte";

  export let pages: number;
  export let currentPage: number;
  export let fetchPage: (page: number) => void;

  $: firstPage = currentPage === 1;
  $: lastPage = currentPage === pages;

  $: ({ start, currentGroup, end } = computePageGroups(currentPage, pages));

  function nextPage() {
    fetchPage(currentPage + 1);
  }

  function prevPage() {
    fetchPage(currentPage - 1);
  }
</script>

<div class="btn-group">
  <button class="btn btn-md" disabled={firstPage} on:click={prevPage}>«</button>
  {#if start}
    <PageButton page={start} {currentPage} {fetchPage} />
    <button class="btn btn-md" disabled>...</button>
  {/if}

  {#each currentGroup as page}
    <PageButton {page} {currentPage} {fetchPage} />
  {/each}

  {#if end}
    <button class="btn btn-md" disabled>...</button>
    <PageButton page={end} {currentPage} {fetchPage} />
  {/if}

  <button class="btn btn-md" disabled={lastPage} on:click={nextPage}>»</button>
</div>
