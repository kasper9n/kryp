<script lang="ts">
  import Button from '../lib/Button.svelte'
  export let visible = false
  function close() {
    visible = false
  }
</script>

{#if visible}
  <div class="modal-bg" on:click|self={close}>
    <div class="box">
      <slot />
    </div>
  </div>
{/if}

<style lang="sass">
  .modal-bg
    position: fixed
    width: 100%
    height: 100%
    top: 0
    left: 0
    display: flex
    align-items: center
    justify-content: center
    padding: 20px
    box-sizing: border-box
    background-color: rgba(#000000, 0.5)
    .box
      background-color: #f8f9fc
      min-width: 300px
      max-width: 100%
      max-height: 100%
      padding: 18px
      box-sizing: border-box
      border-radius: 7px
      box-shadow: 0px 0px 30px 0px rgba(#000000, 0.5)
      overflow: auto
</style>
