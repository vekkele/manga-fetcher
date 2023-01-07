<script lang="ts">
  export let id: string;
</script>

<slot name="trigger" {id} />

<input type="checkbox" {id} class="modal-toggle" />
<label for={id} class="modal cursor-pointer">
  <label for="" class="modal-box relative">
    <slot />

    {#if $$slots.action}
      <div class="modal-action">
        <slot name="action" {id} />
      </div>
    {/if}
  </label>
</label>
