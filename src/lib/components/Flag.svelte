<script lang="ts">
  export let code: string;
  let hasError = false;

  function handleError() {
    hasError = true;
  }
</script>

{#if hasError}
  <caption>{code}</caption>
{:else}
  <img
    src={`https://mangadex.org/flags/${code}.svg`}
    alt={`${code} lang flag`}
    width="20px"
    on:error={handleError}
  />
{/if}

<style lang="scss">
  @use "@material/theme/color-palette" as scheme;

  caption {
    display: block;
    width: 20px;
    height: 20px;
    background-color: scheme.$green-500;
    border-radius: 4px;
  }
</style>
