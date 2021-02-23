import { Accounts } from 'meteor/accounts-base'
import { Meteor } from 'meteor/meteor'
import { mError, is } from './general'

Accounts.onCreateUser((options, user: any) => {
  user.currency = 'USD'
  user.country = 'United States'
})

Meteor.methods({
  'accounts.create'({ email, password }) {

    const options = {
      email,
      password,
    }

    const schema = {
      email: [is.string, is.email],
      password: [is.string, is.min(2), is.max(100)],
    }
    const vResult = is.valid(schema, options)
    if (vResult.errors) throw mError(1000, 'Input error', vResult.errors)
    try {
      const userId = Accounts.createUser(options)
    } catch(err) {
      if (err.reason === 'Email already exists.') {
        throw mError(1000, 'Input error', { email: 'available' })
      }
    }
  },
  'accounts.edit'(options) {
    const schema = {
      currency: [is.string],
      country: [is.string],
    }
    const vResult = is.valid(schema, options)
    if (vResult.errors) throw mError(1000, 'Unknown')
    Meteor.users.update(Meteor.userId(), {
      $set: {
        currency: options.currency,
        country: options.country,
      },
    })
  },
})
