<script lang="ts">
  import { onDestroy } from "svelte";
  import "ress/dist/ress.min.css";
  import "../theme/smui-dark.css";

  const darkMedia = window.matchMedia("(prefers-color-scheme: dark)");
  let dark = darkMedia.matches; //TODO: Handle light/dark theme

  function onDarkChange(e: MediaQueryListEvent) {
    dark = e.matches;
  }

  darkMedia.addEventListener("change", onDarkChange);
  onDestroy(() => {
    darkMedia.removeEventListener("change", onDarkChange);
  });
</script>

<main>
  <slot />
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
  }
</style>
