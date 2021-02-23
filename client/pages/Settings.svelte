<title>Settings - Kryp</title>
<script>
  import { Meteor } from 'meteor/meteor'
  setTimeout(() => {
    const x = Meteor.user()
    console.log(x)
  }, 1000)
  let currency = ''
  let country = ''
  let errors = {}
  function save() {
    Meteor.call('accounts.edit', {
      currency,
      country,
    }, (err, res) => {
      if (err) {
        console.log(err)
        errors = err.details
      } else {
        console.log('succ')
      }
    })
  }
</script>

<style>
  h4 {
    margin-top: 12px;
    margin-bottom: 0px;
  }
</style>

<h1>Settings</h1>

<h4>Currency</h4>
<input type="text" bind:value={currency}>
{#if errors.currency} <p>{errors.currency}</p> {/if}

<h4>Country</h4>
<input type="text" bind:value={country}>
{#if errors.country} <p>{errors.country}</p> {/if}

<button type='submit' on:click={save}>Save</button>
