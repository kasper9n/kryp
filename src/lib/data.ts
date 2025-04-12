import { get, writable } from 'svelte/store'
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

const recent_files_key = 'kryp_recent_files'
export const recent_files = writable(
	JSON.parse(localStorage.getItem(recent_files_key) || '[]') as string[],
)
export function save_recent_files() {
	localStorage.setItem(recent_files_key, JSON.stringify(get(recent_files)))
}

event.listen('opened', async (e: { payload: { opened?: boolean; file_path?: string | null } }) => {
	console.log('OPENED event', e)
	if (e.payload?.opened === true) {
		opened.set(true)
	} else if (e.payload?.opened === false) {
		opened.set(false)
	}
	if (e.payload?.file_path) {
		const file_path = e.payload.file_path
		recent_files.update((files) => {
			if (!files.includes(file_path)) {
				files.push(file_path)
				files.slice(0, 10)
			}
			return files
		})
		save_recent_files()
	}
	settings.set(await runCmd('get_tax_settings'))
})
