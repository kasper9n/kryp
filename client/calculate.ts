import { Transactions } from '../lib/Transactions.js'
import { Wallets } from '../lib/Wallets.js'

function isOneOf(value: any, ...matches: any) {
  return matches.includes(value)
}

export function calculate() {
  const walletsList = Wallets.find({}).fetch()
  const transactions = Transactions.find({}, { sort: { date: 1 } }).fetch()
  const wallets = {}
  for (const wallet of walletsList) {
    wallets[wallet.name] = {}
  }
  for (const tx of transactions) {
    if (isOneOf(tx.type, 'trade', 'transfer', 'withdrawal')) {
      const balances = wallets[tx.fromWallet]
      let balance = balances[tx.fromAsset] || 0
      balance -= Number(tx.fromAmount)
      balances[tx.fromAsset] = balance
    }
    if (isOneOf(tx.type, 'trade', 'transfer', 'deposit')) {
      const balances = wallets[tx.toWallet]
      let balance = balances[tx.toAsset] || 0
      balance += Number(tx.toAmount)
      balances[tx.toAsset] = balance
    }
  }
  console.log(wallets)
}

