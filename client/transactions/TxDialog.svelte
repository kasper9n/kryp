<script>
  export let action
  import Transactions from '../../lib/Transactions.js'
  let visible = false
  export function open() {
    visible = true
  }
  let tradeTypes = [
    { id: 0, text: 'Trade' },
    { id: 1, text: 'Transfer' },
    { id: 2, text: 'Deposit' },
    { id: 3, text: 'Withdrawal' },
  ]
  let tradeType = tradeTypes[0]
  let date = 'Aug 28 21:27PM'
  let txHash = ''
  let note = ''
  let fromWallet = ''
  let from = ''
  let fromCur = ''
  let fee = ''
  let feeCur = ''
  let toWallet = ''
  let to = ''
  let toCur = ''
  function save(e) {
    Transactions.insert({
      type: tradeType.text,
      date,
      txHash,
      note,

      fromWallet,
      from,
      fromCur,

      fee,
      feeCur,

      to,
      toCur,
      toWallet,
    })
    visible = false
  }
  function bgClick() {
    visible = false
  }
</script>

<style>
  .bg {
    top: 0px;
    left: 0px;
    width: 100vw;
    height: 100vh;
    position: fixed;
    background-color: rgba(0, 0, 0, 0.66);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  .box {
    background-color: #ffffff;
    padding: 10px;
    width: 500px;
  }
  .row {
    display: flex;
  }
  h4 {
    margin-top: 12px;
    margin-bottom: 0px;
  }
</style>

{#if visible}
  <div class="bg" on:click|self={bgClick}>
    <div class="box">
      {#if action === 'add'}
        <h2>Add transaction</h2>
      {:else if action === 'edit'}
        <h2>Edit transaction</h2>
      {/if}
      <h4>Transaction Type</h4>
      <select bind:value={tradeType}>
        {#each tradeTypes as tradeType}
          <option value={tradeType}>
            {tradeType.text}
          </option>
        {/each}
      </select>
      <h4>Date</h4>
      <div class="row">
        <input type="text" bind:value={date}>
      </div>
      <h4>From</h4>
      <div class="row">
        <input type="text" bind:value={fromWallet} placeholder='Wallet'>
        <input type="text" bind:value={fromCur} placeholder='Amount'>
        <input type="text" bind:value={from} placeholder='Asset'>
      </div>
      <h4>To</h4>
      <div class="row">
        <input type="text" bind:value={toWallet} placeholder='Wallet'>
        <input type="text" bind:value={toCur} placeholder='Amount'>
        <input type="text" bind:value={to} placeholder='Asset'>
      </div>
      <h4>Fee</h4>
      <div class="row">
        <input type="text" bind:value={feeCur} placeholder='Amount'>
        <input type="text" bind:value={fee} placeholder='Asset'>
      </div>
      <h4>Transaction Hash</h4>
      <div class="row">
        <input type="text" bind:value={txHash}>
      </div>
      <h4>Note</h4>
      <div class="row">
        <textarea value={note}></textarea>
      </div>
      <div class="row">
        <button on:click={save}>Save</button>
      </div>
    </div>
  </div>
{/if}
