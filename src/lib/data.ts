import { commands as c, events, type Result, type TaxSettings } from '../../bindings'
export * from '../../bindings'
import { get, writable } from 'svelte/store'

// Utility type to unwrap Result<T, E> in a Promise
type UnwrapResult<T> = T extends Promise<Result<infer U, unknown>> ? Promise<U> : T

// Transform each method to unwrap its return type
type UnwrapResultMethods<T> = {
	[K in keyof T]: T[K] extends (...args: infer A) => infer R
		? (...args: A) => UnwrapResult<R>
		: T[K]
}

export const run_unwrap = new Proxy({} as UnwrapResultMethods<typeof c>, {
	get:
		(_, property: string) =>
		async (...args: unknown[]) => {
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const result: Result<any, any> = await (c as any)[property](...args)
			if (result.status === 'error') {
				c.errorPopup(String(result.error))
				throw new Error(result.error)
			}
			return result.data
		},
})

export const opened = writable(false)

export const settings = writable({
	base_currency: 'USD',
	apis: [],
} as TaxSettings)

run_unwrap.isOpen().then((is_open) => {
	opened.set(is_open)
})
run_unwrap.getTaxSettings().then((tax_settings) => {
	settings.set(tax_settings)
})

const recent_files_key = 'kryp_recent_files'
export const recent_files = writable(
	JSON.parse(localStorage.getItem(recent_files_key) || '[]') as string[],
)
export function save_recent_files() {
	localStorage.setItem(recent_files_key, JSON.stringify(get(recent_files)))
}

events.openedEvent.listen(async (e) => {
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
	settings.set(await run_unwrap.getTaxSettings())
})
