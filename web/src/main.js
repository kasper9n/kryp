import Vue from 'vue'
import Chakra, {
  ThemeProvider,
  ColorModeProvider,
  Box,
  CSSReset,
} from '@chakra-ui/vue'
import App from '@/App.vue'
import router from '@/router'
import store from '@/store'

Vue.config.productionTip = false

Vue.use(Chakra)

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

new Vue({
  router,
  store,
  render: (h) => h(ThemeProvider, [
    h(ColorModeProvider, [h(Box, [h(CSSReset), h(App)])]),
  ]),
}).$mount('#app')
