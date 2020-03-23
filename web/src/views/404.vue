<template lang='pug'>
.page
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
.page
  padding: 30px
  max-width: 390px
  text-align: center
  display: flex
  flex-direction: column
  justify-content: center
  .title
    margin: 0px
    margin-bottom: 5px
  .text
    margin: 0px
</style>
