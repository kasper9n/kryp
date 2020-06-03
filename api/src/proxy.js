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
  const proxy = http.request(options, function (res) {
    clientRes.writeHead(res.statusCode, res.headers)
    res.pipe(clientRes, { end: true })
  })
  clientReq.pipe(proxy, { end: true })
}

module.exports = {
  http: () => {
    return http.createServer(requestHandler)
  },
  https: (httpsOptions) => {
    return require('https').createServer(httpsOptions, requestHandler)
  },
}
