<script lang="ts">
  import NumericInput from '$lib/NumericInput.svelte'
  import StringQuantity from '$lib/StringQuantity.svelte'
  import { formatDateTime } from '$lib/transactions'
  import type { Transaction } from '$lib/transactions'
  import type { ImportData } from './index.svelte'

  export let transaction: Transaction
  export let cost: string | null
  export let error: string | null
  export let i: number
  export let importData: ImportData

  let amount: string
  let asset: string
</script>

{#if !cost}
  <tr class:odd={i % 2 === 0}>
    <td
      class:blue={transaction.type === 'Trade'}
      class:purple={transaction.type === 'Transfer'}
      class:green={transaction.type === 'Deposit'}
      class:red={transaction.type === 'Withdrawal'}
    >
      {transaction.tag}
    </td>
    {#if transaction.type === 'Trade'}
      <td class="sent amount">{transaction.sent_amount}</td>
      <td class="sent asset">{transaction.sent_asset}</td>
      <td class="sent wallet">{transaction.sent_wallet}</td>
      <td class="recv amount">{transaction.recv_amount}</td>
      <td class="recv asset">{transaction.recv_asset}</td>
      <td class="recv wallet">{transaction.recv_wallet}</td>
      <td class="fee amount">
        {#if transaction.fee_asset !== '' && transaction.fee_amount !== '0'}
          {transaction.fee_amount}
        {/if}
      </td>
      <td class="fee asset">{transaction.fee_asset}</td>
    {:else if transaction.type === 'Transfer'}
      <td class="sent amount">{transaction.sent_amount}</td>
      <td class="sent asset">{transaction.sent_asset}</td>
      <td class="sent wallet">{transaction.sent_wallet}</td>
      <td class="recv amount">{transaction.recv_amount}</td>
      <td class="recv asset">{transaction.recv_asset}</td>
      <td class="recv wallet">{transaction.recv_wallet}</td>
      <td class="fee amount" />
      <td class="fee asset" />
    {:else if transaction.type === 'Deposit'}
      <td class="sent amount" />
      <td class="sent asset" />
      <td class="sent wallet" />
      <td class="recv amount">{transaction.amount}</td>
      <td class="recv asset">{transaction.asset}</td>
      <td class="recv wallet">{transaction.wallet}</td>
      <td class="fee amount" />
      <td class="fee asset" />
    {:else if transaction.type === 'Withdrawal'}
      <td class="sent amount">{transaction.amount}</td>
      <td class="sent asset">{transaction.asset}</td>
      <td class="sent wallet">{transaction.wallet}</td>
      <td class="recv amount" />
      <td class="recv asset" />
      <td class="recv wallet" />
      <td class="fee amount" />
      <td class="fee asset" />
    {/if}
    <td class="note">{transaction.note}</td>
    <td class="hash">{transaction.hash}</td>
    <td class="date">{formatDateTime(new Date(transaction.date))}</td>
    <td>
      <div class="flex min-w-[10rem]">
        <StringQuantity bind:quantity={transaction.manual_worth} bind:amount bind:asset />
        <NumericInput
          bind:value={amount}
          style={'border-top-right-radius: 0px; border-bottom-right-radius: 0px'}
          placeholder="Amount"
        />
        <input type="text" class="asset" bind:value={asset} placeholder="Asset" />
      </div>
    </td>
    {#if importData.has_errors}
      <td class="red">
        {#if error}
          {error}
        {/if}
      </td>
    {/if}
  </tr>
{/if}

<style lang="sass">
  input
    font-size: inherit
    width: 100%
    min-width: 0px
    padding: 4px 6px
    margin: 0px
    border: 1px solid #c6cddd
    border-radius: 3px
    outline: none
    transition: 80ms var(--ease)
    transition-property: border-color, box-shadow
    &:focus
      border-color: #0269f7
      box-shadow: 0px 0px 0px 2px rgba(#0269f7, 0.4)
  .asset:focus
    z-index: 1 // outline fix
  .asset
    min-width: 60px
    width: 0px
    margin-left: -1px
    border-top-left-radius: 0px
    border-bottom-left-radius: 0px
</style>
