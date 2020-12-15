<title>Wallets - Kryp</title>
<script>
  import { Meteor } from 'meteor/meteor'
  import WalletDialog from '../components/WalletDialog.svelte'
  import * as stores from '../stores'
  let open
  $: wallets = stores.wallets
  let WalletDialogOptions

  function addWallet() {
    open('add')
  }
  function editWallet(wallet) {
    open('edit', wallet)
  }
  function deleteWallet(wallet) {
    Meteor.call('wallets.delete', {
      id: wallet._id,
    }, (err, res) => {
      if (err) {
        console.log('deleteWallet() err:', err)
      }
    })
  }
</script>

<style>
  .action-menu {
    display: flex;
  }
</style>

<h1>wallets</h1>
<WalletDialog {...WalletDialogOptions} bind:open={open} />
<div class="action-menu">
  <button on:click={addWallet}>Add wallet</button>
</div>

<table>
  <tr>
    <th>Name</th>
  </tr>
  {#each $wallets as wallet}
    <tr>
      <td>{wallet.name}</td>
      <td>
        <button on:click={editWallet(wallet)}>Edit</button>
        <button on:click={deleteWallet(wallet)}>Delete</button>
      </td>
    </tr>
  {/each}
</table>
