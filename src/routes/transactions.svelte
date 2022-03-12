<script lang="ts">
  import TxTable from '$lib/TxTable.svelte'
  import TxList from '$lib/TxList.svelte'
  import TransactionModal from '$lib/modals/Transaction.svelte'
  import Button from '$lib/Button.svelte'
  import { createEventDispatcher } from 'svelte'
  let view = 0

  let showAdd = false
  const dispatch = createEventDispatcher()
</script>

<div class="page">
  <div class="toolbar">
    <Button group={['List', 'Table']} bind:selected={view} />
    <div style="padding: 6px;" />
    <Button on:click={() => (showAdd = true)}>Add</Button>
    <div style="padding: 6px;" />
    <Button secondary on:click={() => dispatch('import')}>Import</Button>
  </div>
  {#if view === 0}
    <TxList />
  {:else if view === 1}
    <TxTable />
  {/if}
</div>

{#if showAdd}
  <TransactionModal on:close={() => (showAdd = false)} />
{/if}

<style lang="sass">
  $accent: #3061F6
  $border: #c6cddd
  .page
    padding: 20px
    margin: auto
  .toolbar
    padding: 10px 0px
    display: flex
    align-items: center
</style>
