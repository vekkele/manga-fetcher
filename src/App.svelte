<script lang="ts">
  import router from "./router/router";
  import type { Route } from "./router/router";
  import { onDestroy } from "svelte";

  let route: Route;

  const darkMedia = window.matchMedia("(prefers-color-scheme: dark)");
  let dark = darkMedia.matches;

  function onDarkChange(e: MediaQueryListEvent) {
    dark = e.matches;
  }

  darkMedia.addEventListener("change", onDarkChange);
  onDestroy(() => {
    darkMedia.removeEventListener("change", onDarkChange);
  });

  router.subscribe((r) => {
    console.log({ r });
    route = r;
  });

  $: component = router.routes[route];
</script>

<svelte:head>
  <!-- SMUI Styles -->
  {#if dark}
    <link rel="stylesheet" href="/smui-dark.css" />
  {:else}
    <link rel="stylesheet" href="/smui.css" />
  {/if}
</svelte:head>

<svelte:body class="light-theme" />
<main>
  <svelte:component this={component} />
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }
</style>
