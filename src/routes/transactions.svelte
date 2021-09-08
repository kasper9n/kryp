<script context="module">
  // persist variable by defining in in module
  import { writable } from 'svelte/store'
  let view = writable(0)
</script>

<script lang="ts">
  import TxTable from '../lib/TxTable.svelte'
  import TxList from '../lib/TxList.svelte'
  import TransactionModal from '../modals/Transaction.svelte'
  import Button from '../lib/Button.svelte'
  let showAdd = false
  function add() {
    showAdd = true
  }
</script>

<svelte:head>
  <title>Transactions - Kryp</title>
</svelte:head>

<div class="page">
  <div class="toolbar">
    <Button group={['List', 'Table']} bind:selected={$view} />
    <div class="spacer" />
    <Button on:click={add}>Add</Button>
  </div>
  {#if $view === 0}
    <TxList />
  {:else if $view === 1}
    <TxTable />
  {/if}
</div>
<TransactionModal bind:visible={showAdd} />

<style lang="sass">
  $accent: #3061F6
  $border: #c6cddd
  .page
    padding: 20px
    max-width: 1000px
    margin: auto
  .toolbar
    padding: 10px 0px
    display: flex
    align-items: center
  .spacer
    width: 10px
    height: 10px
</style>
