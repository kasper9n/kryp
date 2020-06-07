const logger = require('./logger.js')
global.$err = logger.$err
global.$log = logger.$log
Error.stackTraceLimit = 30

async function main() {
  // mongoose
  const mongoose = require('mongoose')
  mongoose.set('useNewUrlParser', true)
  mongoose.set('useFindAndModify', false)
  mongoose.set('useCreateIndex', true)
  mongoose.set('useUnifiedTopology', true)
  const dbuser = process.env.DB_USERNAME
  const dbpass = process.env.DB_PASSWORD
  const dbPort = process.env.DB_PORT
  const url = `mongodb://${dbuser}:${dbpass}@db:${dbPort}/cryptrack`
  $log('Mongoose connection initiated...')
  await mongoose.connect(url, {}).then(() => {
    $log('Mongoose connected')
  }).catch(error => {
    $err(5001, 'Error when connecting to db', error)
  })
  mongoose.connection.on('error', err => {
    $err(5006, 'Mongoose error occured after initial connection', err)
  })

  // app
  const Koa = require('koa')
  const app = new Koa()

  app.use(async (ctx, next) => {
    console.log(`Request received: ${ctx.request.method} ${ctx.request.path}`)
    await next()
  })

  // response error handling
  app.use(logger.ctxErr)
  app.use(logger.ctxSuccess)
  app.use(logger.ctxAuthErr)

  // CORS fix
  const cors = require('@koa/cors')
  app.use(cors({ origin: '*' }))

  // body parser
  const bodyParser = require('koa-bodyparser')
  app.use(bodyParser())

  // session
  const session = require('koa-session')
  app.keys = [process.env.API_SESSION_KEY]
  const sessionConfig = {
    maxAge: 31557600000, // 365.25 days in ms
  }
  app.use(session(sessionConfig, app)) // maxAge

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

  // if koa's status is still the default, use the 404 response
  app.use(async (ctx, next) => {
    if (ctx.response.status === 404) ctx.$err(404, 'Not found')
    await next()
  })

  // Serve on https using a self-signed certificate.
  // - In development, this will show an error. Firefox lets you click past
  //   that error, so you can use that to check if the website works.
  // - In production, use a service like Cloudflare to provide encryption. Set
  //   the encryption mode to "Full" to allow self-signed certificates.
  const https = require('https')
  const selfSignedCert = require('./ssl/get-certificate.js')
  const httpsOptions = { key: selfSignedCert, cert: selfSignedCert }
  const httpsServer = https.createServer(httpsOptions, app.callback())
  httpsServer.listen(process.env.API_HTTPS_PORT)

  if (process.env.NODE_ENV === 'development') {
    // Serve http
    app.listen(process.env.API_HTTP_PORT)

    // proxy traffic to web service
    const proxy = require('./dev-proxy.js')
    proxy.http().listen(process.env.WEB_HTTP_PORT)
    proxy.https(httpsOptions).listen(process.env.WEB_HTTPS_PORT)
  }
}

main()
