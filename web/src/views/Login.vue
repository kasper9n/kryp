<template lang='pug'>
form.mini-page(
  @submit.prevent='login'
  :action='$pocket.apiUrl'
  method='post'
  novalidate='true'
)
  h1 Log in
  .page-error(v-if='pageErrorMsg !== ""') {{ pageErrorMsg }}

  label(for='login-email')
  TextBox.textbox(
    id='login-email'
    v-model='email'
    name='email'
    :error='!!emailError'
    placeholder='Email'
    type='email'
    autocomplete='username'
  )
  p.error(v-if='emailError === "empty"') Enter an email address
  p.error(v-if='emailError === "invalid"') Invalid email address

  label(for='login-password')
  TextBox.textbox(
    id='login-password'
    v-model='password'
    name='password'
    :error='!!passwordError'
    placeholder='Password'
    type='password'
    autocomplete='current-password'
  )
  p.error(v-if='passwordError === "empty"') Enter a password

  .row
    router-link(to='/reset-password') Forgot password?
    Button.btn Log in
</template>

<script>
import validator from 'validator'
import TextBox from '@/components/TextBox.vue'
import Button from '@/components/Button.vue'

export default {
  title: 'Login',
  components: {
    TextBox,
    Button,
  },
  data: function () {
    return {
      pageErrorMsg: '',
      email: '',
      emailError: false,
      password: '',
      passwordError: false,
      inProgress: false,
    }
  },
  methods: {
    validateEmail: function () {
      this.emailError = false
      const email = this.email
      if (validator.isEmpty(email)) this.emailError = 'empty'
      else if (!validator.isEmail(email)) this.emailError = 'invalid'
    },
    validatePassword: function () {
      this.passwordError = false
      const password = this.password
      if (validator.isEmpty(password)) this.passwordError = 'empty'
    },
    login: function (e) {
      if (this.inProgress === true) return
      this.pageErrorMsg = ''
      this.validateEmail()
      this.validatePassword()
      if (this.emailError || this.passwordError) return
      this.inProgress = true
      this.$account.login({
        email: this.email,
        password: this.password,
      }).then(() => {
        console.log('login success')
        this.inProgress = false
        // full page reload so password managers detect submission
        window.location.href = this.$route.query.continue || '/p/x/dashboard'
      }, err => {
        if (err.msg === 'Server unreachable') {
          this.pageErrorMsg = 'Unable to reach server'
        } else if (err.msg === 'Login incorrect') {
          this.pageErrorMsg = 'Incorrect email or password'
        } else {
          this.pageErrorMsg = `Unexpected error: ${err.code} ${err.msg}`
        }
        this.inProgress = false
      })
    },
  },
}
</script>

<style lang='sass' scoped>
.textbox
  width: 100%
p.error
  color: var(--negative-color)
  margin-top: -6px
  margin-left: 2px
  font-size: 13px
  text-align: left
.page-error
  color: var(--negative-color)
  margin-top: -6px
  margin-left: 2px
  font-size: 13px
  text-align: left
  background-color: var(--error-background-color)
  border-radius: 3px
  padding: 10px 20px
  text-align: center
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
