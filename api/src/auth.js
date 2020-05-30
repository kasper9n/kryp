const passport = require('koa-passport')
const User = require("./mongoose-models.js").User
const bcrypt = require("bcryptjs")

passport.serializeUser((user, done) => {
  done(null, user._id)
})

passport.deserializeUser(async function(id, done) {
  try {
    const user = await User.findById(id)
    done(null, user)
  } catch (error) {
    done(error)
  }
})

const LocalStrategy = require('passport-local').Strategy
passport.use(new LocalStrategy({
    usernameField: 'email',
    passwordField: 'password',
  }, function(email, password, done) {
    User.findOne({ email: email }, function(err, resultUser) {
      if (err) return done(error)
      else if (!resultUser) return done(null, false, 'email incorrect')
      // match password
      bcrypt.compare(password, resultUser.password, (err, isMatch) => {
        if (err) return done(err)
        else if (isMatch) return done(null, resultUser)
        else return done(null, false, 'password incorrect')
      })
    })
  }
))
