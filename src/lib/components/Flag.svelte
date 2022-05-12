<script lang="ts">
  export let code: string;
  let hasError = false;

  function handleError() {
    hasError = true;
  }
</script>

{#if hasError}
  <span
    class="flex items-center justify-center w-5 h-5 rounded bg-primary text-white"
  >
    {code}
  </span>
{:else}
  <img
    src={`https://mangadex.org/flags/${code}.svg`}
    alt={`${code} lang flag`}
    width="20px"
    on:error={handleError}
  />
{/if}
