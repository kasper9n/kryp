<title>Log out - Kryp</title>
<script>
  import { navigate } from 'svelte-routing'
  import { Meteor } from 'meteor/meteor'
  let error = ''
  Meteor.logout((err) => {
    if (err) {
      error = err.message
    } else {
      navigate('/', { replace: true })
    }
  })
</script>

<style>
  p, h3 {
    text-align: center;
  }
</style>

{#if error}
  <h3>Error logging out</h3>
  <p>{error}</p>
{/if}
