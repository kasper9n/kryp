<script lang="ts">
  import { DateInput } from 'date-picker-svelte'
  import Button from '$lib/Button.svelte'
  import Modal from '$lib/Modal.svelte'
  import { tags, Transaction } from '$lib/transactions'
  import { popup, runCmd, UnreachableCaseError } from '$lib/general'
  import NumericInput from '$lib/NumericInput.svelte'
  import Dropdown from '$lib/Dropdown.svelte'
  import { createEventDispatcher, onMount } from 'svelte'

  const dispatch = createEventDispatcher()
  function close() {
    dispatch('close')
  }
  function numStr(str: string) {
    if (str === '') return '0'
    else return str
  }

  function getManualWorth() {
    if (info.manual_worth_amount === '') {
      return null
    } else {
      return info.manual_worth_amount + ' ' + info.manual_worth_asset
    }
  }

  async function save() {
    validate(info, false)
    if (hasErrors || !info.date) return

    let json: Transaction
    if (tag.type === 'Trade') {
      json = {
        type: tag.type,
        tag: tag.value,
        date: info.date.getTime(),
        note: info.note,
        hash: info.hash,
        sent_amount: numStr(info.sent_amount),
        sent_asset: info.sent_asset,
        sent_wallet: info.sent_wallet,
        recv_amount: numStr(info.recv_amount),
        recv_asset: info.recv_asset,
        recv_wallet: info.recv_wallet,
        fee_amount: numStr(info.fee_amount),
        fee_asset: info.fee_asset,
        manual_worth: getManualWorth(),
        cost: numStr(info.cost),
      }
    } else if (tag.type === 'Transfer') {
      json = {
        type: tag.type,
        tag: tag.value,
        date: info.date.getTime(),
        note: info.note,
        hash: info.hash,
        sent_amount: numStr(info.sent_amount),
        sent_asset: info.sent_asset,
        sent_wallet: info.sent_wallet,
        recv_amount: numStr(info.recv_amount),
        recv_asset: info.recv_asset,
        recv_wallet: info.recv_wallet,
        manual_worth: getManualWorth(),
        cost: numStr(info.cost),
      }
    } else if (tag.type === 'Deposit') {
      json = {
        type: tag.type,
        tag: tag.value,
        date: info.date.getTime(),
        note: info.note,
        hash: info.hash,
        amount: numStr(info.recv_amount),
        asset: info.recv_asset,
        wallet: info.recv_wallet,
        manual_worth: getManualWorth(),
        cost: numStr(info.cost),
      }
    } else if (tag.type === 'Withdrawal') {
      json = {
        type: tag.type,
        tag: tag.value,
        date: info.date.getTime(),
        note: info.note,
        hash: info.hash,
        amount: numStr(info.sent_amount),
        asset: info.sent_asset,
        wallet: info.sent_wallet,
        manual_worth: getManualWorth(),
        cost: numStr(info.cost),
      }
    } else {
      popup('Unsupported tx type: ' + tag)
      throw new UnreachableCaseError(tag.type)
    }
    console.log('Add transaction:', json)
    await runCmd('add_transaction', { ttype: json.type, json: JSON.stringify(json) })
    close()
  }
  let tag = tags[0]
  $: enabledFields = getEnabledFields(tag.type)
  function getEnabledFields(kind: string) {
    return {
      sent: kind !== 'Deposit',
      recv: kind !== 'Withdrawal',
      fee: kind === 'Trade',
    }
  }
  type Info = {
    date: Date | null
    note: string
    hash: string
    sent_amount: string
    sent_asset: string
    sent_wallet: string
    recv_amount: string
    recv_asset: string
    recv_wallet: string
    fee_amount: string
    fee_asset: string
    manual_worth_amount: string
    manual_worth_asset: string
    cost: string
  }
  function getDefault(): Info {
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
      manual_worth_amount: '',
      manual_worth_asset: '',
      cost: '',
    }
  }
  let info = getDefault()
  let showNetWorth = false
  onMount(() => {
    info = getDefault()
    showNetWorth = false
    errors.clear()
  })

  let validDate: boolean
  let errors: Set<string> = new Set()
  let hasErrors: boolean
  function validate(info: Info, onlyRemove = false) {
    if (!onlyRemove) {
      errors.add('sent_amount')
      errors.add('sent_asset')
      errors.add('sent_wallet')
      errors.add('recv_amount')
      errors.add('recv_asset')
      errors.add('recv_wallet')
      errors.add('manual_worth_asset')
    }
    if (!enabledFields.sent || info.sent_amount) errors.delete('sent_amount')
    if (!enabledFields.sent || info.sent_asset) errors.delete('sent_asset')
    if (!enabledFields.sent || info.sent_wallet) errors.delete('sent_wallet')
    if (!enabledFields.recv || info.recv_amount) errors.delete('recv_amount')
    if (!enabledFields.recv || info.recv_asset) errors.delete('recv_asset')
    if (!enabledFields.recv || info.recv_wallet) errors.delete('recv_wallet')
    if (info.manual_worth_amount && info.manual_worth_asset) errors.delete('manual_worth_asset')
    if (!info.manual_worth_amount) errors.delete('manual_worth_asset')
    if (!info.date) validDate = false
    errors = errors
    hasErrors = !!errors.size || !validDate
  }
  $: validate(info, true)

  function keydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault()
      close()
    }
  }
</script>

<Modal on:keydown={keydown} on:close>
  <form on:submit|preventDefault={save} class="container">
    <h2>Add transaction</h2>
    <div class="row">
      <p>Type</p>
      <Dropdown options={tags} bind:value={tag} let:option let:selected>
        <div class="tag-option" class:selected data-type={option.type}>
          <svg xmlns="http://www.w3.org/2000/svg" width="6" height="6" viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="12" />
          </svg>{option.value}
        </div>
      </Dropdown>
    </div>
    <div class="row date-row">
      <p>Date</p>
      <DateInput bind:value={info.date} bind:valid={validDate} />
    </div>
    <div class="row main-info">
      <div class="sent">
        {#if enabledFields.sent}
          <h4>Sent</h4>
          <div class="row">
            <input
              type="text"
              class="wallet"
              class:invalid={errors.has('sent_wallet')}
              bind:value={info.sent_wallet}
              placeholder="Wallet"
            />
          </div>
          <div class="row">
            <NumericInput
              bind:value={info.sent_amount}
              invalid={errors.has('sent_amount')}
              style={'border-top-right-radius: 0px; border-bottom-right-radius: 0px'}
              placeholder="Amount"
            />
            <input
              type="text"
              class="asset"
              class:invalid={errors.has('sent_asset')}
              bind:value={info.sent_asset}
              placeholder="Asset"
            />
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
              bind:value={info.recv_wallet}
              placeholder="Wallet"
            />
          </div>
          <div class="row">
            <NumericInput
              bind:value={info.recv_amount}
              invalid={errors.has('recv_amount')}
              style={'border-top-right-radius: 0px; border-bottom-right-radius: 0px'}
              placeholder="Amount"
            />
            <input
              type="text"
              class="asset"
              class:invalid={errors.has('recv_asset')}
              bind:value={info.recv_asset}
              placeholder="Asset"
            />
          </div>
        {/if}
      </div>
      {#if enabledFields.fee}
        <div class="fee">
          <h4>Fee</h4>
          <div class="row">
            <NumericInput bind:value={info.fee_amount} placeholder="Amount" />
          </div>
          <div class="row">
            <input type="text" bind:value={info.fee_asset} placeholder="Asset" />
          </div>
        </div>
      {/if}
    </div>
    {#if !showNetWorth}
      <div class="set-net-worth">
        <button type="button" class="m-0.5 py-0.5 px-1" on:click={() => (showNetWorth = true)}
          >Set Worth</button
        >
      </div>
    {/if}
    {#if showNetWorth}
      <div class="row">
        <p>Net Worth</p>
        <div class="amount-container">
          <NumericInput
            bind:value={info.manual_worth_amount}
            style={'border-top-right-radius: 0px; border-bottom-right-radius: 0px'}
            placeholder="Amount"
          />
        </div>
        <input
          type="text"
          class="asset"
          class:invalid={errors.has('manual_worth_asset')}
          bind:value={info.manual_worth_asset}
          placeholder="Asset"
        />
      </div>
    {/if}
    <div class="row">
      <p>Tx Hash</p>
      <input type="text" class="note" bind:value={info.hash} />
    </div>
    <div class="row">
      <p>Note</p>
      <textarea class="note" bind:value={info.note} />
    </div>
    <div class="mt-4 grid grid-flow-col justify-end gap-2">
      <Button secondary on:click={close}>Cancel</Button>
      <Button type="submit">Add</Button>
    </div>
  </form>
</Modal>

<style lang="sass">
  // fix tailwind issues
  :global(.dropdown select)
    background-image: none
  div :global(.date-time-picker .dropdown svg)
    box-sizing: content-box

  .container
    width: 580px
    max-width: 100%
    user-select: none
    cursor: default
    font-size: 12px
    --date-input-width: 136px
    --dropdown-width: 136px
  .tag-option
    padding: 4px 8px
    display: flex
    align-items: center
    border: 1px solid transparent
    &.selected
      background-color: rgba(#4d88ff, 0.25)
      color: #2974ff
    svg
      margin-right: 5px
      transform: scale(1) // for rendering glitch
      width: 7px
      height: 7px
    &[data-type='Deposit'] svg
      fill: #35d085
    &[data-type='Trade'] svg
      fill: #2ea8fa
    &[data-type='Withdrawal'] svg
      fill: #f92f72
    &[data-type='Transfer'] svg
      fill: #b853ee
  .sent, .received, .fee
    background-color: #eff2f5
    border: 1px solid #c6cddd
    width: 0px
    flex-grow: 1
    padding-bottom: 2px
    .row, h4
      margin: 10px
  .sent
    border-radius: 7px 0px 0px 7px
    width: 40%
  .received
    border-left: none
    width: 40%
  .row
    display: flex
    margin-top: 10px
  .main-info
    position: relative
    > :last-child
      border-radius: 0px 7px 7px 0px
      border-left: none
    .row
      margin: 9px
  .fee
    width: 25%
  p
    display: inline-block
    min-width: 80px
    margin: 4px 0px
    font-size: 13px
    cursor: default
  input, textarea
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
  input, textarea, .date-row :global(input)
    font-size: inherit
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
    margin-left: -1px
    border-top-left-radius: 0px
    border-bottom-left-radius: 0px
  .set-net-worth
    color: #0269f7
    font-size: 12px
    text-align: right
  .amount-container
    max-width: 130px
    display: flex
</style>
