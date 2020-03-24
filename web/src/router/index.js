import Vue from 'vue'
import VueRouter from 'vue-router'
import store from '@/store/index.js'
import Home from '@/views/Home.vue'
import Login from '@/views/Login.vue'
import Signup from '@/views/Signup.vue'
import PageNotFound from '@/views/404.vue'
import Dashboard from '@/views/app/Dashboard.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    component: Home,
  },
  {
    path: '/signup',
    component: Signup,
  },
  {
    path: '/login',
    component: Login,
  },
  {
    path: '/app',
    redirect: '/app/dashboard',
  },
  {
    path: '/app/dashboard',
    component: Dashboard,
  },
  {
    path: '*',
    component: PageNotFound,
  },
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
})

router.beforeEach((to, from, next) => {
  if (to.path.startsWith('/app')) {
    if (store.state.account.loggedIn) {
      store.commit('appArea', true)
      next()
    } else {
      next({ path: '/login', replace: true, query: { continue: to.path } })
    }
  } else {
    store.commit('appArea', false)
    next()
  }
})

export default router
