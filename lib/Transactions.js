import { Mongo } from 'meteor/mongo'
import { Meteor } from 'meteor/meteor'
import is from './is.ts'

export const Transactions = new Mongo.Collection('transactions')

function validateTx(tx) {
  if (tx && tx.hash === '') delete tx.hash
  if (tx && tx.note === '') delete tx.note
  if (tx && tx.feeAmount === '') delete tx.feeAmount
  if (tx && tx.feeAsset === '') delete tx.feeAsset
  const schema = {
    type: [is.string],
    date: [is.integer],
    hash: [is.string, is.optional],
    note: [is.string, is.optional],
    fromWallet: [is.string],
    fromAmount: [is.string, is.numeric],
    fromAsset: [is.string],
    feeAmount: [is.string, is.numeric, is.optional],
    feeAsset: [is.string, is.optional],
    toWallet: [is.string],
    toAmount: [is.string, is.numeric],
    toAsset: [is.string],
  }
  const vResult = is.valid(schema, tx)
  if (vResult.errors) {
    throw new Meteor.Error(1000, 'Input error', vResult.errors)
  } else {
    return vResult.value
  }
}

Meteor.methods({
  'transactions.add'({ transaction: tx }) {
    const finalTx = validateTx(tx)
    Transactions.insert(finalTx)
  },
  'transactions.edit'({ transaction: tx, id }) {
    const finalTx = validateTx(tx)
    Transactions.update(
      { _id: id },
      finalTx,
    )
  },
  'transactions.delete'({ id }) {
    Transactions.remove(id)
  },
})
