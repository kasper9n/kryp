<template lang='pug'>
.dashboard
  p Logged in: {{ loggedIn }}
  p Email: {{ email }}
  p firstName: {{ firstName }}
  p lastName: {{ lastName }}
  button(@click='login()') Log in
  button(@click='logout()') Log out
</template>

<script>
import { mapActions, mapState } from 'vuex'

export default {
  title: 'Dashboard',
  methods: {
    ...mapActions('account', ['login', 'logout']),
  },
  computed: mapState('account', ['loggedIn', 'email', 'firstName', 'lastName']),
}
</script>

<style lang='sass' scoped>
</style>
