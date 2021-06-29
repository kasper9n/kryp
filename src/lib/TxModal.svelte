<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import Button from '../lib/Button.svelte'
  import DatePicker from '../lib/DatePicker.svelte'
  import { slide } from 'svelte/transition'
  import Modal from '../lib/Modal.svelte'
  import type { Transaction } from './transactions'
  import { refresh, popup } from '../lib/general'
  export let visible = false
  function cancel() {
    visible = false
  }

  function save() {
    validate(tx, false)
    if (hasErrors) return
    if (kind === 'Trade') {
      let fixedTx: Transaction = {
        kind,
        date: tx.date.getTime(),
        note: tx.note,
        hash: tx.hash,
        sent_amount: tx.sent_amount === '' ? '0' : tx.sent_amount,
        sent_asset: tx.sent_asset,
        sent_wallet: tx.sent_wallet,
        recv_amount: tx.recv_amount === '' ? '0' : tx.recv_amount,
        recv_asset: tx.recv_asset,
        recv_wallet: tx.recv_wallet,
        fee_amount: tx.fee_amount === '' ? '0' : tx.fee_amount,
        fee_asset: tx.fee_asset,
        cost: tx.cost === '' ? '0' : tx.cost,
      }
      invoke('add_transaction', { json: JSON.stringify(fixedTx) })
        .then(() => {
          visible = false
          refresh()
        })
        .catch(popup)
    }
  }
  let kind = 'Trade'
  function getDefault() {
    return {
      date: new Date(),
      note: '',
      hash: '',
      sent_amount: '',
      sent_asset: '',
      sent_wallet: '',
      recv_amount: '',
      recv_asset: '',
      recv_wallet: '',
      fee_amount: '',
      fee_asset: '',
      cost: '',
    }
  }
  let tx = getDefault()
  function open() {
    tx = getDefault()
    errors.clear()
  }
  $: if (visible) open()

  let errors: Set<string> = new Set()
  let hasErrors: boolean
  function validate(tx: any, onlyRemove = false) {
    if (!onlyRemove) {
      errors.add('sent_amount')
      errors.add('sent_asset')
      errors.add('sent_wallet')
      errors.add('recv_amount')
      errors.add('recv_asset')
      errors.add('recv_wallet')
    }
    if (tx.sent_amount !== '') errors.delete('sent_amount')
    if (tx.sent_asset !== '') errors.delete('sent_asset')
    if (tx.sent_wallet !== '') errors.delete('sent_wallet')
    if (tx.recv_amount !== '') errors.delete('recv_amount')
    if (tx.recv_asset !== '') errors.delete('recv_asset')
    if (tx.recv_wallet !== '') errors.delete('recv_wallet')
    errors = errors
    hasErrors = !!errors.size || invalidDate
  }
  $: validate(tx, true)
  let invalidDate: boolean
</script>

<Modal bind:visible>
  <h2>Add transaction</h2>
  <form on:submit|preventDefault={save}>
    <div class="row">
      <p>Type</p>
      <select bind:value={kind}>
        <option value="Trade">Trade</option>
        <option value="Transfer">Transfer</option>
        <option value="Deposit">Deposit</option>
        <option value="Withdrawal">Withdrawal</option>
      </select>
    </div>
    <div class="row">
      <p>Date</p>
      <DatePicker bind:value={tx.date} bind:invalid={invalidDate} />
    </div>
    <div class="row">
      <div class="sent">
        {#if kind !== 'Deposit'}
          <h4>Sent</h4>
          <div class="row">
            <input
              type="text"
              class="wallet"
              class:invalid={errors.has('sent_wallet')}
              bind:value={tx.sent_wallet}
              placeholder="Wallet" />
          </div>
          <div class="row">
            <input
              type="text"
              class="asset"
              class:invalid={errors.has('sent_asset')}
              bind:value={tx.sent_asset}
              placeholder="Asset" />
            <input
              type="number"
              class="amount"
              class:invalid={errors.has('sent_amount')}
              bind:value={tx.sent_amount}
              placeholder="Amount" />
          </div>
        {/if}
      </div>
      <div class="received">
        {#if kind !== 'Withdrawal'}
          <h4>Received</h4>
          <div class="row">
            <input
              type="text"
              class="wallet"
              class:invalid={errors.has('recv_wallet')}
              bind:value={tx.recv_wallet}
              placeholder="Wallet" />
          </div>
          <div class="row">
            {#if kind === 'Transfer'}
              <input
                type="text"
                class="asset"
                bind:value={tx.sent_asset}
                disabled={kind === 'Transfer'}
                placeholder="Asset" />
            {:else}
              <input
                type="text"
                class="asset"
                class:invalid={errors.has('recv_asset')}
                bind:value={tx.recv_asset}
                disabled={kind === 'Transfer'}
                placeholder="Asset" />
            {/if}
            <input
              type="number"
              class="amount"
              class:invalid={errors.has('recv_amount')}
              bind:value={tx.recv_amount}
              placeholder="Amount" />
          </div>
        {/if}
      </div>
    </div>
    <h4>Optional Details</h4>
    {#if kind === 'Trade'}
      <div class="row fee" transition:slide|local>
        <p>Fee</p>
        <input type="text" class="asset" bind:value={tx.fee_asset} placeholder="Asset" />
        <input type="text" class="amount" bind:value={tx.fee_amount} placeholder="Amount" />
      </div>
    {/if}
    <div class="row">
      <p>Tx Hash</p>
      <input type="text" class="note" bind:value={tx.hash} />
    </div>
    <div class="row">
      <p>Note</p>
      <textarea class="note" bind:value={tx.note} />
    </div>
    <div class="bottom">
      <Button secondary on:click={cancel}>Cancel</Button>
      <div class="spacer" />
      <Button type="submit">Add</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  .sent, .received
    background-color: #eff2f5
    border: 1px solid #c6cddd
    padding: 10px
    width: 260px
  .sent
    border-radius: 7px 0px 0px 7px
  .received
    border-radius: 0px 7px 7px 0px
    border-left: none
  .row
    display: flex
    padding-bottom: 10px
  .fee
    width: 270px
    max-width: 100%
  h4
    margin-bottom: 10px
  select
    margin: 4px 0px
  p
    display: inline-block
    min-width: 65px
    margin: 0px
    margin-top: 4px
    font-size: 14px
    cursor: default
  input, textarea
    min-width: 0px
    padding: 4px 8px
    width: 100%
    margin: 0px
    font-family: inherit
    font-size: 13px
    border: 1px solid #c6cddd
    border-radius: 3px
  .invalid
    border: 1px solid rgba(#f92f72, 0.5)
    background-color: #fff0f5
  textarea
    resize: vertical
  :disabled
    background-color: #e5e5e6
    opacity: 0.7
  .wallet
    display: block
    width: 100%
  .asset:focus, .amount:focus
    z-index: 1 // outline fix
  .asset
    width: 30%
    border-right: none
    border-top-right-radius: 0px
    border-bottom-right-radius: 0px
  .amount
    width: 70%
    border-top-left-radius: 0px
    border-bottom-left-radius: 0px
  .bottom
    display: flex
    justify-content: flex-end
  .spacer
    width: 10px
    height: 10px
</style>
