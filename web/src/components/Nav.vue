<template lang='pug'>
.nav
  router-link.nav-item.nav-link(
    :to='$pocket.loggedIn ? "/overview" : "/"'
  )
    h2 Cryptrack

  template(v-if='$pocket.loggedIn')
    Dropdown.nav-item.portfolio-picker(
      :defaultText='$pocket.currentPortfolio.name'
      :options=`portfolioPickerOptions`
      @change='portfolioPickerChange'
    )
    CreatePortfolioDialog(ref='createPortfolioDialog')

    router-link.nav-item.nav-link(:to='`/portfolio/${$pocket.currentPortfolio.id}`')
      h4 Dashboard
    router-link.nav-item.nav-link(:to='`/portfolio/${$pocket.currentPortfolio.id}/transactions`')
      h4 Transactions
    .separator
    router-link.nav-item.nav-link(to='/logout')
      h4 Log out

  template(v-else)
    .separator
    router-link.nav-item.nav-link(to='/login')
      h4 Log in
    .nav-item.nav-button(to='/signup')
      Button(@click='$router.push("/signup")') Sign up

  //- button with outline trick so that outline doesnt show on click
  button.outline-parent.nav-item.icon(
    @click.enter='$pocket.toggleDarkTheme()'
  )
    .outline-child(tabindex='-1')
      MoonIcon.outline-child(v-if='$pocket.darkTheme' size='18')
      SunIcon.outline-child(v-else size='18')
</template>

<script>
import Dropdown from '@/components/Dropdown.vue'
import Button from '@/components/Button.vue'
import TextBox from '@/components/TextBox.vue'
import CreatePortfolioDialog from '@/components/dialogs/CreatePortfolio.vue'
import { SunIcon, MoonIcon } from 'vue-feather-icons'

export default {
  components: {
    Dropdown,
    Button,
    TextBox,
    CreatePortfolioDialog,
    SunIcon,
    MoonIcon,
  },
  data () {
    return {
      showCreatePortfolioDialog: false,
      createPortfolioName: '',
    }
  },
  methods: {
    portfolioPickerChange (newText) {
      const portfolio = this.$pocket.portfolios.find(p => p.name === newText)
      if (this.$route.params.portfolioId) {
        this.$router.push({
          name: this.$route.name,
          params: { portfolioId: portfolio.id },
        })
      }
      this.$pocket.setPortfolioId(portfolio.id)
    },
  },
  computed: {
    portfolioPickerOptions () {
      const options = []
      options.push({ type: 'space' })
      options.push({ type: 'text', text: 'Portfolio' })

      for (const portfolio of this.$pocket.portfolios) {
        options.push({ text: portfolio.name })
      }

      options.push({ type: 'separator' })
      options.push({
        type: 'button',
        text: 'Create portfolio',
        handler: () => {
          this.$refs.createPortfolioDialog.open()
        },
      })
      options.push({ type: 'space' })
      return options
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
  position: relative
  z-index: 100
  max-width: 1400px
  $nav-item-horizontal-margin: 10px
  button.icon
    background-color: transparent
    border: none
    padding: 0px
    cursor: pointer
    transition: 0.15s var(--easing)
    transition-property: opacity, color
    &:hover
      opacity: 0.75
    .outline-child
      transition: box-shadow 0.15s var(--easing)
    svg
      padding: 4px
      display: block
  .nav-item
    margin: 0px $nav-item-horizontal-margin
  .portfolio-picker
    max-width: 180px
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
    padding: 4px 2px
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

.outline-parent:focus, .outline-parent > .outline-child:focus
    outline: none
.outline-parent:focus > .outline-child
    box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
</style>
