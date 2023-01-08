<script lang="ts">
  import Search from "$lib/icons/Search.svelte";
  import { mangaList } from "$lib/store";

  let query = "";
  const loading = mangaList.loading;
  $: searchDisabled = !query || $loading;

  function submit() {
    if (searchDisabled) return;
    mangaList.search(query);
  }
</script>

<section class="search-section">
  <label class="label" for="search-input">
    <span class="label-text">Search Query</span>
  </label>
  <form on:submit|preventDefault={submit} class="input-group">
    <input
      id="search-input"
      type="text"
      class="input input-bordered flex-1"
      spellcheck="false"
      bind:value={query}
    />
    <button
      class="btn btn-square btn-primary"
      disabled={searchDisabled}
      type="submit"
    >
      <Search class="h-6 w-6" />
    </button>
  </form>
</section>

<style lang="postcss">
  .search-section {
    @apply form-control pb-4 max-w-md;
    min-width: 15rem; /* 240px */
  }
</style>
