type Tag = {
  type: Transaction['type']
  value: string
  name: string
}
export const tags: Tag[] = [
  { type: 'Trade', value: 'Trade', name: 'Trade' },
  { type: 'Transfer', value: 'Transfer', name: 'Transfer' },
  { type: 'Deposit', value: 'Deposit', name: 'Deposit' },
  { type: 'Deposit', value: 'Gift', name: 'Gift' },
  { type: 'Deposit', value: 'Interest', name: 'Interest' },
  { type: 'Withdrawal', value: 'Withdrawal', name: 'Withdrawal' },
  { type: 'Withdrawal', value: 'Spend', name: 'Spend' },
  { type: 'Withdrawal', value: 'Lost', name: 'Lost' },
]

export type Trade = {
  type: 'Trade'
  tag: string
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
  manual_worth: string | null
  cost: string
}

export type Transfer = {
  type: 'Transfer'
  tag: string
  date: number
  note: string
  hash: string
  sent_amount: string
  sent_asset: string
  sent_wallet: string
  recv_amount: string
  recv_asset: string
  recv_wallet: string
  manual_worth: string | null
  cost: string
}

export type Deposit = {
  type: 'Deposit'
  tag: string
  date: number
  note: string
  hash: string
  amount: string
  asset: string
  wallet: string
  manual_worth: string | null
  cost: string
}

export type Withdrawal = {
  type: 'Withdrawal'
  tag: string
  date: number
  note: string
  hash: string
  amount: string
  asset: string
  wallet: string
  manual_worth: string | null
  cost: string
}

export type Transaction = Trade | Transfer | Deposit | Withdrawal

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
