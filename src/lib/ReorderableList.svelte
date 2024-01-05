<script lang="ts">
	// https://github.com/sveltejs/eslint-plugin-svelte/issues/348
	// eslint-disable-next-line no-undef
	type T = $$Generic
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	interface $$Slots {
		default: {
			item: T
			index: number
		}
	}
	export let items: T[]
</script>

<ul>
	{#each items as item, i}
		<li>
			<slot {item} index={i} />
		</li>
	{/each}
</ul>

<style lang="sass">
	ul
		list-style: none
		margin: 0px
		padding: 0px
		border: 1px solid hsl(220, 20%, 88%)
		border-radius: 3px
</style>
