<script>
  import './dialog.css'
  import { Meteor } from 'meteor/meteor'
  let action
  let visible = false
  let walletId
  let wallet
  reset()
  function reset() {
    walletId = null
    wallet = {
      name: '',
    }
  }
  export const open = (actionArg, newWallet) => {
    action = actionArg
    if (action === 'add') reset()
    else if (action === 'edit') {
      walletId = newWallet._id
      wallet = {
        type: newWallet.name || 'Trade',
      }
    }
    visible = true
  }
  let errors = {}
  function save() {
    let method = 'wallets.add'
    if (action === 'edit') method = 'wallets.edit'
    Meteor.call(method, {
      wallet: wallet,
      id: walletId,
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

{#if visible}
  <div class="bg" on:click={bgClick}></div>
  <div class="container">
    <form class="box" on:submit|preventDefault>
      {#if action === 'add'}
        <h2>Add wallet</h2>
      {:else if action === 'edit'}
        <h2>Edit wallet</h2>
      {/if}
      <h4>Name</h4>
      <div class="row">
        <input type="text" bind:value={wallet.name}>
        {#if errors.name} <p>{errors.name}</p> {/if}
      </div>
      <div class="row">
        <button type='submit' on:click={save}>Save</button>
      </div>
    </form>
  </div>
{/if}
