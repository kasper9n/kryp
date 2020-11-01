<script lang='ts'>
  import { url } from '@roxi/routify'
  import Link from './Link.svelte'
</script>

<style lang='sass'>
  .navbar
    display: flex
    align-items: center
    max-width: 1200px
    height: 60px
  .logo
    font-size: 28px
  .nav-item :global(> div)
    padding: 4px 8px
    margin-left: 26px
    cursor: pointer
    user-select: none
</style>

<template lang='pug'>
  .navbar
    .nav-item.logo Kryp
    .nav-item
      Link(to='/') Dashboard
    .nav-item
      Link(to='/wallets') Wallets
    .nav-item
      Link(to='/transactions') Transactions
</template>
