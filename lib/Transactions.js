import { Mongo } from 'meteor/mongo'
import { Meteor } from 'meteor/meteor'
import validator from 'validator'
import Joi from 'joi'

export const Transactions = new Mongo.Collection('transactions')

function validate(tx) {
  function numerical(value, helpers) {
    if (validator.isNumeric(value)) return value
    else return helpers.error('string.invalid')
  }

  if (tx && tx.hash === '') delete tx.hash
  if (tx && tx.note === '') delete tx.note
  if (tx && tx.feeAmount === '') delete tx.feeAmount
  if (tx && tx.feeAsset === '') delete tx.feeAsset
  const schema = Joi.object({
    type: Joi.string().required(),
    date: Joi.string().required(),
    hash: Joi.string(),
    note: Joi.string(),
    fromWallet: Joi.string().required(),
    fromAmount: Joi.string().custom(numerical).required(),
    fromAsset: Joi.string().required(),
    feeAmount: Joi.string().custom(numerical),
    feeAsset: Joi.string(),
    toWallet: Joi.string().required(),
    toAmount: Joi.string().custom(numerical).required(),
    toAsset: Joi.string().required(),
  })
  const vResult = schema.validate(tx, {
    stripUnknown: true,
    abortEarly: false,
  })
  if (vResult.error) throw new Meteor.Error(1000, 'Input error', vResult.error.details)
  else return vResult.value
}

Meteor.methods({
  'transactions.add'({ transaction: tx }) {
    const finalTx = validate(tx)
    Transactions.insert(finalTx)
  },
  'transactions.edit'({ transaction: tx, id }) {
    const finalTx = validate(tx)
    Transactions.update(
      { _id: id },
      finalTx,
    )
  },
  'transactions.delete'({ id }) {
    Transactions.remove(id)
  },
})
