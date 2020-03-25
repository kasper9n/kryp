import router from '@/router/index.js'

const $pocket = {
  isInAppArea: false,
}

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

const $portfolios = {
  portfolios: [
    {
      id: '2n8pgyqnvq',
      name: 'Main',
    },
    {
      id: '464fuh3na3',
      name: 'Crypto',
    },
    {
      id: '9ggukb7jtk',
      name: 'Stocks and stuff',
    },
  ],
}

export default {
  $pocket,
  $account,
  $portfolios,
}
