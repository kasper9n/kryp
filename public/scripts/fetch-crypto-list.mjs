import https from 'https'
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

async function get(url) {
	const response = await fetch(url)
	if (!response.ok) {
		console.log('statusCode:', response.statusCode)
		console.log('headers:', response.headers)
		console.log(response.body)
		throw 'Unexpected response code or content type'
	}
	return response
}

async function download(uri, filename) {
	const req = https.request(uri, function (res) {
		if (res.statusCode !== 200) {
			console.log('statusCode:', res.statusCode)
			console.log('headers:', res.headers)
			console.log(uri)
			throw 'Unexpected response code'
		}
		const file = fs.createWriteStream(filename)
		res.pipe(file)
	})
	req.on('error', (e) => {
		throw 'Request error' + e
	})
	req.end()
}

async function getTopCoins(coinCount) {
	let coins = []
	const pageCount = Math.max(coinCount / 250) + 1
	for (let n = 1; n < pageCount; n++) {
		console.log('Fetching page ' + n)
		await new Promise((resolve) => setTimeout(resolve, 1000))
		const response = await get(
			`https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=250&page=${n}`,
		)
		const pageCoins = await response.json()
		coins = coins.concat(pageCoins)
	}
	if (coins.length < coinCount) {
		throw 'Not enough coins'
	}
	return coins.slice(0, coinCount)
}

function imageToSmall(urlstring) {
	// https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579
	const url = new URL(urlstring)
	const pathSegments = url.pathname.split('/')
	if (pathSegments[1] === 'coins' && pathSegments[2] === 'images' && pathSegments[4] === 'large') {
		pathSegments[4] = 'small'
	} else {
		throw 'Unexpected image url ' + urlstring
	}
	return url.origin + pathSegments.join('/') + url.search
}

async function main() {
	const fiatFullListStr = fs.readFileSync(path.join(__dirname, './fiat-list-full.json'))
	const fiatFullList = JSON.parse(fiatFullListStr)

	const fetchedCoinObjects = await getTopCoins(500)
	const list = {}
	const cryptoIconDir = path.join(__dirname, 'crypto-icons')
	if (!fs.existsSync(cryptoIconDir)) {
		fs.mkdirSync(cryptoIconDir)
	}
	let addedCount = 0
	for (const coin of fetchedCoinObjects) {
		if (!coin.symbol || !coin.id || !coin.name || !coin.image) {
			console.error('Missing data', coin, '\n')
		}
		const symbol = coin.symbol.toUpperCase()
		if (fiatFullList[symbol]) {
			console.error(`${symbol} is already in fiat list. Skipping...`)
		} else if (list[symbol]) {
			console.error(`${symbol} already exists as ${list[symbol][0]}. Skipping...`)
		} else {
			const imageExt = path.extname(new URL(coin.image).pathname)
			const smallImage = imageToSmall(coin.image)
			await download(smallImage, path.join(cryptoIconDir, coin.id + imageExt))

			list[symbol] = [coin.id, coin.name]
			addedCount++
		}
	}
	console.log(`Added ${addedCount} coins`)
	if (addedCount !== Object.values(list).length) {
		throw `Error: Only ${Object.values(list).length} in list`
	}

	fs.writeFileSync(path.join(__dirname, 'crypto-list.json'), JSON.stringify(list, null, '\t'))
}

try {
	await main()
} catch (e) {
	console.log('Error', e)
}
