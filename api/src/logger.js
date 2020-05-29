const red = '\x1b[31m'
const reset = '\x1b[0m'
function log(msg) {
  console.log(msg)
}

// code 1000-1999: user input error
// code 5000-5999: unexpected server error
function err(code, msg, err) {
  console.error(
    red + `Error E${code} ` + reset + msg + ':'
  )
  console.error(err || 'No error object provided')
}

async function ctxErr(ctx, next) {
  ctx.$err = (code, msg, error) => {
    errorType = String(500).charAt(0)
    if (errorType === 5) {
      err(code, msg, error)
      ctx.body = { code, msg: 'Server error' }
    } else if (errorType === 1) {
      ctx.body = { code, msg, error}
    }
  }
  await next()
}

module.exports = { log, err, ctxErr }
