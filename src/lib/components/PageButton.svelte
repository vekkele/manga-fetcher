<script lang="ts">
  export let page: number;
  export let currentPage: number;
  export let fetchPage: (page: number) => void;

  $: isActive = (page: number) => (page === currentPage ? "btn-active" : "");
</script>

<button on:click={() => fetchPage(page)} class="btn btn-md {isActive(page)}">
  {page}
</button>
