import { Transactions } from '../lib/Transactions.js'
import { Wallets } from '../lib/Wallets.js'
export const transactions = Transactions.find({})
export const wallets = Wallets.find({})

import { format, parse } from 'date-fns'
export const dateFormat = 'yyyy/MM/dd h:mm a'
export function formatDate(date, dateFormat) {
  return format(date, dateFormat)
}
export function parseDate(date, dateFormat) {
  const referenceDate = new Date(1577833200000)
  return parse(date, dateFormat, referenceDate)
}
