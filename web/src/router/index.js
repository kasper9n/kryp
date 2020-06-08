import Vue from 'vue'
import VueRouter from 'vue-router'
import store from '@/store'
import Home from '@/views/Home.vue'
import Login from '@/views/Login.vue'
import Signup from '@/views/Signup.vue'
import Confirm from '@/views/Confirm.vue'
import Overview from '@/views/Overview.vue'
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
    path: '/overview',
    name: 'overview',
    component: Overview,
    meta: { login: true },
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

router.beforeEach((to, from, next) => {
  // logout path
  if (to.path === '/logout') {
    store.$pocket.logout().then(() => {
      console.log('logout success')
      next({ path: '/login', replace: true })
    }, err => {
      if (err.msg === 'Server unreachable') {
        console.log('server unreachable')
        store.$pocket.setErrorMsg('Unable to reach server')
        next(false)
      } else if (err.msg === 'Unauthorized') {
        // we were already logged out, so treat that as normal
        next({ path: '/login', replace: true })
      } else {
        store.$pocket.setErrorMsg(`Unexpected error: ${err.code} ${err.msg}`)
        next(false)
      }
    })
  } else {
    next()
  }
})

router.beforeEach((to, from, next) => {
  // redirect to login if necessary
  if (to.meta.login === true && store.$pocket.loggedIn !== true) {
    store.$pocket.init().then(() => {
      console.log('init success')
      next()
    }, err => {
      if (err.msg === 'Server unreachable') {
        console.log('server unreachable')
        store.$pocket.setErrorMsg('Unable to reach server')
        next(false)
      } else if (err.msg === 'Unauthorized') {
        next({ path: '/login', replace: true, query: { continue: to.path } })
      } else {
        store.$pocket.setErrorMsg(`Unexpected error: ${err.code} ${err.msg}`)
        next(false)
      }
    })
  } else {
    next()
  }
})

router.beforeEach((to, from, next) => {
  // if url has portfolioId param, update id in store or change to valid portfolio
  if (to.params.portfolioId) {
    if (store.$pocket.getPortfolio(to.params.portfolioId)) {
      store.$pocket.setPortfolioId(to.params.portfolioId)
      next()
    } else {
      next({ path: '/overview', replace: true })
    }
  } else {
    next()
  }
})

export default router
