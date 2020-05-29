const Router = require('@koa/router')
const router = new Router()
const validator = require('validator')
const User = require("./../mongoose-models.js").User

async function userExists(email) {
  try {
    const user = await User.findOne({ email: email })
    if (user) return true
    else return false
    console.log(user)
  } catch (error) {
    console.log(error)
  }
}

router.post('/register', async (ctx, next) => {
  console.log(ctx.request.body)
  let email = ctx.request.body.email
  const password = ctx.request.body.password
  const errors = []

  if      (typeof email !== 'string') errors.push('email empty')
  else if (validator.isEmpty(email)) errors.push('email empty')
  else if (!validator.isEmail(email)) errors.push('email invalid')

  if      (typeof password !== 'string') errors.push('password empty')
  else if (validator.isEmpty(password)) errors.push('password empty')
  else if (!validator.isLength(password, {min: 8, max: 100})) errors.push('email invalid')

  email = validator.normalizeEmail

  try {
    if (await userExists(email)) errors.push('user exists')
  } catch (error) {
    ctx.$err(5003, 'Error checking if user exists', error)
  }

  if (errors.length !== 0) ctx.$err(1001, 'Input error', errors)

  // try {
  //   await new User({ email, password }).save()
  //   console.log('created new user')
  // } catch (error) {
  //   ctx.$err(5004, 'Error creating new user', error)
  // }
})

module.exports = router
