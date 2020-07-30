const selfsigned = require('selfsigned')
const fs = require('fs')
const del = require('del')

const certOptions = {
  days: 365,
  algorithm: 'sha256',
  keySize: 2048,
  extensions: [
    {
      name: 'basicConstraints',
      cA: true,
    },
    {
      name: 'keyUsage',
      keyCertSign: true,
      digitalSignature: true,
      nonRepudiation: true,
      keyEncipherment: true,
      dataEncipherment: true,
    },
    {
      name: 'subjectAltName',
      altNames: [
        {
          // type 2 is DNS
          type: 2,
          value: 'localhost',
        },
        {
          type: 2,
          value: 'localhost.localdomain',
        },
        {
          type: 2,
          value: 'lvh.me',
        },
        {
          type: 2,
          value: '*.lvh.me',
        },
        {
          type: 2,
          value: '[::1]',
        },
        {
          // type 7 is IP
          type: 7,
          ip: '127.0.0.1',
        },
        {
          type: 7,
          ip: 'fe80::1',
        },
      ],
    },
  ],
}

const certificatePath = __dirname + '/certificate.pem'
let certificateExists = fs.existsSync(certificatePath)

if (certificateExists) {
  const certificateTtl = 1000 * 60 * 60 * 24
  const certificateStat = fs.statSync(certificatePath)

  const now = new Date()

  // cert is more than 30 days old, kill it with fire
  if ((now - certificateStat.ctime) / certificateTtl > 30) {
    console.log('SSL Certificate is more than 30 days old. Removing.')

    del.sync([certificatePath], { force: true })

    certificateExists = false
  }
}

if (!certificateExists) {
  console.log('Generating SSL Certificate')

  const attributes = [{ name: 'commonName', value: 'localhost' }]
  const pems = selfsigned.generate(attributes, certOptions)

  fs.writeFileSync(certificatePath, pems.private + pems.cert, {
    encoding: 'utf8',
  })
}

const certificate = fs.readFileSync(certificatePath)
module.exports = certificate
