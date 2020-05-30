const mongoose = require("mongoose")
const Schema = mongoose.Schema
module.exports = {}

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
