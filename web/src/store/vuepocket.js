// super simple vue store

// Usage example:
// - @/store/index.js:
//   ```
//   const user = {
//     email: 'example@beans.com',
//     updateEmail (newEmail) {
//       this.email = newEmail;
//     },
//   }
//
//   const modules = { user: user }
//   export default modules
//   ```
//
// - @/main.js:
//   ```
//   // ...
//   import VuePocket from 'vuepocket.js' // this file
//   import store from '@/store'
//   Vue.use(VuePocket, store)
//   // ...
//   ```
//
// - @/components/Example.vue
//   ```
//   <template>
//     <div>{{ $user.email }}</div>
//   </template>
//   // ...
//   ```
//
// - @/router/index.js
//   ```
//   // In other JavaScript files, simply access the store like so:
//   import store from '@/store'
//   store.$user.email = null
//   // ...
//   ```

function addModules (Vue, modules) {
  for (var key in modules) {
    if (!Object.prototype.hasOwnProperty.call(modules, key)) continue
    const module = modules[key]
    Vue.prototype[key] = Vue.observable(module)
  }
}

function install (Vue, options) {
  addModules(Vue, options)
}

export default {
  install,
}
