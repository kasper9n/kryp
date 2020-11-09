<script>
  import Transactions from '../../lib/Transactions.js'
  import Transaction from './Tx.svelte'
  import TxDialog from './TxDialog.svelte'
  let open
  $: transactions = Transactions.find({})
  let txDialogOptions

  function addTx() {
    // txDialogOptions = {action: 'add'}
    open('add')
  }
  function editTx(transaction) {
    // txDialogOptions = {action: 'edit'}
    open('edit', transaction)
  }
  function deleteTx(transaction) {
    Transactions.remove(transaction._id)
  }
</script>

<style>
  .action-menu {
    display: flex;
  }
</style>


<h1>Transactions</h1>
<TxDialog {...txDialogOptions} bind:open={open} />
<div class="action-menu">
  <button on:click={addTx}>Add transaction</button>
</div>

{#each $transactions as tx}
  <div class="row">
    <div class='type'>{tx.type}</div>
    <div>from: {tx.fromAmount} {tx.fromAsset} {tx.fromWallet}</div>
    <div>to: {tx.toAmount} {tx.toAsset} {tx.toWallet}</div>
    <div>fee: {tx.feeAmount} {tx.feeAsset}</div>
    <div>txHash: {tx.txHash}</div>
    <div>note: {tx.note}</div>
    <div>date: {tx.date}</div>
    <button on:click={editTx(tx)}>Edit</button>
    <button on:click={deleteTx(tx)}>Delete</button>
  </div>
{/each}
