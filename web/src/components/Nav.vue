<template lang='pug'>
.nav.logged-in(v-if='$store.state.appArea && $store.state.account.loggedIn')
  router-link.item(to='/app/dashboard')
    h2 Cryptrack
  select.item
    option Main
    option Crypto
    option Stocks
  router-link.item(to='/app/portfolio/g7eia')
    h4 Main
  .separator
  .item(@click='logout()')
    h4 Log out
.nav.logged-out(v-else)
  router-link.item(to='/')
    h2 Cryptrack
  .separator
  router-link.item(to='/Signup')
    h4 Sign up
  router-link.item(to='/login')
    h4 Log in
</template>

<script>
import { mapActions } from 'vuex'

export default {
  methods: {
    ...mapActions('account', ['logout']),
    loginPage () {
      return this.$router.currentRoute.path === '/login'
    },
  },
}
</script>

<style lang='sass' scoped>
.nav
  background-color: var(--background-color-1)
  box-shadow: var(--shadow)
  display: flex
  align-items: center
  font-weight: 600
  height: var(--header-height)
  padding: 0px 30px
  white-space: nowrap
  user-select: none
  .item
    cursor: pointer
    margin: 0px 10px
    transition: 0.15s var(--easing)
    transition-property: opacity, color
    text-decoration: none
    padding: 2px
    h2, h3, h4
      margin: 0px
    &:hover
      opacity: 0.75
  .separator
    margin-right: auto

.router-link-exact-active
  color: var(--accent-color)
</style>
