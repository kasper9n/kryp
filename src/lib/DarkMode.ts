import { writable } from 'svelte/store'

const prefersDarkMQ = matchMedia('(prefers-color-scheme: dark)')
export const darkMode = writable(prefersDarkMQ.matches)

function handler(e: { matches: boolean }) {
  darkMode.set(e.matches)
}
// new onchange/addEventListenr api not supported in macOS Catalina
if (prefersDarkMQ.addListener) {
  prefersDarkMQ.addListener(handler)
}
