const http = require('http')

function requestHandler(clientReq, clientRes) {
  // console.log('proxy serve: ' + clientReq.url)
  const options = {
    hostname: 'web',
    port: process.env.WEB_HTTP_PORT,
    path: clientReq.url,
    method: clientReq.method,
    headers: clientReq.headers,
  }
  const proxyReq = http.request(options, function (res) {
    clientRes.writeHead(res.statusCode, res.headers)
    res.pipe(clientRes, { end: true }).on('error', function (err) {
      $err(5011, 'Proxy request.pipe error (dev environment only)', err)
    })
  }).on('error', function (err) {
    $err(5009, 'Proxy request error (dev environment only)', err)
  })
  clientReq.pipe(proxyReq, { end: true }).on('error', function (err) {
    $err(5010, 'Proxy request.pipe error (dev environment only)', err)
  })
}

module.exports = {
  http: () => {
    return http.createServer(requestHandler)
  },
  https: (httpsOptions) => {
    return require('https').createServer(httpsOptions, requestHandler)
  },
}
