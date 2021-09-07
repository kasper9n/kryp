import { invoke } from '@tauri-apps/api/tauri'
import { writable } from 'svelte/store'
import { refresher, popup } from './general'
import type { Transaction } from './transactions'

type Tax = {
  transactions: Transaction[]
  base_currency: string
  price_data: PriceData
  realized_gains: Realized[]
  deposits: Deposit[]
  balances: Balance[]
  tags: string[]
}

const defaultTax = {
  transactions: [],
  base_currency: 'USD',
  price_data: {
    assets: {},
  },
  realized_gains: [],
  deposits: [],
  balances: [],
  tags: [],
}
export const tax = writable(defaultTax as Tax)

export type PriceData = {
  assets: Map<string, PriceDataAsset>
}
export type PriceDataAsset = {
  symbol: string
  kind: string
  interval: string
  prices: Map<number, number>
}

export type Balance = {
  acquire_date: number
  amount: string
  currency: string
  wallet: string
  cost: string
}

export type Realized = {
  date: number
  input: string
  output: string
  wallet: string
}

export type Deposit = {
  date: number
  amount: string
  currency: string
  value: string
  wallet: string
}

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
