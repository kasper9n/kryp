<script>
  import { url } from '@roxi/routify'
</script>

<style lang='sass'>
  .navbar
    display: flex
    align-items: center
    max-width: 1200px
    height: 70px
    user-select: none
  .logo
    font-size: 24px
  .nav-item
    padding: 4px 8px
    margin-right: 26px
    cursor: pointer
</style>

<template lang='pug'>
  .navbar
    router-link(to='/').nav-item.logo Kryp
    router-link(to='/').nav-item Dashboard
    router-link(to='/wallets').nav-item Wallets
    router-link(to='/transactions').nav-item Transactions
</template>
