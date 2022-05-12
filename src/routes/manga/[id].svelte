<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";

  $: id = $page.params["id"];

  function back() {
    goto("/");
  }
</script>

<button class="btn" on:click={back}>To Search</button>

{#if !id}
  <h1>Invalid selectd manga</h1>
{:else}
  <h1>Title info {id}</h1>
{/if}
