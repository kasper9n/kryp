<script>
  import Transactions from '../../lib/Transactions.js'
  let action
  let visible = false
  let txId
  let tx
  reset()
  function reset() {
    txId = null
    tx = {
      type: 'Trade',
      date: 'Aug 28 21:27 PM',
      txHash: '',
      note: '',
      fromWallet: '',
      fromAmount: '',
      fromAsset: '',
      feeAmount: '',
      feeAsset: '',
      toWallet: '',
      toAmount: '',
      toAsset: '',
    }
  }
  export const open = (actionArg, newTx) => {
    action = actionArg
    if (action === 'add') reset()
    else if (action === 'edit') {
      txId = newTx._id
      tx = {
        type: newTx.type || 'Trade',
        date: newTx.date || 'Aug 27 21:27 PM',
        txHash: newTx.txHash || '',
        note: newTx.note || '',
        fromWallet: newTx.fromWallet || '',
        fromAmount: newTx.fromAmount || '',
        fromAsset: newTx.fromAsset || '',
        feeAmount: newTx.feeAmount || '',
        feeAsset: newTx.feeAsset || '',
        toWallet: newTx.toWallet || '',
        toAmount: newTx.toAmount || '',
        toAsset: newTx.toAsset || '',
      }
    }
    visible = true
  }
  let tradeTypes = [
    'Trade',
    'Transfer',
    'Deposit',
    'Withdrawal',
  ]
  function save() {
    if (action === 'add') {
      Transactions.insert(tx)
    } else if (action === 'edit') {
      Transactions.update(
        { _id: txId },
        tx,
      )
    }
    visible = false
    reset()
  }
  function bgClick(e) {
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
  }
  h2 {
    margin-top: 0px;
  }
  .container {
    pointer-events: none;
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
    pointer-events: all;
    background-color: #ffffff;
    padding: 20px;
    width: 500px;
  }
  .row {
    display: flex;
  }
  h4 {
    margin-top: 12px;
    margin-bottom: 0px;
  }
  input.asset {
    width: 50px
  }
</style>

{#if visible}
  <div class="bg" on:click={bgClick}></div>
  <div class="container" on:submit|preventDefault>
    <form class="box">
      {#if action === 'add'}
        <h2>Add transaction</h2>
      {:else if action === 'edit'}
        <h2>Edit transaction</h2>
      {/if}
      <h4>Transaction Type</h4>
      <select bind:value={tx.type}>
        {#each tradeTypes as tradeType}
          <option value={tradeType}>
            {tradeType}
          </option>
        {/each}
      </select>
      <h4>Date</h4>
      <div class="row">
        <input type="text" bind:value={tx.date}>
      </div>
      <h4>From</h4>
      <div class="row">
        <input type="text" class="amount" bind:value={tx.fromAmount} placeholder='Amount'>
        <input type="text" class="asset" bind:value={tx.fromAsset} placeholder='Asset'>
        <input type="text" class="wallet" bind:value={tx.fromWallet} placeholder='Wallet'>
      </div>
      <h4>To</h4>
      <div class="row">
        <input type="text" class="amount" bind:value={tx.toAmount} placeholder='Amount'>
        <input type="text" class="asset" bind:value={tx.toAsset} placeholder='Asset'>
        <input type="text" class="wallet" bind:value={tx.toWallet} placeholder='Wallet'>
      </div>
      <h4>Fee</h4>
      <div class="row">
        <input type="text" class="amount" bind:value={tx.feeAmount} placeholder='Amount'>
        <input type="text" class="asset" bind:value={tx.feeAsset} placeholder='Asset'>
      </div>
      <h4>Transaction Hash</h4>
      <div class="row">
        <input type="text" bind:value={tx.txHash}>
      </div>
      <h4>Note</h4>
      <div class="row">
        <textarea bind:value={tx.note}></textarea>
      </div>
      <div class="row">
        <button type='submit' on:click={save}>Save</button>
      </div>
    </form>
  </div>
{/if}
