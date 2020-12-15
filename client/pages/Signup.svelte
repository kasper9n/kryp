<title>Signup - Kryp</title>
<script>
  import { Meteor } from 'meteor/meteor'
  let email = ''
  let password = ''
  let confirmPassword = ''
  let errors = {}

  function createAccount() {
    errors = {}

    if (email === '') errors.email = 'Enter an email'
    if (password === '') errors.password = 'Enter a password'
    if (confirmPassword === '') errors.confirmPassword = 'Confirm your password'
    else if (confirmPassword !== password) errors.confirmPassword = "Passwords don't match"
    if (Object.keys(errors).length !== 0) return

    Meteor.call('accounts.create', {
      email,
      password,
    }, (err, res) => {
      if (err) {
        console.log('signup error', err)
        errors = err.details
      } else {
        console.log('signup success', res)
      }
    })
  }
</script>

<style>
  h1 {
    text-align: center;
  }
  .row {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>

<h1>Signup</h1>

<form class="box" on:submit|preventDefault>
  <div class="row">
    <input type="text" bind:value={email} placeholder='Email'>
    {#if errors.email} <p>{errors.email}</p> {/if}
  </div>
  <div class="row">
    <input type="password" bind:value={password} placeholder='Password'>
    {#if errors.password} <p>{errors.password}</p> {/if}
  </div>
  <div class="row">
    <input type="password" bind:value={confirmPassword} placeholder='Confirm'>
    {#if errors.confirmPassword} <p>{errors.confirmPassword}</p> {/if}
  </div>
  <div class="row">
    <button type='submit' on:click={createAccount}>Create account</button>
  </div>
</form>
