<script>
  import Transactions from '../../lib/Transactions.js'
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

<table>
  <tr>
    <th>Type</th>
    <th>From</th>
    <th>To</th>
    <th>Fee</th>
    <th>txHash</th>
    <th>Note</th>
    <th>Date</th>
    <th></th>
  </tr>
  {#each $transactions as tx}
    <tr>
      <td>{tx.type}</td>
      <td>{tx.fromAmount} {tx.fromAsset} {tx.fromWallet}</td>
      <td>{tx.toAmount} {tx.toAsset} {tx.toWallet}</td>
      <td>{tx.feeAmount} {tx.feeAsset}</td>
      <td>{tx.txHash}</td>
      <td>{tx.note}</td>
      <td>{tx.date}</td>
      <td>
        <button on:click={editTx(tx)}>Edit</button>
        <button on:click={deleteTx(tx)}>Delete</button>
      </td>
    </tr>
  {/each}
</table>
