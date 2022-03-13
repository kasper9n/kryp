import { writable } from 'svelte/store'
import { refresher, runCmd } from '$lib/general'
import type { Transaction } from '$lib/transactions'

type Tax = {
  transactions: Transaction[]
  settings: TaxSettings
  price_data: PriceData
  realized_gains: Realized[]
  deposits: Deposit[]
  balances: Balance[]
}

export type TaxSettings = {
  base_currency: string
  apis: { name: string; key?: string; disabled: boolean }[]
}

const defaultTax: Tax = {
  transactions: [],
  settings: {
    base_currency: 'USD',
    apis: [],
  },
  price_data: {
    assets: {},
  },
  realized_gains: [],
  deposits: [],
  balances: [],
}
export const tax = writable(defaultTax)

export type PriceData = {
  assets: { [key: string]: PriceDataAsset }
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

refresher.subscribe(async () => {
  const newTax: Tax = await runCmd('get_tax')
  console.log(newTax)
  tax.set(newTax)

  const data: Data = await runCmd('get_data')
  opened.set(data.opened)
})
