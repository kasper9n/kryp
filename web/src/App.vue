<template lang='pug'>
#app
  //- workaround for style tags being ignored
  component(is='style' v-if='!isFirefox')
    //- We want to not change Firefox's outline style because it's dotted by
    //- default (which looks weird with 3px outline-width), and it needs to
    //- have the default style to avoid the outline showing up every time a
    //- focusable element is clicked. To avoid specificity issues, I didn't
    //- want to to use a .firefox class, so this was the best way I could find
    //- to conditionally set CSS properties on pseudo elements.
    | :focus {
    |   outline-width: 3px;
    |   outline-color: var(--line-highlight-color);
    | }
  Nav
  transition(name='fade' mode='out-in')
    router-view
</template>

<script>
import Nav from '@/components/Nav.vue'
import LightTheme from '@/styles/LightTheme.vue'
import DarkTheme from '@/styles/DarkTheme.vue'

export default {
  name: 'home',
  components: {
    Nav,
    LightTheme,
    DarkTheme,
  },
  data () {
    return {
      isFirefox: navigator.userAgent.toLowerCase().indexOf('firefox') > -1,
    }
  },
  created () {
    this.$pocket.updateTheme()
  },
}
</script>

<style lang='sass'>
body
  margin: 0px
#app
  font-family: 'Muli', Helvetica, Arial, sans-serif
  font-weight: 600
  font-size: 15px
  -webkit-font-smoothing: antialiased
  -moz-osx-font-smoothing: grayscale
  background-color: var(--background-color-2)
  min-height: 100vh
  width: 100vw
  min-width: 250px
::-moz-focus-inner
  // disable dotted border Firefox shows sometimes even when outline is set to none.
  border: 0
#app, a
  color: var(--text-color)
div
  box-sizing: border-box
svg.feather
  color: var(--text-color)
a:hover
  text-decoration: none
p
  margin: 10px 0px
h1, h2, h3, h4, h5, h6
  font-family: 'Rubik', Helvetica, Arial, sans-serif
  font-weight: 500
.page
  max-width: 1300px
  margin: auto
  padding-top: 20px
  padding-bottom: 50px
  padding-left: 15px
  padding-right: 15px
.mini-page
  max-width: 400px
  box-sizing: border-box
  margin: auto
  padding: 30px
  text-align: center
  min-height: calc(100vh - var(--header-height))

.fade-enter-active, .fade-leave-active
  transition: all 0.1s ease-in-out
  opacity: 1
.fade-enter, .fade-leave-to
  opacity: 0
</style>
