<script lang="ts">
  import SvelteMarkdown from "svelte-markdown";
  import type { Props as SvelteMarkdownProps } from "svelte-markdown";
  import LinkMarkdown from "./LinkMarkdown.svelte";

  export let source: SvelteMarkdownProps["source"];
</script>

<SvelteMarkdown {source} renderers={{ link: LinkMarkdown }} />
