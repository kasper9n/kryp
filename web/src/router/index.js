import Vue from 'vue'
import VueRouter from 'vue-router'
import store from '@/store'
import Home from '@/views/Home.vue'
import Login from '@/views/Login.vue'
import Signup from '@/views/Signup.vue'
import Confirm from '@/views/Confirm.vue'
import PageNotFound from '@/views/404.vue'
import Dashboard from '@/views/portfolio/Dashboard.vue'
import Transactions from '@/views/portfolio/Transactions.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: Home,
  },
  {
    path: '/signup',
    name: 'signup',
    component: Signup,
  },
  {
    path: '/confirm',
    name: 'confirm',
    component: Confirm,
  },
  {
    path: '/login',
    name: 'login',
    component: Login,
  },
  {
    path: '/portfolio/:portfolioId',
    name: 'dashboard',
    component: Dashboard,
    meta: { login: true },
  },
  {
    path: '/portfolio/:portfolioId/transactions',
    name: 'transactions',
    component: Transactions,
    meta: { login: true },
  },
  {
    path: '*',
    name: '404',
    component: PageNotFound,
  },
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
})

const guards = [
  (to, from, next) => {
    // logout path
    if (to.path === '/logout') {
      store.$pocket.logout().then(() => {
        console.log('logout success')
        next({ path: '/login', replace: true })
      }, err => {
        if (err.msg === 'Server unreachable') {
          console.log('server unreachable')
          store.$pocket.pageErrorMsg = 'Unable to reach server'
        } else if (err.msg === 'Unauthorized') {
          // we were already logged out, so treat that as normal
          next({ path: '/login', replace: true })
        } else {
          store.$pocket.pageErrorMsg = `Unexpected error: ${err.code} ${err.msg}`
        }
      })
    }
  },
  (to, from, next) => {
    // redirect to login if necessary
    if (to.meta.login === true && store.$pocket.loggedIn !== true) {
      store.$pocket.init().then(() => {
        console.log('init success')
      }, err => {
        if (err.msg === 'Server unreachable') {
          console.log('server unreachable')
          store.$pocket.pageErrorMsg = 'Unable to reach server'
        } else if (err.msg === 'Unauthorized') {
          next({ path: '/login', replace: true, query: { continue: to.path } })
        } else {
          store.$pocket.pageErrorMsg = `Unexpected error: ${err.code} ${err.msg}`
        }
      })
    }
  },
  (to, from, next) => {
    // if url has portfolioId param, update id in store or change to valid portfolio
    if (to.params.portfolioId) {
      if (store.$pocket.idExists(to.params.portfolioId)) {
        store.$pocket.setPortfolio(to.params.portfolioId)
      } else {
        next({
          name: to.name,
          params: { portfolioId: store.$pocket.currentId },
        })
      }
    }
  },
]
router.beforeEach((to, from, next) => {
  let done = false
  function nextWrapper (arg) {
    next(arg)
    done = true
  }

  for (const guard of guards) {
    if (done) break
    guard(to, from, nextWrapper)
  }
  next()
})

export default router
