import { Transactions } from '../lib/Transactions.js'
import { Wallets } from '../lib/Wallets.js'
export const transactions = Transactions.find({})
export const wallets = Wallets.find({})
