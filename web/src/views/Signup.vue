<template lang='pug'>
.mini-page
  h1 Sign up
  TextBox.textbox(name='email' placeholder='Email' type='text')
  TextBox.textbox(name='password' placeholder='Password' type='password')
  TextBox.textbox(name='password-repeat' placeholder='Confirm' type='password')
  Button.btn(@click='$account.signup("/app/dashboard")') Create account
</template>

<script>
import TextBox from '@/components/TextBox.vue'
import Button from '@/components/Button.vue'

export default {
  title: 'Sign up',
  components: {
    TextBox,
    Button,
  },
}
</script>

<style lang='sass' scoped>
.textbox
  width: 100%
.btn
  margin: 0px
  width: 100%
</style>
