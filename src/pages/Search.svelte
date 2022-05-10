<script lang="ts">
  import Button from "@smui/button";
  import Card from "@smui/card";
  import LinearProgress from "@smui/linear-progress";
  import TextField from "@smui/textfield";
  import Ripple from "@smui/ripple";

  import Flag from "src/lib/Flag.svelte";
  import Status from "src/lib/Status.svelte";
  import { router, Route } from "src/router/router";
  import { searchResults, selectedManga } from "src/store";

  let query = "";
  const loading = searchResults.loading;

  function submit() {
    searchResults.search(query);
  }

  function toMangaPage(id: string) {
    selectedManga.set(id);
    router.navigate(Route.Title);
  }
</script>

<section class="search-section">
  <TextField
    class="search-field"
    variant="outlined"
    label="Search Query"
    bind:value={query}
  />
  <Button class="search-button" on:click={submit}>Search</Button>
</section>

{#if $loading}
  <LinearProgress indeterminate />
{:else}
  <section class="result-section">
    {#if $searchResults?.length === 0}
      <h2>No Results</h2>
    {/if}
    {#each $searchResults ?? [] as manga}
      <Card class="card">
        <div
          class="card-body"
          use:Ripple={{ surface: true }}
          on:click={() => toMangaPage(manga.id)}
        >
          <img
            src={manga.coverUrl}
            alt="manga cover"
            height="150"
            width="100"
          />
          <div class="card-content">
            <div class="title-section">
              <h3 class="title">{manga.title}</h3>
              <Status status={manga.status} />
            </div>
            <div class="information">
              <div class="lang-grid">
                {#each manga.langCodes.slice(0, 9) as code}
                  <Flag {code} />
                {/each}
              </div>
            </div>
          </div>
        </div>
      </Card>
    {/each}
  </section>
{/if}

<style lang="scss">
  .search-section {
    display: flex;
    justify-content: start;
    align-items: center;
    padding-bottom: 20px;

    :global(.search-field) {
      min-width: 240px;
      width: 480px;
    }

    :global(.search-button) {
      margin: 0px 24px;
      white-space: nowrap;
    }
  }

  .result-section {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 16px;

    :global(.card) {
      width: 500px;
      min-width: 300px;
      overflow: hidden;
    }

    .card-body {
      display: flex;
      width: 100%;
      border-radius: inherit;

      .card-content {
        display: flex;
        justify-content: space-between;
        align-items: start;
        flex-direction: column;
        width: 100%;
        min-width: 200px;
        padding: 8px;

        .title-section {
          display: flex;
          align-items: center;
          justify-content: space-between;
          gap: 8px;
          width: 100%;

          .title {
            white-space: nowrap;
            text-overflow: ellipsis;
            overflow: hidden;
          }
        }

        .information {
          display: flex;
          justify-content: end;
          align-items: end;
          width: 100%;
        }

        .lang-grid {
          display: grid;
          grid-template-columns: auto auto auto;

          gap: 4px;
          align-items: end;
          justify-content: end;
        }
      }
    }
  }
</style>
