<script lang="ts">
	import { run_unwrap, type DWTags, type Report } from '$lib/data'
	import Dropdown from '$lib/Dropdown.svelte'

	let year = new Date().getFullYear() - 1

	let tags: DWTags | null = null
	getDepositWithdrawalTags()
	async function getDepositWithdrawalTags() {
		tags = await run_unwrap.getDepositWithdrawalTags()
		console.log('tags', tags)
	}

	let deductibleTags = ['Lost']
	let incomeTags = ['Income', 'Interest']
	let hideValuesLessThan: number | null = 0
	const cost_basis_methods = [
		{ name: 'FIFO', value: 'fifo' },
		{ name: 'HIFO', value: 'hifo' },
	] as const
	let cost_basis_method = cost_basis_methods[0]
	let report: Report | null = null
	$: if (tags) {
		getReport(year, deductibleTags, incomeTags, hideValuesLessThan, cost_basis_method.value)
	}
	async function getReport(
		year: number,
		deductibleTags: string[],
		incomeTags: string[],
		hideValuesLessThan: number | null,
		cost_basis_method: string,
	) {
		report = await run_unwrap.getReport(
			Number(year),
			deductibleTags,
			incomeTags,
			String(hideValuesLessThan || 0),
			cost_basis_method,
		)
		console.log('report', report)
	}

	async function download() {
		report = await run_unwrap.downloadReport(
			Number(year),
			deductibleTags,
			incomeTags,
			String(hideValuesLessThan || 0),
		)
	}
</script>

<div class="mx-auto max-w-5xl overflow-hidden">
	<div class="mx-4 my-4 flex items-center">
		<p class="mr-4">Year</p>
		<input type="number" bind:value={year} />
		{#if !report}
			<span class="ml-4">Loading...</span>
		{/if}
	</div>
	<div class="mx-4 my-4 flex items-center">
		<p class="mr-4">Cost basis method</p>
		<div class="w-28">
			<Dropdown options={cost_basis_methods} bind:value={cost_basis_method} />
		</div>
	</div>

	<div class="flex">
		{#if tags}
			<div class="mx-4 my-4 w-1/2">
				<p class="mr-4">Income tags</p>
				<span class="text-sm opacity-75">In some cases, forks and airdrops may be income</span>
				{#each tags.deposit_tags.sort() as tag}
					<label class="flex items-center px-1">
						<input type="checkbox" class="mr-1" bind:group={incomeTags} value={tag} />
						<div>{tag}</div>
					</label>
				{/each}
			</div>

			<div class="mx-4 my-4 w-1/2">
				<p class="mr-4">Deductible tags</p>
				<span class="text-sm opacity-75"
					>Tags such as Lost that you may be able to deduct in your tax return. Gains are not
					realized for these</span
				>
				{#each tags.withdrawal_tags.sort() as tag}
					<label class="flex items-center px-1">
						<input type="checkbox" class="mr-1" bind:group={deductibleTags} value={tag} />
						<div>{tag}</div>
					</label>
				{/each}
			</div>
		{/if}
	</div>

	<div class="m-4 max-w-xl rounded border border-slate-200 px-4 py-2">
		<div class="my-1.5 flex">
			<div class="w-18 mr-auto">Realized gain</div>
			<div class="font-medium text-emerald-500">
				{#if report}+ {report.total_realized_gain}{/if}
			</div>
		</div>
		<div class="my-1.5 flex">
			<div class="w-18 mr-auto">Realized loss</div>
			<div class="font-medium text-red-500">
				{#if report}- {report.total_realized_loss}{/if}
			</div>
		</div>
		<div class="my-1.5 flex">
			<div class="w-18 mr-auto">Realized</div>
			<div class="font-medium text-blue-500">
				{#if report}{report.total_realized}{/if}
			</div>
		</div>
		<div class="my-2 border-t"></div>
		<div class="my-1.5 flex">
			<div class="w-18 mr-auto">Income</div>
			<div class="font-medium text-emerald-500">
				{#if report}+ {report.total_income}{/if}
			</div>
		</div>
		<div class="my-1.5 flex">
			<div class="w-18 mr-auto">Deductible amount</div>
			<div class="font-medium text-red-500">
				{#if report}- {report.total_deductible}{/if}
			</div>
		</div>
	</div>
	<div class="m-4 rounded border border-slate-200 py-2">
		<div class="mx-4 my-4 flex items-center">
			<p class="mr-4">Hide values less than</p>
			<input type="number" bind:value={hideValuesLessThan} />
		</div>
		<table class="w-full border-separate px-4 text-sm">
			<thead>
				<tr class="my-1 flex text-left font-bold">
					<td class="w-0 flex-grow">Name</td>
					<td class="w-0 flex-grow">Income</td>
					<td class="w-0 flex-grow">Deductible</td>
					<td class="w-0 flex-grow">Realized</td>
					<td class="w-0 flex-grow">Realized Gain</td>
					<td class="w-0 flex-grow">Realized Loss</td>
				</tr>
			</thead>
			<tbody>
				{#if report}
					{#each report.records as row}
						<tr class="flex border-t border-slate-200 py-1">
							<td class="w-0 flex-grow">{row.name}</td>
							<td class="w-0 flex-grow">{row.income}</td>
							<td class="w-0 flex-grow">{row.deductible}</td>
							<td class="w-0 flex-grow">{row.realized}</td>
							<td class="w-0 flex-grow">{row.realized_gain}</td>
							<td class="w-0 flex-grow">{row.realized_loss}</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
		<button type="button" class="mx-2 my-4 px-2 text-blue-500" on:click={download}>Download</button>
	</div>
</div>
