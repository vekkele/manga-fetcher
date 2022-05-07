<script>
  import Button from "@smui/button";
  import router, { Route } from "src/router/router";

  function toSearch() {
    router.navigate(Route.Search);
  }
</script>

<h1>Title info</h1>
<Button on:click={toSearch}>To Search</Button>
