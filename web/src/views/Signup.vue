<template lang='pug'>
form.mini-page(
  @submit.prevent='signup'
  :action='$pocket.apiUrl'
  method='post'
  novalidate='true'
)
  h1 Sign up
  .page-error(v-if='pageErrorMsg !== ""') {{ pageErrorMsg }}

  label(for='signup-email')
  TextBox.textbox(
    id='signup-email'
    v-model='email'
    name='email'
    :error='!!emailError'
    placeholder='Email'
    type='email'
  )
  p.error(v-if='emailError === "empty"') Enter an email address
  p.error(v-if='emailError === "invalid"') Invalid email address
  p.error(v-if='emailError === "exists"') Email already exists

  label(for='signup-password')
  TextBox.textbox(
    id='signup-password'
    v-model='password'
    name='password'
    :error='!!passwordError'
    placeholder='Password'
    type='password'
    autocomplete='new-password'
  )
  p.error(v-if='passwordError === "empty"') Enter a password
  p.error(v-if='passwordError === "too short"') Password must be 8-100 characters
  p.error(v-if='passwordError === "too long"') Password must be 8-100 characters

  label(for='signup-pwconfirm')
  TextBox.textbox(
    id='signup-pwconfirm'
    v-model='pwConfirm'
    name='confirm'
    :error='!!pwConfirmError'
    placeholder='Confirm'
    type='password'
    autocomplete='new-password'
  )
  p.error(v-if='pwConfirmError === "empty"') Confirm your password
  p.error(v-if='pwConfirmError === "incorrect"') Passwords don't match

  Button.btn(:disabled='inProgress') Create account
</template>

<script>
import validator from 'validator'
import TextBox from '@/components/TextBox.vue'
import Button from '@/components/Button.vue'

export default {
  title: 'Sign up',
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
      pwConfirm: '',
      pwConfirmError: false,
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
      else if (this.password.length < 8) this.passwordError = 'too short'
      else if (this.password.length > 100) this.passwordError = 'too long'
    },
    validatePwConfirm: function () {
      this.pwConfirmError = false
      const pwConfirm = this.pwConfirm
      const pw = this.password
      if (validator.isEmpty(pwConfirm)) this.pwConfirmError = 'empty'
      else if (pwConfirm !== pw) this.pwConfirmError = 'incorrect'
    },
    signup: function (e) {
      if (this.inProgress === true) return
      this.pageErrorMsg = ''
      this.validateEmail()
      this.validatePassword()
      this.validatePwConfirm()
      if (this.emailError || this.passwordError || this.pwConfirmError) return
      this.inProgress = true
      this.$pocket.signup({
        email: this.email,
        password: this.password,
      }).then(() => {
        console.log('signup success')
        this.inProgress = false
        this.$router.push('/confirm')
      }, err => {
        if (err.msg === 'Server unreachable') {
          this.pageErrorMsg = 'Unable to reach server'
        } else if (err.msg === 'Input error' && err.error.email === 'exists') {
          this.emailError = 'exists'
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
.btn
  margin: 0px
  width: 100%
</style>
