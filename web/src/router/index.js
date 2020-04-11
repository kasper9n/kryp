import Vue from 'vue'
import VueRouter from 'vue-router'
import store from '@/store'
import Home from '@/views/Home.vue'
import Login from '@/views/Login.vue'
import Signup from '@/views/Signup.vue'
import PageNotFound from '@/views/404.vue'
import Dashboard from '@/views/app/Dashboard.vue'
import Transactions from '@/views/app/Transactions.vue'

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
    path: '/login',
    name: 'login',
    component: Login,
  },
  {
    path: '/app',
    redirect: {
      name: 'dashboard',
      params: { portfolioId: store.$portfolios.currentId },
    },
  },
  {
    path: '/app/p/:portfolioId/dashboard',
    name: 'dashboard',
    component: Dashboard,
  },
  {
    path: '/app/p/:portfolioId/transactions',
    name: 'transactions',
    component: Transactions,
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
    if (to.path === '/app/logout') {
      store.$account.logout(() => {
        next({ path: '/login', replace: true })
      })
    }
  },
  (to, from, next) => {
    // redirect to login if necessary, and set $pocket.isInAppArea
    if (to.path.startsWith('/app')) {
      if (store.$account.loggedIn) {
        store.$pocket.isInAppArea = true
      } else {
        next({ path: '/login', replace: true, query: { continue: to.path } })
      }
    } else {
      store.$pocket.isInAppArea = false
    }
  },
  (to, from, next) => {
    // if url has portfolioId param, update id in store or change to valid portfolio
    if (to.params.portfolioId) {
      if (store.$portfolios.idExists(to.params.portfolioId)) {
        store.$portfolios.setPortfolio(to.params.portfolioId)
      } else {
        next({
          name: to.name,
          params: { portfolioId: store.$portfolios.currentId },
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
