<script lang="ts">
  import Button from '$lib/Button.svelte'
  import { runCmd } from '$lib/general'
  import Modal from '$lib/Modal.svelte'
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()

  async function importFile() {
    await runCmd('import')
  }

  function keydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault()
      dispatch('close')
    }
  }
  function close() {
    dispatch('close')
  }
</script>

<Modal width="460px" title="Import" on:keydown={keydown} on:close={close}>
  <div class="select-none">
    <p class="center">Import a custom CSV or TSV file</p>
    <div class="center">
      <Button on:click={importFile}>Import</Button>
    </div>
  </div>
</Modal>

<style lang="sass">
  .select-none
    user-select: none
  .center
    display: flex
    align-items: center
    justify-content: center
</style>
