const red = '\x1b[31m'
const reset = '\x1b[0m'
function $log(msg) {
  console.log(msg)
}

// code 4000-4999: client error
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
    ctx.response.body = responseObject
  }
  await next()
}

async function ctxErr(ctx, next) {
  ctx.$err = (code, msg, error = true) => {
    errorType = String(code).charAt(0)
    if (errorType === '5') {
      $err(code, msg, error)
      ctx.response.body = { code, msg: 'Server error' }
      ctx.response.status = 500
    } else if (code === 404) {
      ctx.response.body = { code, msg, error }
      ctx.response.status = 404
    } else if (code === 401) {
      ctx.response.body = { code, msg, error }
      ctx.response.status = 401
    } else if (errorType === '4') {
      ctx.response.body = { code, msg, error }
      ctx.response.status = 400
    } else {
      $err(code, msg, error)
      ctx.response.body = { code, msg: 'Server error' }
      ctx.response.status = 500
      $err(5000, `Invalid error code ${code} in last error`, new Error())
    }
  }
  await next()
}

async function ctxAuthErr(ctx, next) {
  ctx.$authErr = () => {
    ctx.$err(401, 'Unauthorized')
  }
  await next()
}

module.exports = { $log, $err, ctxSuccess, ctxErr, ctxAuthErr }
