<script>
  import { Meteor } from 'meteor/meteor'
  let errors = {}
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
      hash: '',
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
    errors = {}
    action = actionArg
    if (action === 'add') reset()
    else if (action === 'edit') {
      txId = newTx._id
      tx = {
        type: newTx.type || 'Trade',
        date: newTx.date || 'Aug 27 21:27 PM',
        hash: newTx.hash || '',
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
  let txTypes = [
    'Trade',
    'Transfer',
    'Deposit',
    'Withdrawal',
  ]
  function save() {
    let method = 'transactions.add'
    if (action === 'edit') method = 'transactions.edit'
    Meteor.call(method, {
      transaction: tx,
      id: txId,
    }, (err, res) => {
      if (err) {
        console.log(err)
        errors = err.details
      } else {
        visible = false
        reset()
      }
    })
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
</style>

{#if visible}
  <div class="bg" on:click={bgClick}></div>
  <div class="container">
    <form class="box" on:submit|preventDefault>
      {#if action === 'add'}
        <h2>Add transaction</h2>
      {:else if action === 'edit'}
        <h2>Edit transaction</h2>
      {/if}
      <h4>Transaction Type</h4>
      <select bind:value={tx.type}>
        {#each txTypes as txType}
          <option value={txType}>
            {txType}
          </option>
        {/each}
      </select>
      {#if errors.type} <p>{errors.type}</p> {/if}
      <h4>Date</h4>
      <div class="row">
        <input type="text" bind:value={tx.date}>
        {#if errors.date} <p>{errors.date}</p> {/if}
      </div>
      <h4>From</h4>
      <div class="row">
        <div>
          <input type="text" class="amount" bind:value={tx.fromAmount} placeholder='Amount'>
          {#if errors.fromAmount} <p>{errors.fromAmount}</p> {/if}
        </div>
        <div>
          <input type="text" class="asset" bind:value={tx.fromAsset} placeholder='Asset'>
          {#if errors.fromAsset} <p>{errors.fromAsset}</p> {/if}
        </div>
        <div>
          <input type="text" class="wallet" bind:value={tx.fromWallet} placeholder='Wallet'>
          {#if errors.fromWallet} <p>{errors.fromWallet}</p> {/if}
        </div>
      </div>
      <h4>To</h4>
      <div class="row">
        <div>
          <input type="text" class="amount" bind:value={tx.toAmount} placeholder='Amount'>
          {#if errors.toAmount} <p>{errors.toAmount}</p> {/if}
        </div>
        <div>
          <input type="text" class="asset" bind:value={tx.toAsset} placeholder='Asset'>
          {#if errors.toAsset} <p>{errors.toAsset}</p> {/if}
        </div>
        <div>
          <input type="text" class="wallet" bind:value={tx.toWallet} placeholder='Wallet'>
          {#if errors.toWallet} <p>{errors.toWallet}</p> {/if}
        </div>
      </div>
      <h4>Fee</h4>
      <div class="row">
        <div>
          <input type="text" class="amount" bind:value={tx.feeAmount} placeholder='Amount'>
          {#if errors.feeAmount} <p>{errors.feeAmount}</p> {/if}
        </div>
        <div>
          <input type="text" class="asset" bind:value={tx.feeAsset} placeholder='Asset'>
          {#if errors.feeAsset} <p>{errors.feeAsset}</p> {/if}
        </div>
      </div>
      <h4>Transaction Hash</h4>
      <div class="row">
        <input type="text" bind:value={tx.hash}>
        {#if errors.hash} <p>{errors.hash}</p> {/if}
      </div>
      <h4>Note</h4>
      <div class="row">
        <textarea bind:value={tx.note}></textarea>
        {#if errors.note} <p>{errors.note}</p> {/if}
      </div>
      <div class="row">
        <button type='submit' on:click={save}>
          {#if action === 'add'} Add
          {:else if action === 'edit'} Save
          {/if}
        </button>
      </div>
    </form>
  </div>
{/if}
