<script lang="ts">
  export let pages: number;
  export let currentPage: number;
  export let fetchPage: (page: number) => void;

  $: firstPage = currentPage === 1;
  $: lastPage = currentPage === pages;

  $: pageList = Array(pages)
    .fill(0)
    .map((_, i) => i + 1);

  $: isActive = (page: number) => (page === currentPage ? "btn-active" : "");

  function nextPage() {
    fetchPage(currentPage + 1);
  }

  function prevPage() {
    fetchPage(currentPage - 1);
  }
</script>

<div class="btn-group">
  <button class="btn btn-md" disabled={firstPage} on:click={prevPage}>«</button>

  {#each pageList as page}
    <button
      on:click={() => fetchPage(page)}
      class="btn btn-md {isActive(page)}"
    >
      {page}
    </button>
  {/each}

  <button class="btn btn-md" disabled={lastPage} on:click={nextPage}>»</button>
</div>
