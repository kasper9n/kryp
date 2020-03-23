<template lang='pug'>
#app
  Nav
  transition(name='fade' mode='out-in')
    router-view
</template>

<script>
import Nav from '@/components/Nav.vue'
import Theme from '@/components/Theme.vue'

export default {
  name: 'home',
  components: {
    Nav,
    Theme,
  },
}
</script>

<style lang='sass'>
body
  margin: 0px
#app
  font-family: 'Muli', Helvetica, Arial, sans-serif
  font-weight: 600
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
#app
  font-size: 15px
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
  max-width: 1150px
  margin: auto
  min-height: calc(100vh - var(--header-height))

.fade-enter-active, .fade-leave-active
  transition: all 0.1s ease-in-out
  opacity: 1

.fade-enter, .fade-leave-to
  opacity: 0

</style>
