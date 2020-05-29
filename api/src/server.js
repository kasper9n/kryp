const logger = require('./logger.js')
global.$err = logger.err
global.$log = logger.log

async function main() {
  // mongoose
  const mongoose = require('mongoose')
  const dbuser = process.env.DB_USERNAME
  const dbpass = process.env.DB_PASSWORD
  const url = `mongodb://${dbuser}:${dbpass}@db:27017/cryptrack`
  const options = { useNewUrlParser: true, useUnifiedTopology: true }
  $log('Mongoose connection initiated...')
  await mongoose.connect(url, options).then(() => {
    $log('Mongoose connection success')
  }).catch(error => {
    $err(5001, 'Error when connecting to db', error)
  })

  // app
  const Koa = require('koa')
  const app = new Koa()

  app.use(async (ctx, next) => {
    console.log('Request received')
    await next()
  })

  // response error handling
  app.use(logger.ctxErr)

  // body parser
  const bodyParser = require('koa-bodyparser')
  app.use(bodyParser())

  // session
  const session = require('koa-session')
  app.keys = [process.env.API_SESSION_KEY]
  app.use(session({}, app))

  // auth
  require('./auth.js')
  const passport = require('koa-passport')
  app.use(passport.initialize())
  app.use(passport.session())

  // json
  const json = require('koa-json')
  app.use(json())

  // routes
  const userRouter = require('./routes/user.js')
  app.use(userRouter.routes())
  app.use(userRouter.allowedMethods())

  // serve
  app.listen(80)
}

// log unhandled errors
main().catch((error) => {
  $err(5002, 'Uncaught error', error)
})
