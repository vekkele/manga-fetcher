<script lang="ts">
  export let status: string;
</script>

<caption class={`status ${status}`}>{status}</caption>

<style lang="scss">
  @use "@material/theme/color-palette" as scheme;

  .status {
    padding: 4px;
    border-radius: 8px;
    color: white;
    background-color: scheme.$grey-700;
  }

  .ongoing {
    background-color: scheme.$green-700;
  }

  .completed {
    background-color: scheme.$cyan-700;
  }

  .cancelled {
    background-color: scheme.$red-700;
  }

  .hiatus {
    background-color: scheme.$orange-700;
  }
</style>
