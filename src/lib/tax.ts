import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
import { writable } from 'svelte/store'
import { refresher, popup } from './general'
import type { Transaction } from './transactions'

type Tax = {
  balances: any[]
  base_currency: string
  price_data: any
  realized_gains: any
  transactions: Transaction[]
}

export const tax = (() => {
  const defaultTax = {
    balances: [],
    base_currency: 'USD',
    transactions: [],
  }
  const store = writable(defaultTax as Tax)
  refresher.subscribe(() => {
    invoke('get_tax')
      .then((tax: Tax) => {
        console.log(tax)
        store.set(tax)
      })
      .catch(popup)
  })
  return {
    subscribe: store.subscribe,
  }
})()
