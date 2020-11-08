<script>
  export let transaction
  import Transactions from '../../lib/Transactions.js'
  function deleteTx() {
    Transactions.remove(transaction._id)
  }
</script>

<h4>Tx</h4>
<div>type: {transaction.type}</div>
<div>date: {transaction.date}</div>
<div>fromWallet: {transaction.fromWallet}</div>
<div>from: {transaction.from}</div>
<div>fromCur: {transaction.fromCur}</div>
<div>to: {transaction.to}</div>
<div>toCur: {transaction.toCur}</div>
<div>toWallet: {transaction.toWallet}</div>
<div>fee: {transaction.fee}</div>
<div>feeCur: {transaction.feeCur}</div>
<div>txHash: {transaction.txHash}</div>
<div>note: {transaction.note}</div>
<button on:click={deleteTx}>Delete</button>
