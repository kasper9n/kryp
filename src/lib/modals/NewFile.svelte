<script lang="ts">
  import { focus, runCmd } from '$lib/general'
  import Modal from '$lib/Modal.svelte'
  import Button from '$lib/Button.svelte'
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()
  let baseCurrency = 'USD'

  function keydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault()
      dispatch('close')
    }
  }
  async function create() {
    await runCmd('new_file', {
      baseCurrency: baseCurrency,
    })
    dispatch('close')
  }
</script>

<Modal width="340px" on:close on:keydown={keydown}>
  <h2>New File</h2>
  <form on:submit|preventDefault={create}>
    <p>Base currency</p>
    <input type="text" bind:value={baseCurrency} use:focus />
    <div class="bottom">
      <Button secondary on:click={() => dispatch('close')}>Cancel</Button>
      <Button type="submit">Create</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  p
    font-size: 13px
    margin-bottom: 5px
  input
    min-width: 0px
    padding: 4px 8px
    width: 100%
    box-sizing: border-box
    margin: 0px
    font-family: inherit
    font-size: 13px
    border: 1px solid #c6cddd
    border-radius: 3px
  .bottom
    display: grid
    grid-auto-flow: column
    grid-gap: 10px
    margin-top: 18px
    width: min-content
    margin-left: auto
</style>
