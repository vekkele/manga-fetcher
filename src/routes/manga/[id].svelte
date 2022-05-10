<script lang="ts">
  import Button from "@smui/button";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";

  function back() {
    goto("/");
  }
</script>

<Button on:click={back}>To Search</Button>

{#if !back}
  <h1>Invalid selectd manga</h1>
{:else}
  <h1>Title info {JSON.stringify($page)}</h1>
{/if}
