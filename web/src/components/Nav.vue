<template lang='pug'>
.nav.logged-in(v-if='$pocket.isInAppArea && $account.loggedIn')
  router-link.nav-link(to='/app')
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
  router-link.nav-link(to='/app/dashboard')
    h4 Dashboard
  router-link.nav-link(to='/app/transactions')
    h4 Transactions
  .separator
  router-link.nav-link(to='/app/logout')
    h4 Log out
  MoonIcon.dark-theme-toggle(
    v-if='$pocket.darkTheme'
    @click='$pocket.toggleDarkTheme()'
    size='18'
  )
  SunIcon.dark-theme-toggle(
    v-else
    @click='$pocket.toggleDarkTheme()'
    size='18'
  )
.nav.logged-out(v-else)
  router-link.nav-link(to='/')
    h2 Cryptrack
  .separator
  router-link.nav-link(to='/login')
    h4 Log in
  router-link.nav-button(to='/Signup')
    Button Sign up
  MoonIcon.dark-theme-toggle(
    v-if='$pocket.darkTheme'
    @click='$pocket.toggleDarkTheme()'
    size='18'
  )
  SunIcon.dark-theme-toggle(
    v-else
    @click='$pocket.toggleDarkTheme()'
    size='18'
  )
</template>

<script>
import Dropdown from '@/components/Dropdown.vue'
import Button from '@/components/Button.vue'
import { SunIcon, MoonIcon } from 'vue-feather-icons'

export default {
  components: {
    Dropdown,
    Button,
    SunIcon,
    MoonIcon,
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
  position: relative
  z-index: 100
  $nav-item-horizontal-margin: 10px
  .dark-theme-toggle
    cursor: pointer
    padding: 4px
    margin-right: $nav-item-horizontal-margin
  .nav-link, .portfolio-picker, .nav-button
    margin: 0px $nav-item-horizontal-margin
  .portfolio-picker
    max-width: 250px
    width: auto
    font-weight: 700
  .nav-button
    button
      margin: 0px
  .nav-link
    cursor: pointer
    transition: 0.15s var(--easing)
    transition-property: opacity, color
    text-decoration: none
    padding: 2px
    h2, h3, h4
      font-family: 'Muli'
      font-weight: 700
      margin: 0px
    &:hover
      opacity: 0.75
  .separator
    margin-right: auto

.router-link-exact-active
  color: var(--accent-color)
</style>
