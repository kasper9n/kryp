<script lang="ts">
	import { runCmd } from '$lib/general'
	import { goto } from '$app/navigation'
	import type { ImportData } from '../+page.svelte'
	import TxTableRow from '../TxTableRow.svelte'

	let importData: ImportData | null = null
	runCmd('get_import_data').then((data: ImportData) => {
		importData = data
		console.log(importData)
	})

	async function continueImport() {
		await runCmd('continue_import')
		goto('/transactions')
	}

	async function cancel() {
		await runCmd('cancel_import')
		goto('/import')
	}

	async function updateImportTransactions() {
		if (importData) {
			importData = await runCmd('update_import_transactions', {
				transactions: importData.transactions,
			})
			console.log(importData)
		}
	}
</script>

<h1 class="text-center">Import</h1>
{#if importData === null || importData.transactions.length === 0}
	<!-- loading -->
{:else if !importData.has_errors}
	<p class="text-center">Successful scan. Continue to import?</p>
	<div class="button-row flex justify-center">
		<button type="button" class="button button-secondary" on:click={cancel}>Cancel</button>
		<div class="w-2"></div>
		<button type="button" class="button" disabled={importData.has_errors} on:click={continueImport}
			>Continue</button
		>
	</div>
{:else}
	<p class="red text-center">
		Could not calculate the cost of the following transactions. Please enter them manually
	</p>
	<div class="button-row">
		<button type="button" class="button" on:click={updateImportTransactions}>Save</button>
	</div>
	<table>
		<thead>
			<tr>
				<th>Type</th>
				<th>Sent</th>
				<th>Cur.</th>
				<th>Wallet</th>
				<th>Received</th>
				<th>Cur.</th>
				<th>Wallet</th>
				<th>Fee</th>
				<th>Cur.</th>
				<th class="note">Note</th>
				<th>Hash</th>
				<th>Date</th>
				<th>Net worth</th>
				{#if importData.has_errors}
					<th>Error</th>
				{/if}
			</tr>
		</thead>
		<tbody>
			{#each importData.transactions as tx, i}
				<TxTableRow transaction={tx.transaction} cost={tx.cost} error={tx.error} {i} {importData} />
			{/each}
		</tbody>
	</table>

	<style lang="sass">
	.center
		display: flex
		align-items: center
		justify-content: center
	table
		margin: 36px auto
		border-spacing: 0px
		border-collapse: collapse
		cursor: default
		font-size: 13px
	td
		padding: 5px
		border: 1px solid var(--input-border)
		overflow: hidden
		text-overflow: ellipsis
		&.amount
			text-align: right
	thead
		th
			text-align: center
			padding-bottom: 4px
	tbody
		background-color: #f6f6f9
	.odd
		background-color: #ffffff

	.note
		width: 180px
		max-width: 180px
	.hash
		max-width: 100px
	.date
		white-space: nowrap

	.red
		color: #f92f72
	.green
		color: #25b670
	.blue
		color: #2ea8fa
	.purple
		color: #b853ee
	.button-row
		display: flex
		margin: 18px auto
		justify-content: center
	.space
		padding: 6px
	</style>
{/if}
