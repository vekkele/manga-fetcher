<script lang="ts">
  export let status: string;
</script>

<span class={`badge text-white ${status}`}>{status}</span>

<style lang="postcss">
  .ongoing {
    @apply bg-green-700;
  }

  .completed {
    @apply bg-cyan-700;
  }

  .cancelled {
    @apply bg-red-700;
  }

  .hiatus {
    @apply bg-orange-700;
  }
</style>
