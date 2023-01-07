<script lang="ts">
  import Info from "$lib/icons/Info.svelte";
  import Modal from "./Modal.svelte";
</script>

<Modal id="mangadex-credit-modal">
  <div slot="trigger" let:id class="fixed top-4 right-4">
    <label for={id} class="btn btn-circle text-white">
      <Info />
    </label>
  </div>

  <p class="py-4">
    All manga provided by
    <a href="https://mangadex.org/" class="link">MangaDex.org</a>
  </p>

  <label slot="action" let:id for={id} class="btn">Understood!</label>
</Modal>
