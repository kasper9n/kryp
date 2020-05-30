const Router = require('@koa/router')
const router = new Router()
const validator = require('validator')
const bcrypt = require("bcryptjs")
const passport = require('koa-passport')
const User = require("./../mongoose-models.js").User

router.post('/register', async (ctx, next) => {
  let email = ctx.request.body.email
  const password = ctx.request.body.password
  const errors = []

  if      (typeof email !== 'string') errors.push('Email empty')
  else if (validator.isEmpty(email)) errors.push('Email empty')
  else if (!validator.isEmail(email)) errors.push('Email invalid')

  if      (typeof password !== 'string') errors.push('Password empty')
  else if (validator.isEmpty(password)) errors.push('Password empty')
  else if (!validator.isLength(password, {min: 8})) errors.push('Password too short')
  else if (!validator.isLength(password, {max: 100})) errors.push('Password too long')

  email = validator.normalizeEmail(email)

  try {
    const user = await User.findOne({ email: email })
    if (user) errors.push('user exists')
  } catch (error) {
    return ctx.$err(5003, 'Error checking if user exists', error)
  }

  if (errors.length !== 0) return ctx.$err(1001, 'Input error', errors)

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
  } catch (error) {
    return ctx.$err(5005, `Error hashing password or generating salt`, error)
  }

  try {
    await new User({
      email,
      password: hashedPassword,
      }).save()
  } catch (error) {
    return ctx.$err(5004, `Error creating new user '${email}'`, error)
  }

  return ctx.$success()
})

router.post('/login', async (ctx, next) => {
  let email = ctx.request.body.email
  const password = ctx.request.body.password

  if      (typeof email !== 'string') ctx.$err(1002, 'Email empty')
  else if (validator.isEmpty(email)) ctx.$err(1003, 'Email empty')
  else if (typeof password !== 'string') ctx.$err(1004, 'Password empty')
  else if (validator.isEmpty(password)) ctx.$err(1005, 'Password empty')

  email = validator.normalizeEmail(email)

  let resultUser
  try {
    resultUser = await User.findOne({ email: email })
  } catch (error) {
    return ctx.$err(5007, 'Error checking if user exists', error)
  }
  if (!resultUser) return ctx.err(5008, 'Email incorrect')

  return passport.authenticate('local', function(err, user, info) {
    if (err) {
      return ctx.$err(5009, 'Error authenticating user', err)
    } else if (user === false) {
      return ctx.$err(5010, 'Authentication failed')
    } else {
      ctx.$success()
      return ctx.login(user)
    }
  })(ctx)

})

router.post('/logout', async (ctx, next) => {
  ctx.logout()
  return ctx.$success()
})

module.exports = router
