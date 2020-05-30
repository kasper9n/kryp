const Router = require('@koa/router')
const router = new Router()
const validator = require('validator')
const bcrypt = require("bcryptjs")
const User = require("./../mongoose-models.js").User

router.post('/register', async (ctx, next) => {
  let email = ctx.request.body.email
  const password = ctx.request.body.password
  const errors = []

  if      (typeof email !== 'string') errors.push('email empty')
  else if (validator.isEmpty(email)) errors.push('email empty')
  else if (!validator.isEmail(email)) errors.push('email invalid')

  if      (typeof password !== 'string') errors.push('password empty')
  else if (validator.isEmpty(password)) errors.push('password empty')
  else if (!validator.isLength(password, {min: 8})) errors.push('password too short')
  else if (!validator.isLength(password, {max: 100})) errors.push('password too long')

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

module.exports = router
