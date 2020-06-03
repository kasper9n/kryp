module.exports = {
  devServer: {
    // Allow other hosts than localhost. Useful for pointing a CloudFlare
    // domain to your local network, and enabling port forwarding, whether
    // it's for sharing the website or testing if HTTPS works as expected.
    disableHostCheck: true,
    // Disable progress output because docker-compose causes a 90,000 character
    // line to be logged.
    progress: false,
  },
}
