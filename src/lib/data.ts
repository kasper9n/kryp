import { writable } from 'svelte/store'
import { runCmd } from '$lib/general'
import { event } from '@tauri-apps/api'

export type PriceData = {
	assets: { [key: string]: PriceDataAsset }
}
export type PriceDataAsset = {
	symbol: string
	kind: string
	interval: string
	prices: Map<number, number>
}

export type Balance = {
	acquire_date: number
	amount: string
	currency: string
	wallet: string
	cost: string
}

export type Realized = {
	date: number
	input: string
	output: string
	wallet: string
}

export type Deposit = {
	date: number
	amount: string
	currency: string
	value: string
	wallet: string
}

export type TaxSettings = {
	base_currency: string
	apis: { name: string; key?: string; disabled: boolean }[]
}

export const opened = writable(false)

export const settings = writable({
	base_currency: 'USD',
	apis: [],
} as TaxSettings)

runCmd('is_open').then((isOpen) => {
	opened.set(isOpen)
})
runCmd('get_tax_settings').then((taxSettings) => {
	settings.set(taxSettings)
})

event.listen('opened', async (e) => {
	console.log('OPENED event')
	if (e.payload === true) {
		opened.set(true)
	} else if (e.payload === false) {
		opened.set(false)
	}
	settings.set(await runCmd('get_tax_settings'))
})
