import router from '@/router/index.js'

const $account = {
  loggedIn: false,
  email: null,
  firstName: null,
  lastName: null,
  login (redirectTo) {
    this.loggedIn = true
    this.email = 'example@gmail.com'
    this.firstName = 'Foo'
    this.lastName = 'Barson'
    router.push(redirectTo)
  },
  signup (redirectTo) {
    this.login(redirectTo)
  },
  logout (state) {
    this.loggedIn = false
    this.email = null
    this.firstName = null
    this.lastName = null
    router.push('/login')
  },
}

const $pocket = {
  isInAppArea: false,
}

export default {
  $account,
  $pocket,
}
