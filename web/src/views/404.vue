<template lang='pug'>
.mini-page
  .centered
    h1.title 404
    p.text Page not found
</template>

<script>
export default {
  title: '404',
}
</script>

<style lang='sass' scoped>
.link
  display: flex
  align-items: center
.mini-page
  display: flex
  flex-direction: column
  justify-content: center
  .title
    margin: 0px
    margin-bottom: 5px
  .text
    margin: 0px
</style>
