<script lang="ts">
	import TxTable from '$lib/TxTable.svelte'
	import TxList from '$lib/TxList.svelte'
	import TransactionModal from '$lib/modals/Transaction.svelte'
	import { tags, type Transaction } from '$lib/transactions'
	import { runCmd } from '$lib/general'
	import ButtonGroup from '$lib/ButtonGroup.svelte'
	import InlinePopup from '$lib/InlinePopup.svelte'
	import { goto } from '$app/navigation'

	let view = 0

	let showAdd = false
	function closeModal() {
		showAdd = false
		reload()
	}

	let search = {
		tags: [] as string[],
		asset: '',
	}

	let transactions: Promise<Transaction[]> = runCmd('get_transactions', { search })
	function reload() {
		transactions = runCmd('get_transactions', { search })
	}
	$: transactions = runCmd('get_transactions', { search })
</script>

<div class="flex h-full flex-col">
	<div class="mt-2 flex flex-shrink-0 items-center px-4">
		<ButtonGroup values={['List', 'Table']} bind:selected={view} />
		<div class="p-1.5" />
		<button type="button" class="button" on:click={() => (showAdd = true)}>Add</button>
		<div class="p-1.5" />
		<button type="button" class="button button-secondary" on:click={() => goto('/import')}
			>Import</button
		>
	</div>
	<div class="my-2 flex flex-shrink-0 items-center px-4">
		<InlinePopup let:toggle>
			<button type="button" class="button button-secondary button-slim" on:click={toggle}
				>Type</button
			>
			<div slot="popup" class="popup-box rounded bg-white px-4 py-2 dark:bg-black">
				{#each tags as tag}
					<label class="flex select-none items-center">
						<input type="checkbox" bind:group={search.tags} value={tag.name} />
						<span class="ml-1">{tag.name}</span>
					</label>
				{/each}
			</div>
		</InlinePopup>
		<div class="p-1.5" />
		<div class="relative">
			<input
				class="w-28 rounded px-3 py-1"
				type="text"
				placeholder="Asset"
				bind:value={search.asset}
			/>
		</div>
	</div>
	<div class="h-0 flex-grow">
		{#await transactions then transactions}
			{#if view === 0}
				<TxList {transactions} />
			{:else if view === 1}
				<TxTable {transactions} />
			{/if}
		{/await}
	</div>
</div>

{#if showAdd}
	<TransactionModal onClose={closeModal} />
{/if}

<style lang="sass">
	input[type='text']
		background-color: var(--bg-max)
		color: var(--text)
		border: 1px solid var(--input-border)
		border-radius: 7px
	.popup-box
		border: 1px solid var(--input-border)
</style>
