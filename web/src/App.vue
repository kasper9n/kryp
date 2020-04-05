<template lang='pug'>
#app
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
:focus
  outline-color: var(--line-highlight-color)
  outline-width: 3px
#app, a
  color: var(--text-color)
div
  box-sizing: border-box
a:hover
  text-decoration: none
p
  margin: 10px 0px
h1, h2, h3, h4, h5, h6
  font-family: 'Rubik', Helvetica, Arial, sans-serif
  font-weight: 500
input
  outline: none

.page
  max-width: 1300px
  margin: auto
  padding-top: 20px
  padding-bottom: 50px
  padding-left: 15px
  padding-right: 15px
.mini-page
  max-width: 400px
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
