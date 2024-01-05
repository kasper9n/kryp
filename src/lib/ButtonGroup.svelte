<script lang="ts">
	// https://github.com/sveltejs/eslint-plugin-svelte/issues/348
	// eslint-disable-next-line no-undef
	type T = $$Generic
	export let values: T[]
	export let selected = 0
</script>

<div class="view">
	{#each values as value, i}
		<button class:selected={selected === i} on:click={() => (selected = i)}>{value}</button>
	{/each}
</div>

<style lang="sass">
	.view
		display: flex
		background-color: var(--bg-max)
		border-radius: 7px
		box-shadow: 0px 0px 2px 0px var(--input-border)
		height: 34px
		border: 1px solid var(--input-border)
	button
		display: block
		margin: 2px
		cursor: pointer
		user-select: none
		font-size: 13px
		font-weight: 500
		background-color: transparent
		border: none
		color: var(--text)
		padding: 0px 16px
		min-width: 60px
		transition: 100ms ease-out
		transition-property: transform, opacity, box-shadow
		border-radius: 4px
		&:active
			opacity: 0.95
			transform: scale(0.95)
	.selected
		color: var(--bg)
		background-color: var(--selected-button-group)
		box-shadow: 0px 0px 1px 0px #000000
</style>
