<script lang="ts">
  import Info from "$lib/icons/Info.svelte";
</script>

<div class="fixed top-4 right-4">
  <label for="info-modal" class="btn btn-circle text-white">
    <Info />
  </label>
</div>

<input type="checkbox" id="info-modal" class="modal-toggle" />
<label for="info-modal" class="modal cursor-pointer">
  <label for="" class="modal-box relative">
    <p class="py-4">
      All manga provided by
      <a href="https://mangadex.org/" class="link">MangaDex.org</a>
    </p>
    <div class="modal-action">
      <label for="info-modal" class="btn">Understood!</label>
    </div>
  </label>
</label>
