<script lang="ts">
	import { event } from '@tauri-apps/api'
	import { onDestroy } from 'svelte'
	import { goto } from '$app/navigation'
	import { run_unwrap } from '$lib/data'

	let source = 'Kryp'

	let timezone = ''
	run_unwrap.getSystemTimezone().then((tz) => {
		if (tz) {
			timezone = tz
		}
	})

	async function importFile() {
		const cancelled = await run_unwrap.scanImportFile(source, timezone)
		if (!cancelled) {
			goto('/import/confirm')
		}
	}

	type ImportStatus = {
		index: number
		count: number
	}
	let status: ImportStatus | null = null
	const statusUnlisten = event.listen('importStatus', (e) => {
		if (e.payload) {
			status = e.payload as ImportStatus
		}
	})
	onDestroy(async () => (await statusUnlisten)())
</script>

<h1 class="center">Import</h1>
<div class="page">
	{#if status}
		<p class="center">Scanned {status.index}/{status.count} transactions</p>
	{:else}
		<div class="my-4">
			<div class="mx-auto text-center">Type</div>
			<select class="mx-auto block text-center text-sm" bind:value={source}>
				<option value="Kryp">Kryp</option>
				<option value="Binance">Binance</option>
			</select>
		</div>
		<div class="my-4">
			<div class="mx-auto text-center">Timezone</div>
			{#if source === 'Kryp'}
				<input class="mx-auto block text-sm" type="text" bind:value={timezone} />
			{:else}
				<input class="mx-auto block text-sm" type="text" value="Auto" disabled />
			{/if}
		</div>
		<p class="center">Import a custom CSV or TSV file</p>
		<div class="center my-4">
			<button type="button" class="button" on:click={importFile}>Import</button>
		</div>
	{/if}
</div>

<style lang="sass">
	.center
		display: flex
		align-items: center
		justify-content: center
	.page
		margin: 10px
</style>
