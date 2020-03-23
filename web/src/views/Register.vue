<template lang='pug'>
.register.page
  h1 Sign up
  TextBox.textbox(name='email' placeholder='Email' type='text')
  TextBox.textbox(name='password' placeholder='Password' type='password')
  TextBox.textbox(name='password-repeat' placeholder='Confirm' type='password')
  Button.btn(@click='register($route.query.continue || "/app")') Create account
</template>

<script>
import { mapActions, mapState } from 'vuex'
import TextBox from '@/components/TextBox.vue'
import Button from '@/components/Button.vue'

export default {
  title: 'Register',
  components: {
    TextBox,
    Button,
  },
  methods: {
    ...mapActions('account', ['register', 'logout']),
  },
  computed: mapState('account', ['loggedIn']),
}
</script>

<style lang='sass' scoped>
.register
  padding: 30px
  max-width: var(--page-form-width)
  text-align: center
  .textbox
    width: 100%
  .btn
    margin: 0px
    width: 100%

</style>
