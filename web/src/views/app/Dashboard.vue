<template lang='pug'>
.dashboard
  p Logged in: {{ $account.loggedIn }}
  p Email: {{ $account.email }}
  p firstName: {{ $account.firstName }}
  p lastName: {{ $account.lastName }}
</template>

<script>
export default {
  title: 'Dashboard',
}
</script>

<style lang='sass' scoped>
</style>
