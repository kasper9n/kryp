import { Mongo } from 'meteor/mongo'
import { Meteor } from 'meteor/meteor'
import validator from 'validator'
import Joi from 'joi'
import { is, validate } from './validate.js'

export const Transactions = new Mongo.Collection('transactions')

function validateTx(tx) {
  if (tx && tx.hash === '') delete tx.hash
  if (tx && tx.note === '') delete tx.note
  if (tx && tx.feeAmount === '') delete tx.feeAmount
  if (tx && tx.feeAsset === '') delete tx.feeAsset
  const schema = {
    type: [is.string, is.required],
    date: [is.string, is.required],
    hash: [is.string],
    note: [is.string],
    fromWallet: [is.string, is.required],
    fromAmount: [is.string, is.numeric, is.required],
    fromAsset: [is.string, is.required],
    feeAmount: [is.string, is.numeric],
    feeAsset: [is.string, is],
    toWallet: [is.string, is.required],
    toAmount: [is.string, is.numeric, is.required],
    toAsset: [is.string, is.required],
  }
  const vResult = validate(schema, tx)
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
