const red = '\x1b[31m'
const reset = '\x1b[0m'
function $log(msg) {
  console.log(msg)
}

// code 1000-1999: user input error
// code 5000-5999: unexpected server error
function $err(code, msg, err) {
  console.error(
    red + `Error E${code} ` + reset + msg + ':'
  )
  console.error(err || 'No error object provided')
}

async function ctxSuccess(ctx, next) {
  ctx.$success = (responseObject = {}) => {
    responseObject.error = null
    ctx.body = responseObject
  }
  await next()
}

async function ctxErr(ctx, next) {
  ctx.$err = (code, msg, error) => {
    errorType = String(code).charAt(0)
    if (errorType === '5') {
      $err(code, msg, error)
      ctx.body = { code, msg: 'Server error' }
    } else if (errorType === '1') {
      ctx.body = { code, msg, error}
    } else {
      $err(code, msg, error)
      ctx.body = { code, msg: 'Server error' }
      $err(5000, `Invalid error code ${code} in last error`, new Error())
    }
  }
  await next()
}

module.exports = { $log, $err, ctxSuccess, ctxErr }
