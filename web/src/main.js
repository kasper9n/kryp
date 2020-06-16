import Vue from 'vue'
import App from '@/App.vue'
import router from '@/router/index.js'
import VuePocket from '@/store/vuepocket.js'
import store from '@/store/index.js'

Vue.config.productionTip = false

const MetaPlugin = {
  install (Vue, { titleTemplate }) {
    if (!titleTemplate) titleTemplate = '%s'
    Vue.mixin({
      created () {
        if (this.$options.fullTitle) {
          document.title = this.$options.fullTitle
        } else if (this.$options.title) {
          document.title = titleTemplate.replace('%s', this.$options.title)
        }
      },
    })
  },
}
Vue.use(MetaPlugin, {
  titleTemplate: '%s - Cryptrack',
})

Vue.use(VuePocket, store)

Vue.prototype.$log = console.log

new Vue({
  router,
  render: (h) => h(App),
}).$mount('#app')
