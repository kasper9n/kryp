<script lang="ts">
	import { runCmd } from '$lib/general'
	import Modal from 'modal-svelte'
	import Button from '$lib/Button.svelte'
	import TextInput from '$lib/TextInput.svelte'

	export let onClose: () => void
	let baseCurrency = 'USD'

	async function create() {
		await runCmd('new_file', {
			baseCurrency: baseCurrency,
		})
		onClose()
	}
</script>

<Modal title="New File" class="w-[340px]" form={create} onCancel={onClose}>
	<p>Base currency</p>
	<TextInput bind:value={baseCurrency} autofocus />
	<div class="mt-4 grid grid-flow-col justify-end gap-2">
		<Button secondary on:click={() => onClose()}>Cancel</Button>
		<Button type="submit">Create</Button>
	</div>
</Modal>

<style lang="sass">
	p
		font-size: 13px
		margin-bottom: 5px
</style>
