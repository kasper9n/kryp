<script lang="ts">
  import TxTable from '$lib/TxTable.svelte'
  import TxList from '$lib/TxList.svelte'
  import TransactionModal from '$lib/modals/Transaction.svelte'
  import Button from '$lib/Button.svelte'
  import { createEventDispatcher } from 'svelte'
  import { tags, Transaction } from '$lib/transactions'
  import { runCmd } from '$lib/general'
  import ButtonGroup from '$lib/ButtonGroup.svelte'
  import InlinePopup from '$lib/InlinePopup.svelte'

  let view = 0

  let showAdd = false
  const dispatch = createEventDispatcher()
  function closeModal() {
    showAdd = false
    reload()
  }

  let search = {
    tags: [] as string[],
    asset: '',
  }

  let transactions: Promise<Transaction[]> = runCmd('get_transactions', { search })
  function reload() {
    transactions = runCmd('get_transactions', { search })
  }
  $: transactions = runCmd('get_transactions', { search })
</script>

<div class="flex h-0 flex-grow flex-col">
  <div class="my-2 flex flex-shrink-0 items-center px-4">
    <ButtonGroup values={['List', 'Table']} bind:selected={view} />
    <div class="p-1.5" />
    <Button on:click={() => (showAdd = true)}>Add</Button>
    <div class="p-1.5" />
    <Button secondary on:click={() => dispatch('import')}>Import</Button>
  </div>
  <div class="my-2 flex flex-shrink-0 items-center px-4">
    <InlinePopup let:toggle>
      <button class="rounded border border-gray-300 bg-white py-1.5 px-3" on:click={toggle}
        >Type</button
      >
      <div slot="popup" class="rounded border bg-white px-4 py-2">
        {#each tags as tag}
          <label class="flex select-none items-center">
            <input type="checkbox" bind:group={search.tags} value={tag.name} />
            <span class="ml-1">{tag.name}</span>
          </label>
        {/each}
      </div>
    </InlinePopup>
    <div class="p-1.5" />
    <div class="relative">
      <input
        class="w-28 rounded border-gray-300 py-1.5 px-3"
        type="text"
        placeholder="Asset"
        bind:value={search.asset}
      />
    </div>
  </div>
  <div class="h-0 flex-grow">
    {#await transactions then transactions}
      {#if view === 0}
        <TxList {transactions} />
      {:else if view === 1}
        <TxTable {transactions} />
      {/if}
    {/await}
  </div>
</div>

{#if showAdd}
  <TransactionModal on:close={closeModal} />
{/if}
