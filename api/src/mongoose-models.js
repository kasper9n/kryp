const mongoose = require("mongoose")
const Schema = mongoose.Schema
module.exports = {}

async function generateId(count, k) {
  const symbols = 'abcdefghjkmnpqrtuvwxyz1234567890'
  let str = '';

  for(var i = 0; i < count; i++) {
    str += symbols[parseInt(Math.random() * (symbols.length))];
  }

  return str
  // module.exports.User find id, if it exists, loop

  // base.getID(str, function(err, res) {
  //   if(!res.length) {
  //     k(str) // use the continuation
  //   } else generate(count, k) // otherwise, recurse on generate
  // });
}



const portfolioSchema = new Schema({
  name: '',
  transactions: [],
}, { timestamps: true })
module.exports.Portfolio = mongoose.model('Portfolio', portfolioSchema)

const userSchema = new Schema({
  email: String,
  password: String,
  portfolios: [portfolioSchema],
}, { timestamps: true })
module.exports.User = mongoose.model('User', userSchema)
