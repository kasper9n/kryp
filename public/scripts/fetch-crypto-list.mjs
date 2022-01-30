import https from 'https'
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

function get(url) {
  return new Promise((resolve, reject) => {
    const req = https.get(url, (res) => {
      let data = ''
      res.on('data', (d) => {
        data += d
      })
      res.on('end', () => {
        if (
          res.statusCode !== 200 ||
          res.headers['content-type'] !== 'application/json; charset=utf-8'
        ) {
          console.log('statusCode:', res.statusCode)
          console.log('headers:', res.headers)
          console.log(data)
          reject('Unexpected response code or content type')
        }
        resolve(data)
      })
    })
    req.on('error', reject)
  })
}

async function download(uri, filename) {
  return new Promise((resolve, reject) => {
    const req = https.request(uri, function (res) {
      if (res.statusCode !== 200) {
        console.log('statusCode:', res.statusCode)
        console.log('headers:', res.headers)
        reject('Unexpected response code')
      }
      const file = fs.createWriteStream(filename)
      res.pipe(file)
    })
    req.on('error', reject)
    req.end()
  })
}

const fiatFullListStr = fs.readFileSync(path.join(__dirname, '../assets/fiat-list-full.json'))
const fiatFullList = JSON.parse(fiatFullListStr)

const data = await get(
  'https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page=1'
)
const markets = JSON.parse(data)
const list = {}
const cryptoIconDir = path.join(__dirname, 'crypto-icons')
if (!fs.existsSync(cryptoIconDir)) {
  fs.mkdirSync(cryptoIconDir)
}
for (const market of markets) {
  if (!market.symbol || !market.id || !market.name || !market.image) {
    console.error('Missing data', market, '\n')
    console.error(data)
  }
  const symbol = market.symbol.toUpperCase()
  if (fiatFullList[symbol]) {
    console.error('Crypto symbol is already in fiat list: ' + symbol)
  }
  list[symbol] = [market.id, market.name]

  const imageExt = path.extname(new URL(market.image).pathname)
  download(market.image, path.join(cryptoIconDir, market.id + imageExt))
}

fs.writeFileSync(path.join(__dirname, 'output-list.json'), JSON.stringify(list))
