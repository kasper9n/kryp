import router from '@/router/index.js'

const $pocket = {
  isInAppArea: false,
  darkTheme: false,
  toggleDarkTheme () {
    this.darkTheme = !this.darkTheme
    this.updateTheme()
  },
  updateTheme () {
    const dataset = document.querySelector('html').dataset
    if (this.darkTheme === true) dataset.theme = 'dark'
    else dataset.theme = 'light'
  },
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
    if (redirectTo) router.push(redirectTo)
  },
  signup (redirectTo) {
    this.login(redirectTo)
  },
  logout (callback) {
    this.loggedIn = false
    this.email = null
    this.firstName = null
    this.lastName = null
    if (callback) callback()
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
