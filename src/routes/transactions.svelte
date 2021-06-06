<script lang="ts">
  import TxList from '../lib/TxList.svelte'
  import TxTable from '../lib/TxTable.svelte'
  let view = 'list'
</script>

<svelte:head>
  <title>Transactions - Kryp</title>
</svelte:head>

<div class="page">
  <div class="toolbar">
    <div class="view">
      <button class:enabled={view === 'list'} on:mousedown={() => (view = 'list')}>List</button>
      <button class:enabled={view === 'table'} on:mousedown={() => (view = 'table')}>Table</button>
    </div>
  </div>
  {#if view === 'list'}
    <TxList />
  {:else if view === 'table'}
    <TxTable />
  {/if}
</div>

<style lang="sass">
  .page
    margin: 20px
  .toolbar
    padding: 10px 0px
  .view
    display: inline-flex
    background-color: #e6e6e6
    border-radius: 5px
    button
      cursor: pointer
      border: none
      margin: 0px
      height: 32px
      padding: 0px 16px
      font-size: 13px
      color: #000000
      background-color: transparent
      outline: none
      border-radius: 5px
    .enabled
      color: #ffffff
      background-color: #EB5B77
</style>
