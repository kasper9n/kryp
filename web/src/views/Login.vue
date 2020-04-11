<template lang='pug'>
.mini-page
  h1 Log in
  TextBox.textbox(name='email' placeholder='Email')
  TextBox.textbox(name='password' placeholder='Password' type='password')
  .row
    router-link(to='/reset-password') Forgot password?
    Button.btn(@click='$account.login($route.query.continue || "/dashboard")') Log in
</template>

<script>
import TextBox from '@/components/TextBox.vue'
import Button from '@/components/Button.vue'

export default {
  title: 'Login',
  components: {
    TextBox,
    Button,
  },
}
</script>

<style lang='sass' scoped>
.textbox
  width: 100%
.row
  display: flex
  align-items: center
  justify-content: space-between
  a
    margin-right: 10px
    display: flex
    align-items: center
  .btn
    margin-top: 0px
    margin: 0px
</style>
