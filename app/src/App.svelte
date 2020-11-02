<script>
  import { Router } from '@roxi/routify'
  import { routes } from '../.routify/routes.js'
  import NavBar from './Navbar.svelte'
  import { goto } from '@roxi/routify'

  document.addEventListener('click', (e) => {
    if (e.target.nodeName === 'ROUTER-LINK' && e.target.hasAttribute('to')) {
      $goto(e.target.getAttribute('to'))
    }
  })
</script>

<style lang='sass'>
  :global
    b, h1, h2, h3, h4, h5, h6
      font-family: 'Overpass'
      font-weight: 600
    body
      margin: 0px 30px
      background-color: var(--bg-color-2)
    #app
      max-width: 1200px
      margin: auto
      font-size: 15px
      font-family: 'Inter', sans-serif
      color: var(--text-color)
  :root
    --bg-color-1: #ffffff
    --bg-color-2: #f8f9fc
    --text-color: #444444
    --shadow: 0px 0px 5px 0px rgba(0,0,0,0.1)
</style>

<template lang='pug'>
  NavBar
  .page
    Router('{routes}')
</template>
