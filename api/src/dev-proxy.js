const http = require('http')
const httpProxy = require('http-proxy')

httpProxy.createProxyServer({
  target: {
    protocol: 'http:',
    host: 'web',
  },
})

module.exports = {
  http: () => {
    const proxy = httpProxy.createProxyServer({
      target: {
        host: 'web',
        port: process.env.WEB_HTTP_PORT,
      },
      ws: true,
    })
    proxy.on('error', (err) => {
      $err(5009, 'dev-proxy server error', err)
    })
    return proxy
  },
  https: (httpsOptions) => {
    const proxy = httpProxy.createProxyServer({
      target: {
        host: 'web',
        port: process.env.WEB_HTTP_PORT,
      },
      ws: true,
      ssl: {
        key: httpsOptions.key,
        cert: httpsOptions.cert,
      },
    })
    proxy.on('error', (err) => {
      $err(5010, 'dev-proxy server error', err)
    })
    return proxy
  },
}
