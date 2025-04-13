<script lang="ts">
	import { run_unwrap } from '$lib/data'
	import Modal from 'modal-svelte'

	export let onClose: () => void
	let baseCurrency = 'USD'

	async function create() {
		await run_unwrap.newFile(baseCurrency)
		onClose()
	}
</script>

<Modal title="New File" class="w-[340px]" form={create} onCancel={onClose}>
	<p>Base currency</p>
	<!-- svelte-ignore a11y-autofocus -->
	<input type="text" class="text-input" bind:value={baseCurrency} autofocus />
	<div class="mt-4 grid grid-flow-col justify-end gap-2">
		<button type="button" class="button button-secondary" on:click={() => onClose()}>Cancel</button>
		<button type="submit" class="button">Create</button>
	</div>
</Modal>

<style lang="sass">
	p
		font-size: 13px
		margin-bottom: 5px
</style>
