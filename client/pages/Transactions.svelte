<script>
  import useTracker from 'meteor/rdb:svelte-meteor-data'
  import Transactions from '../../lib/Transactions.js'
  console.log(Transactions.find({}).fetch())
  // $: transactions = Transactions.find({}).fetch()
  $: transactions = Transactions.find({});
</script>

<h1>Transactions</h1>
{#each $transactions as transaction}
  <h4>works:</h4>
  <p>{transaction.x}</p>
{/each}
<p>tx</p>
