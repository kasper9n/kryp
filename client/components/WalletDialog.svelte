<script>
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
        <button type='submit' on:click={save}>
          {#if action === 'add'} Add
          {:else if action === 'edit'} Save
          {/if}
          </button>
      </div>
    </form>
  </div>
{/if}
