import Vue from 'vue'
import Vuex from 'vuex'
import router from '@/router/index.js'

Vue.use(Vuex)

const account = {
  namespaced: true,
  state: {
    loggedIn: false,
    email: null,
    firstName: null,
    lastName: null,
  },
  mutations: {
    login (state) {
      state.loggedIn = true
      state.email = 'example@gmail.com'
      state.firstName = 'Foo'
      state.lastName = 'Barson'
    },
    logout (state) {
      state.loggedIn = false
      state.email = null
      state.firstName = null
      state.lastName = null
    },
  },
  actions: {
    login ({ commit }, redirectTo) {
      commit('login')
      router.push(redirectTo)
    },
    logout ({ commit }) {
      commit('logout')
      router.push('/login')
    },
  },
  modules: {
  },
}

export default new Vuex.Store({
  state: {
    appArea: false,
  },
  mutations: {
    appArea (state, trueOrFalse) {
      state.appArea = trueOrFalse
    },
  },
  modules: {
    account,
  },
})
