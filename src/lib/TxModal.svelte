<script lang="ts">
  import { writable } from 'svelte/store'
  import type { Writable } from 'svelte/store'
  import { invoke } from '@tauri-apps/api/tauri'
  import Button from '../lib/Button.svelte'
  import DatePicker from '../lib/DatePicker.svelte'
  import { slide } from 'svelte/transition'
  import Modal from '../lib/Modal.svelte'
  import type { Transaction } from './transactions'
  import { refresh, popup } from '../lib/general'
  import NumericInput from './NumericInput.svelte'
  export let visible = false
  function cancel() {
    visible = false
  }
  function numStr(str: string) {
    if (str === '') return '0'
    else return str
  }

  function save() {
    validate(tx, false)
    if (hasErrors) return
    let fixedTx: Transaction
    if (kind === 'Trade') {
      fixedTx = {
        kind,
        date: tx.date.getTime(),
        note: tx.note,
        hash: tx.hash,
        sent_amount: numStr(tx.sent_amount),
        sent_asset: tx.sent_asset,
        sent_wallet: tx.sent_wallet,
        recv_amount: numStr(tx.recv_amount),
        recv_asset: tx.recv_asset,
        recv_wallet: tx.recv_wallet,
        fee_amount: numStr(tx.fee_amount),
        fee_asset: tx.fee_asset,
        cost: numStr(tx.cost),
      }
    } else if (kind === 'Deposit' || kind === 'Income') {
      fixedTx = {
        kind,
        date: tx.date.getTime(),
        note: tx.note,
        hash: tx.hash,
        sent_amount: numStr(''),
        sent_asset: '',
        sent_wallet: '',
        recv_amount: numStr(tx.recv_amount),
        recv_asset: tx.recv_asset,
        recv_wallet: tx.recv_wallet,
        fee_amount: numStr(''),
        fee_asset: '',
        cost: numStr(tx.cost),
      }
    } else {
      popup('Unsupported tx type: ' + kind)
      return
    }
    console.log('fixedTx', fixedTx)
    invoke('add_transaction', { json: JSON.stringify(fixedTx) })
      .then(() => {
        visible = false
        refresh()
      })
      .catch(popup)
  }
  let kind = 'Trade'
  $: enabledFields = getEnabledFields(kind)
  function getEnabledFields(kind: string) {
    let fields = {
      sent: true,
      recv: true,
      fee: false,
    }
    if (kind === 'Trade') {
      fields.fee = true
    }
    if (kind === 'Deposit' || kind === 'Income') {
      fields.sent = false
    }
    if (kind === 'Withdrawal') {
      fields.recv = false
    }
    return fields
  }
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
    if (!enabledFields.sent || tx.sent_amount) errors.delete('sent_amount')
    if (!enabledFields.sent || tx.sent_asset) errors.delete('sent_asset')
    if (!enabledFields.sent || tx.sent_wallet) errors.delete('sent_wallet')
    if (!enabledFields.recv || tx.recv_amount) errors.delete('recv_amount')
    if (!enabledFields.recv || tx.recv_asset) errors.delete('recv_asset')
    if (!enabledFields.recv || tx.recv_wallet) errors.delete('recv_wallet')
    errors = errors
    hasErrors = !!errors.size || invalidDate
  }
  $: validate(tx, true)
  let invalidDate: boolean
</script>

<Modal bind:visible>
  <h2>Add transaction</h2>
  <form on:submit|preventDefault={save} class="googoogaga">
    <div class="row">
      <p>Type</p>
      <select bind:value={kind}>
        <option value="Trade">Trade</option>
        <option value="Transfer">Transfer</option>
        <option value="Deposit">Deposit</option>
        <option value="Income">Income</option>
        <option value="Withdrawal">Withdrawal</option>
      </select>
    </div>
    <div class="row">
      <p>Date</p>
      <DatePicker bind:value={tx.date} bind:invalid={invalidDate} />
    </div>
    <div class="row">
      <div class="sent">
        {#if enabledFields.sent}
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
            <NumericInput
              bind:value={tx.sent_amount}
              invalid={errors.has('sent_amount')}
              noLeftBorder
              placeholder="Amount" />
          </div>
        {/if}
      </div>
      <div class="received">
        {#if enabledFields.recv}
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
            <input
              type="text"
              class="asset"
              class:invalid={errors.has('recv_asset')}
              bind:value={tx.recv_asset}
              placeholder="Asset" />
            <NumericInput
              bind:value={tx.recv_amount}
              invalid={errors.has('recv_amount')}
              noLeftBorder
              placeholder="Amount" />
          </div>
        {/if}
      </div>
    </div>
    <h4>Optional Details</h4>
    {#if enabledFields.fee}
      <div class="row fee" transition:slide|local>
        <p>Fee</p>
        <input type="text" class="asset" bind:value={tx.fee_asset} placeholder="Asset" />
        <NumericInput bind:value={tx.fee_amount} noLeftBorder placeholder="Amount" />
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
      <Button type="submit">Add</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  .googoogaga
    width: 550px
    max-width: 100%
  .sent, .received
    background-color: #eff2f5
    border: 1px solid #c6cddd
    padding: 10px
    width: 0px
    flex-grow: 1
    // :last-child
    //   margin-bottom: 0px
  .sent
    border-radius: 7px 0px 0px 7px
  .received
    border-radius: 0px 7px 7px 0px
    border-left: none
  .row
    display: flex
    margin-bottom: 10px
  .fee
    width: 320px
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
    padding: 4px 6px
    width: 100%
    margin: 0px
    font-family: inherit
    font-size: 12px
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
  .asset:focus
    z-index: 1 // outline fix
  .asset
    min-width: 60px
    width: 0px
    border-right: none
    border-top-right-radius: 0px
    border-bottom-right-radius: 0px
  .bottom
    display: grid
    grid-auto-flow: column
    grid-gap: 10px
    margin-top: 18px
    width: min-content
    margin-left: auto
</style>
