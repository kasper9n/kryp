<script lang="ts">
  import { runCmd } from '$lib/general'
  import Modal from '$lib/Modal.svelte'
  import Button from '$lib/Button.svelte'
  import { createEventDispatcher } from 'svelte'
  import TextInput from '$lib/TextInput.svelte'

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
    <TextInput bind:value={baseCurrency} />
    <div class="mt-4 grid grid-flow-col justify-end gap-2">
      <Button secondary on:click={() => dispatch('close')}>Cancel</Button>
      <Button type="submit">Create</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  p
    font-size: 13px
    margin-bottom: 5px
</style>
