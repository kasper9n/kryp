const mongoose = require("mongoose")
const Schema = mongoose.Schema
module.exports = {}

const portfolioSchema = new Schema({
  name: '',
  transactions: [],
}, {
  timestamps: true,
  toObject: {
    transform: (doc, ret, options) => {
      delete ret.__v
      ret.id = ret._id
      delete ret._id
      return ret
    }
  },
})
module.exports.Portfolio = mongoose.model('Portfolio', portfolioSchema)

const userSchema = new Schema({
  email: { type: String, required: true, unique: true },
  password: { type: String, required: true },
  portfolios: [portfolioSchema],
}, {
  timestamps: true,
  toObject: {
    transform: (doc, ret, options) => {
      delete ret.__v
      ret.id = ret._id
      delete ret._id
      delete ret.password
      return ret
    }
  },
})
module.exports.User = mongoose.model('User', userSchema)
