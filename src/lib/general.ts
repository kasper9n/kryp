import { invoke } from '@tauri-apps/api/tauri'
import { writable } from 'svelte/store'

export const refresher = writable(1) // 1 so it's always truthy
export function refresh() {
  refresher.update((v) => v + 1)
}

export function popup(msg: string) {
  invoke('error_popup', { msg })
}

let lastActiveElement = document.body
export function focus(el: HTMLElement) {
  if (document.activeElement instanceof HTMLElement) {
    lastActiveElement = document.activeElement
    el.focus()
  }
}
export function focusLast() {
  if (lastActiveElement) lastActiveElement.focus()
}
