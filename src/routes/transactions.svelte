<script context="module">
  import { writable } from 'svelte/store'
  let view = writable(0)
</script>

<script lang="ts">
  import TxTable from '$lib/TxTable.svelte'
  import TxList from '$lib/TxList.svelte'
  import TransactionModal from '$modals/Transaction.svelte'
  import ImportModal from '$modals/Import.svelte'
  import Button from '$lib/Button.svelte'
  let showAdd = false
  let showImport = false
</script>

<div class="page">
  <div class="toolbar">
    <Button group={['List', 'Table']} bind:selected={$view} />
    <div style="padding: 6px;" />
    <Button on:click={() => (showAdd = true)}>Add</Button>
    <div style="padding: 6px;" />
    <Button secondary on:click={() => (showImport = true)}>Import</Button>
  </div>
  {#if $view === 0}
    <TxList />
  {:else if $view === 1}
    <TxTable />
  {/if}
</div>

{#if showAdd}
  <TransactionModal on:close={() => (showAdd = false)} />
{/if}
{#if showImport}
  <ImportModal on:close={() => (showImport = false)} />
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
