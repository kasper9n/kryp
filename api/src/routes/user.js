const Router = require('@koa/router')
const router = new Router()
const validator = require('validator')
const bcrypt = require("bcryptjs")
const passport = require('koa-passport')
const User = require("./../mongoose-models.js").User

router.post('/register', async (ctx, next) => {
  let email = ctx.request.body.email
  const password = ctx.request.body.password
  const error = {}
  function hasError() {
    if (Object.keys(error).length !== 0) return true
  }

  if      (typeof email !== 'string') error.email = 'empty'
  else if (validator.isEmpty(email)) error.email = 'empty'
  else if (!validator.isEmail(email)) error.email = 'invalid'

  if      (typeof password !== 'string') error.password = 'empty'
  else if (validator.isEmpty(password)) error.password = 'empty'
  else if (!validator.isLength(password, {min: 8})) error.password = 'too short'
  else if (!validator.isLength(password, {max: 100})) error.password = 'too long'

  if (hasError()) return ctx.$err(4001, 'Input error', error)

  email = validator.normalizeEmail(email)

  try {
    const user = await User.findOne({ email: email })
    if (user) error.email = 'exists'
  } catch (error) {
    return ctx.$err(5003, 'Error checking if user exists', error)
  }

  if (hasError()) return ctx.$err(4006, 'Input error', error)

  function generateHash(password) {
    return new Promise((resolve, reject) => {
      bcrypt.genSalt(10, (err, salt) => {
        if (err) return reject(err);
        bcrypt.hash(password, salt, (err, hashedPassword) => {
          if (err) return reject(err);
          resolve(hashedPassword);
        });
      });
    })
  }

  let hashedPassword
  try {
    hashedPassword = await generateHash(password)
  } catch (err) {
    return ctx.$err(5005, `Error hashing password or generating salt`, err)
  }

  try {
    await new User({
      email,
      password: hashedPassword,
      portfolios: [
        {
          id: '2n8pgyqnvq',
          name: 'Main',
        },
        {
          id: '464fuh3na3',
          name: 'Crypto',
          transactions: [
            {
              type: 'Trade',
              buy: '0.00000001',
              buyAsset: 'BTC',
              sell: '1.00000000',
              sellAsset: 'USD',
              fee: '0.01000000',
              feeAsset: 'USD',
              account: 'Binance',
              time: '2020-03-11 02:32:11'
            }, {
              type: 'Deposit',
              buy: '0.00000001',
              buyAsset: 'BTC',
              sell: '',
              sellAsset: '',
              fee: '0.01000000',
              feeAsset: 'USD',
              account: 'Binance',
              time: '2019-08-21 11:23:46'
            }, {
              type: 'External Buy',
              buy: '0.00000001',
              buyAsset: 'BTC',
              sell: '',
              sellAsset: '',
              fee: '0.01000000',
              feeAsset: 'USD',
              account: 'Binance',
              time: '2019-08-21 11:23:46'
            },
          ],
        },
        {
          id: '9ggukb7jtk',
          name: 'Stocks and stuff haha',
        },
      ],
    }).save()
  } catch (err) {
    return ctx.$err(5004, `Error creating new user '${email}'`, err)
  }

  return ctx.$success()
})

router.post('/login', async (ctx, next) => {
  let email = ctx.request.body.email
  const password = ctx.request.body.password

  if      (typeof email !== 'string') return ctx.$err(4002, 'Email empty')
  else if (validator.isEmpty(email)) return ctx.$err(4003, 'Email empty')
  else if (typeof password !== 'string') return ctx.$err(4004, 'Password empty')
  else if (validator.isEmpty(password)) return ctx.$err(4005, 'Password empty')

  email = validator.normalizeEmail(email)

  let resultUser
  try {
    resultUser = await User.findOne({ email: email })
  } catch (err) {
    return ctx.$err(5007, 'Error checking if user exists', err)
  }
  if (!resultUser) return ctx.err(4007, 'Login incorrect')

  return passport.authenticate('local', { session: false }, function(err, user, info) {
    if (err) {
      return ctx.$err(5008, 'Error authenticating user', err)
    } else if (user === false) {
      return ctx.$err(4007, 'Login incorrect')
    } else {
      ctx.$success({
        user: {
          email: user.email,
          portfolios: user.portfolios,
        },
      })
      return ctx.login(user)
    }
  })(ctx)

})

router.post('/logout', async (ctx, next) => {
  if (!ctx.isAuthenticated()) return ctx.$authErr()
  ctx.logout()
  return ctx.$success()
})

router.post('/me', async (ctx, next) => {
  if (!ctx.isAuthenticated()) return ctx.$authErr()
  return ctx.$success({
    user: ctx.state.user.toObject()
  })
})

module.exports = router
