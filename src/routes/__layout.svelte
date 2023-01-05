<script lang="ts">
  import InfoModal from "$lib/components/InfoModal.svelte";
  import { onDestroy } from "svelte";
  import "../app.css";

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
  <InfoModal />
</main>
