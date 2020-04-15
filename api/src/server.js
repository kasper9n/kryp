const Koa = require('koa')
const json = require('koa-json')
const mongoose = require('mongoose')

const dbuser = process.env.DB_USERNAME
const dbpass = process.env.DB_PASSWORD
mongoose.connect(
  `mongodb://${dbuser}:${dbpass}@db:27017/admin`,
  { useNewUrlParser: true, useUnifiedTopology: true }
).then(() => {
  console.log('Connected to db')
})

const app = new Koa()
app.use(json())
 
app.use(ctx => {
  ctx.body = { text: 'Hello Koa' }
})
 
app.listen(80)
