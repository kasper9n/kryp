<script>
  import Transactions from '../../lib/Transactions.js'
  import Transaction from './Tx.svelte'
  import TxDialog from './TxDialog.svelte'
  let open
  $: transactions = Transactions.find({})
</script>

<style>
  .action-menu {
    display: flex;
  }
</style>


<h1>Transactions</h1>
<div class="action-menu">
  <TxDialog action='add' bind:open={open} />
  <button on:click={open}>Add transaction</button>
</div>

{#each $transactions as transaction}
  <Transaction {transaction}/>
{/each}
