<script lang="ts">
  import Button from '$lib/Button.svelte'
  import { runCmd } from '$lib/general'
  import type { Transaction } from '$lib/transactions'
  import { formatDateTime } from '$lib/transactions'
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import { router } from 'tinro'

  type ImportTransaction = {
    transaction: Transaction
    cost: string | null
    error: string | null
  }
  type ImportData = {
    transactions: ImportTransaction[]
    has_errors: boolean
  }
  let importData: ImportData | null = null

  async function importFile() {
    const newImportData = await runCmd('start_import')
    importData = newImportData
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

  async function continueImport() {
    await runCmd('continue_import')
    router.goto('/transactions')
  }
  async function cancel() {
    await runCmd('cancel_import')
    importData = null
    status = null
  }

  onDestroy(async () => {
    const unlisten = await statusUnlisten
    unlisten()
    cancel()
  })
</script>

<h1 class="center">Import</h1>
{#if importData}
  {#if importData.has_errors}
    <p class="center red">
      Could not calculate the cost of the following transactions. Please enter them manually
    </p>
  {:else}
    <p class="center">Successful scan. Continue to import?</p>
    <div class="button-row">
      <Button secondary on:click={cancel}>Cancel</Button>
      <div class="space" />
      <Button disabled={importData.has_errors} on:click={continueImport}>Continue</Button>
    </div>
  {/if}
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
        {#if importData.has_errors}
          <th>Error</th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each importData.transactions as tx, i}
        {#if !tx.cost || !importData.has_errors}
          <tr class:odd={i % 2 === 0}>
            <td
              class:blue={tx.transaction.type === 'Trade'}
              class:purple={tx.transaction.type === 'Transfer'}
              class:green={tx.transaction.type === 'Deposit'}
              class:red={tx.transaction.type === 'Withdrawal'}
            >
              {tx.transaction.tag}
            </td>
            {#if tx.transaction.type === 'Trade'}
              <td class="sent amount">{tx.transaction.sent_amount}</td>
              <td class="sent asset">{tx.transaction.sent_asset}</td>
              <td class="sent wallet">{tx.transaction.sent_wallet}</td>
              <td class="recv amount">{tx.transaction.recv_amount}</td>
              <td class="recv asset">{tx.transaction.recv_asset}</td>
              <td class="recv wallet">{tx.transaction.recv_wallet}</td>
              <td class="fee amount">
                {#if tx.transaction.fee_asset !== '' && tx.transaction.fee_amount !== '0'}
                  {tx.transaction.fee_amount}
                {/if}
              </td>
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
            <td class="note">{tx.transaction.note}</td>
            <td class="hash">{tx.transaction.hash}</td>
            <td>{formatDateTime(new Date(tx.transaction.date))}</td>
            {#if importData.has_errors}
              <td class="red">
                {#if tx.error}
                  {tx.error}
                {/if}
              </td>
            {/if}
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>
{:else if status}
  <p class="center">Scanned {status.index} transactions</p>
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
    margin-bottom: 36px
    border-spacing: 0px
    border-collapse: collapse
    cursor: default
    font-size: 13px
  td
    padding: 5px
    border: 1px solid #e7e8e8
    overflow: hidden
    text-overflow: ellipsis
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
  .note
    max-width: 180px
  .hash
    max-width: 100px
  .red
    color: #f92f72
  .green
    color: #25b670
  .blue
    color: #2ea8fa
  .purple
    color: #b853ee
  .button-row
    display: flex
    margin: 18px auto
    justify-content: center
  .space
    padding: 6px
</style>
