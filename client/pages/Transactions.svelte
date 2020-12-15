<title>Transactions - Kryp</title>
<script>
  import { Meteor } from 'meteor/meteor'
  import TxDialog from '../components/TxDialog.svelte'
  import * as stores from '../stores'
  let open
  $: transactions = stores.transactions
  let txDialogOptions

  function addTx() {
    open('add')
  }
  function editTx(transaction) {
    open('edit', transaction)
  }
  function deleteTx(transaction) {
    Meteor.call('transactions.delete', {
      id: transaction._id,
    }, (err, res) => {
      if (err) {
        console.log('deleteTx() err:', err)
      }
    })
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
    <th>Hash</th>
    <th>Note</th>
    <th>Date</th>
    <th></th>
  </tr>
  {#each $transactions as tx}
    <tr>
      <td>{tx.type}</td>
      <td>{tx.fromAmount} {tx.fromAsset} {tx.fromWallet}</td>
      <td>{tx.toAmount} {tx.toAsset} {tx.toWallet}</td>
      <td>{tx.feeAmount || ''} {tx.feeAsset || ''}</td>
      <td>{tx.hash || ''}</td>
      <td>{tx.note || ''}</td>
      <td>{tx.date}</td>
      <td>
        <button on:click={editTx(tx)}>Edit</button>
        <button on:click={deleteTx(tx)}>Delete</button>
      </td>
    </tr>
  {/each}
</table>
