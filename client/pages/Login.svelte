<script>
  import { Meteor } from 'meteor/meteor'
  let email = ''
  let password = ''
  let errors = {}

  function login() {
    errors = {}

    if (email === '') errors.email = 'Enter an email'
    if (password === '') errors.password = 'Enter a password'
    if (Object.keys(errors).length !== 0) return

    const selector = { email: email }
    Meteor.loginWithPassword(selector, password, (err) => {
      if (err) {
        switch(err.message) {
        case 'Unrecognized options for login request [400]':
          errors.general = "This shouldn't happen: "+err.message
          break
        case 'Match failed [400]':
          errors.general = "This shouldn't happen: "+err.message
          break
        case 'User not found [403]':
          errors.general = 'User not found'
          break
        case 'Incorrect password [403]':
          errors.password = 'Incorrect password'
          break
        case 'User has no password set [403]':
          errors.general = "This shouldn't happen: "+err.message
          break
        default:
          errors.general = "This shouldn't happen: "+err.message
        }
        console.log('error logging in', err)
      } else {
        console.log('login success')
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

<h1>Login</h1>

<form class="box" on:submit|preventDefault>
  <div class="row">
    {#if errors.general} <p>{errors.general}</p> {/if}
  </div>
  <div class="row">
    <input type="text" bind:value={email} placeholder='Email'>
    {#if errors.email} <p>{errors.email}</p> {/if}
  </div>
  <div class="row">
    <input type="password" bind:value={password} placeholder='Password'>
    {#if errors.password} <p>{errors.password}</p> {/if}
  </div>
  <div class="row">
    <button type='submit' on:click={login}>Log in</button>
  </div>
</form>
