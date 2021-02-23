import { Accounts } from 'meteor/accounts-base'
import { Meteor } from 'meteor/meteor'
import { mError, is } from './general'

Meteor.methods({
  'accounts.create'({ email, password }) {

    const options = { email, password }

    const schema = {
      email: [is.string, is.email],
      password: [is.string, is.min(2), is.max(100)],
    }
    const vResult = is.valid(schema, options)
    if (vResult.errors) {
      throw mError(1000, 'Input error', vResult.errors)
    } else {
      try {
        const userId = Accounts.createUser(options)
        return
      } catch(err) {
        if (err.reason === 'Email already exists.') {
          throw mError(1000, 'Input error', { email: 'available' })
        }
      }
    }
  },
})
