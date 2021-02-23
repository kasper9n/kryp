import { Mongo } from 'meteor/mongo'
import { Meteor } from 'meteor/meteor'
import is from './is.ts'

export const Wallets = new Mongo.Collection('wallets')

function validateWallet(wallet) {
  const schema = {
    name: [is.string],
  }
  const vResult = is.valid(schema, wallet)
  if (vResult.errors) {
    throw new Meteor.Error(1000, 'Input error', vResult.errors)
  } else {
    return vResult.value
  }
}

Meteor.methods({
  'wallets.add'({ wallet: wallet }) {
    const finalTx = validateWallet(wallet)
    Wallets.insert(finalTx)
  },
  'wallets.edit'({ wallet: wallet, id }) {
    const finalTx = validateWallet(wallet)
    Wallets.update(
      { _id: id },
      finalTx,
    )
  },
  'wallets.delete'({ id }) {
    Wallets.remove(id)
  },
})
