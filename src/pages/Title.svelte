<script>
  import Button from "@smui/button";
  import { router, Route } from "src/router/router";
  import { selectedManga } from "src/store";

  function back() {
    selectedManga.set(null);
    router.navigate(Route.Search);
  }
</script>

<Button on:click={back}>To Search</Button>

{#if !back}
  <h1>Invalid selectd manga</h1>
{:else}
  <h1>Title info</h1>
{/if}
