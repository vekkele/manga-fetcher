<script lang="ts">
  import Button from "@smui/button";
  import TextField from "@smui/textfield";
  import { search, TitleView } from "src/commands";
  import router, { Route } from "src/router/router";

  let query = "";
  let titlesPromise: Promise<TitleView[]>;

  async function submit() {
    titlesPromise = search(query);
  }

  function toTitle() {
    router.navigate(Route.Title);
  }
</script>

<TextField variant="outlined" label="Search Query" bind:value={query} />
<Button on:click={submit}>Search</Button>
<Button on:click={toTitle}>To Title</Button>

{#if titlesPromise}
  {#await titlesPromise}
    <p>Loading...</p>
  {:then titles}
    {#each titles as title}
      <h4>{title.name}</h4>
      <h6>{title.url}</h6>
    {/each}
  {/await}
{/if}
