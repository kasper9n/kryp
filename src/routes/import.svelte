<script lang="ts">
  import Button from '$lib/Button.svelte'
  import { runCmd } from '$lib/general'
  import Modal from '$lib/Modal.svelte'
  import type { Transaction } from '$lib/transactions'
  import { formatDateTime } from '$lib/transactions'
  import { event } from '@tauri-apps/api'
  import { createEventDispatcher, onDestroy } from 'svelte'

  const dispatch = createEventDispatcher()

  type ImportTransaction = {
    transaction: Transaction
    cost: string | null
  }
  type ImportData = {
    transactions: ImportTransaction[]
  }
  let importData: ImportData | null = null
  $: console.log('importData', importData)

  async function importFile() {
    const newImportData = await runCmd('import')
    importData = newImportData
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

  type ImportStatus = {
    index: number
  }
  let status: ImportStatus | null = null
  const statusUnlisten = event.listen('importStatus', (e) => {
    if (e.payload) {
      status = e.payload as ImportStatus
    }
  })
  onDestroy(async () => {
    ;(await statusUnlisten)()
  })
</script>

<h1 class="center">Import</h1>
{#if importData}
  <p class="center red">
    Could not calculate the cost of the following transactions. Please enter them manually
  </p>
  <table>
    <thead>
      <tr>
        <th>Type</th>
        <th>Sent</th>
        <th>Cur.</th>
        <th>Wallet</th>
        <th>Received</th>
        <th>Cur.</th>
        <th>Wallet</th>
        <th>Fee</th>
        <th>Cur.</th>
        <th>Note</th>
        <th>Hash</th>
        <th>Date</th>
      </tr>
    </thead>
    <tbody>
      {#each importData.transactions as tx, i}
        {#if !tx.cost}
          <tr class:odd={i % 2 === 0}>
            <td>{tx.transaction.tag}</td>
            {#if tx.transaction.type === 'Trade'}
              <td class="sent amount">{tx.transaction.sent_amount}</td>
              <td class="sent asset">{tx.transaction.sent_asset}</td>
              <td class="sent wallet">{tx.transaction.sent_wallet}</td>
              <td class="recv amount">{tx.transaction.recv_amount}</td>
              <td class="recv asset">{tx.transaction.recv_asset}</td>
              <td class="recv wallet">{tx.transaction.recv_wallet}</td>
              <td class="fee amount">{tx.transaction.fee_amount}</td>
              <td class="fee asset">{tx.transaction.fee_asset}</td>
            {:else if tx.transaction.type === 'Transfer'}
              <td class="sent amount">{tx.transaction.sent_amount}</td>
              <td class="sent asset">{tx.transaction.sent_asset}</td>
              <td class="sent wallet">{tx.transaction.sent_wallet}</td>
              <td class="recv amount">{tx.transaction.recv_amount}</td>
              <td class="recv asset">{tx.transaction.recv_asset}</td>
              <td class="recv wallet">{tx.transaction.recv_wallet}</td>
              <td class="fee amount" />
              <td class="fee asset" />
            {:else if tx.transaction.type === 'Deposit'}
              <td class="sent amount" />
              <td class="sent asset" />
              <td class="sent wallet" />
              <td class="recv amount">{tx.transaction.amount}</td>
              <td class="recv asset">{tx.transaction.asset}</td>
              <td class="recv wallet">{tx.transaction.wallet}</td>
              <td class="fee amount" />
              <td class="fee asset" />
            {:else if tx.transaction.type === 'Withdrawal'}
              <td class="sent amount">{tx.transaction.amount}</td>
              <td class="sent asset">{tx.transaction.asset}</td>
              <td class="sent wallet">{tx.transaction.wallet}</td>
              <td class="recv amount" />
              <td class="recv asset" />
              <td class="recv wallet" />
              <td class="fee amount" />
              <td class="fee asset" />
            {/if}
            <td>{tx.transaction.note}</td>
            <td>{tx.transaction.hash}</td>
            <td>{formatDateTime(new Date(tx.transaction.date))}</td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>
{:else if status}
  <p>Scanned {status.index} transactions</p>
{:else}
  <p class="center">Import a custom CSV or TSV file</p>
  <div class="center">
    <Button on:click={importFile}>Import</Button>
  </div>
{/if}

<style lang="sass">
  .center
    display: flex
    align-items: center
    justify-content: center
  table
    margin: auto
    border-spacing: 0px
    border-collapse: collapse
    cursor: default
    font-size: 14px
  td
    padding: 6px 10px
    border: 1px solid #e7e8e8
    &.amount
      text-align: right
  thead
    th
      text-align: center
      padding-bottom: 4px
  tbody
    background-color: #f6f6f9
  .odd
    background-color: #ffffff
  .red
    color: #f92f72
  .green
    color: #25b670
</style>
