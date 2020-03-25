<template lang='pug'>
.nav.logged-in(v-if='$pocket.isInAppArea && $account.loggedIn')
  router-link.nav-link(to='/app/dashboard')
    h2 Cryptrack
  Dropdown.portfolio-picker(
    defaultText='Main'
    :options=`[
      { type: "space" },
      {
        type: "text",
        text: "Portfolio",
      },
      {
        text: "Main",
      },
      {
        text: "Crypto",
      },
      {
        text: "Stocks and stuff haha",
      },
      { type: "separator" },
      {
        type: "button",
        text: "Create portfolio",
      },
      { type: "space" },
    ]`
  )
  .separator
  button.nav-link(@click='$account.logout()')
    h4(tabindex='-1') Log out
.nav.logged-out(v-else)
  router-link.nav-link(to='/')
    h2 Cryptrack
  .separator
  router-link.nav-link(to='/Signup')
    h4 Sign up
  router-link.nav-link(to='/login')
    h4 Log in
</template>

<script>
import Dropdown from '@/components/Dropdown.vue'

export default {
  components: {
    Dropdown,
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
  button
    border: none
    font-size: inherit
    outline: none
    h4:focus
      outline: none
    &:focus h4
      outline: var(--line-highlight-color) 3px auto
  .nav-link, .portfolio-picker
    margin: 0px 10px
  .portfolio-picker
    max-width: 250px
    width: auto
  .nav-link
    cursor: pointer
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
