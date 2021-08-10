import { invoke } from '@tauri-apps/api/tauri'
import { writable } from 'svelte/store'
import { refresher, popup } from './general'
import type { Transaction } from './transactions'

export type PriceDataAsset = {
  symbol: string
  kind: string
  interval: string
  prices: Map<number, number>
}
export type PriceData = {
  assets: Map<string, PriceDataAsset>
}

type Tax = {
  balances: any[]
  base_currency: string
  price_data: any
  realized_gains: any
  transactions: Transaction[]
}

const defaultTax = {
  balances: [],
  base_currency: 'USD',
  transactions: [],
}
export const tax = writable(defaultTax as Tax)

type Data = {
  opened: boolean
}

export const opened = writable(false)

refresher.subscribe(() => {
  invoke('get_tax')
    .then((v: Tax) => {
      console.log(v)
      tax.set(v)
    })
    .catch(popup)
  invoke('get_data')
    .then((data: Data) => {
      opened.set(data.opened)
    })
    .catch(popup)
})
