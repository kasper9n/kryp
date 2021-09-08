import { writable } from 'svelte/store'
import { refresher, runCmd } from './general'

export type Transaction = {
  kind: 'Trade' | 'Transfer' | 'Deposit' | 'Withdrawal'
  date: number
  note: string
  hash: string
  sent_amount: string
  sent_asset: string
  sent_wallet: string
  recv_amount: string
  recv_asset: string
  recv_wallet: string
  fee_amount: string
  fee_asset: string
  cost: string
}

export const transactions = (() => {
  const store = writable([] as Transaction[])
  refresher.subscribe(async () => {
    const txs: Transaction[] = await runCmd('get_transactions')
    store.set(txs)
  })
  return {
    subscribe: store.subscribe,
  }
})()

function twoDigit(value: number) {
  return ('0' + value.toString()).slice(-2)
}
export function formatDateTime(date: Date) {
  return (
    date.getFullYear() +
    '-' +
    twoDigit(date.getMonth() + 1) +
    '-' +
    twoDigit(date.getDate()) +
    ' ' +
    twoDigit(date.getHours()) +
    ':' +
    twoDigit(date.getMinutes()) +
    ':' +
    twoDigit(date.getSeconds())
  )
}
export function formatDate(date: Date) {
  return date.getFullYear() + '-' + twoDigit(date.getMonth() + 1) + '-' + twoDigit(date.getDate())
}
export function formatTime(date: Date) {
  return (
    twoDigit(date.getHours()) +
    ':' +
    twoDigit(date.getMinutes()) +
    ':' +
    twoDigit(date.getSeconds())
  )
}
