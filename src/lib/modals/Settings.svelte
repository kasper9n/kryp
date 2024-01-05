<script lang="ts">
	import Button from '$lib/Button.svelte'
	import { settings } from '$lib/data'
	import Modal from '$lib/Modal.svelte'
	import ReorderableList from '$lib/ReorderableList.svelte'
	import { createEventDispatcher } from 'svelte'

	const dispatch = createEventDispatcher()

	function keydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault()
			dispatch('close')
		}
	}
	function save() {
		console.log('x save')
	}
</script>

<Modal width="520px" title="Settings" on:keydown={keydown} on:close>
	<form on:submit|preventDefault={save} class="container">
		<h4>Base currency</h4>
		<input type="text" value={$settings.base_currency} disabled />
		<h4>Price sources</h4>
		<p class="sub">Where prices are fetched from, sorted by priority</p>
		<ReorderableList items={$settings.apis} let:item let:index>
			<div class="list-item">
				<p class="title">
					{#if item.name === 'ExchangeRateHost'}
						Exchangerate.host
					{:else}
						{item.name}
					{/if}
				</p>
				{#if item.key !== undefined}
					<div class="api-row">
						<span class="label">API key</span>
						<input
							type="text"
							bind:value={$settings.apis[index].key}
							placeholder="f17fe84fca08..."
						/>
					</div>
				{:else}
					<span class="mini">No API key required</span>
				{/if}
			</div>
		</ReorderableList>
		<div class="mt-4 grid grid-flow-col justify-end gap-2">
			<Button secondary on:click={() => dispatch('close')}>Cancel</Button>
			<Button type="submit">Save</Button>
		</div>
	</form>
</Modal>

<style lang="sass">
	.container
		font-size: 14px
	h4
		font-weight: 500
		font-size: 14px
		margin-top: 5px
		margin-bottom: 7px
	.sub
		font-weight: 400
		font-size: 12px
		color: hsla(220, 30%, 20%, 0.65)
		margin-top: 5px
		margin-bottom: 7px
	.title
		margin: 0px
		font-weight: 500
	.label
		display: inline-block
		font-size: 13px
		width: 120px

	.list-item
		padding: 8px 12px
	.mini
		font-size: 12px
		color: hsla(220, 30%, 20%, 0.65)
	input
		width: 100%
		min-width: 0px
		font-size: 12px
		height: 27px
		padding: 0px 9px
		box-sizing: border-box
		margin: 0px
		border: 1px solid hsla(222, 25%, 50%, 0.25)
		border-radius: 3px
		transition: 80ms var(--ease)
		transition-property: border-color, box-shadow
		&:focus
			outline: none
			border-color: var(--input-highlight-border)
			box-shadow: 0px 0px 0px 2px var(--input-highlight-shadow)
		&:disabled
			background-color: hsla(220, 20%, 50%, 0.05)
			border: 1px solid hsla(222, 25%, 50%, 0.12)
	.api-row
		margin-top: 4px
		display: flex
		align-items: center
</style>
