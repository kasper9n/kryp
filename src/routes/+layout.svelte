<script lang="ts">
	import '../app.css'
	import { event } from '@tauri-apps/api'
	import { onDestroy } from 'svelte'
	import { goto } from '$app/navigation'
	import { runCmd } from '$lib/general'
	import { opened, settings } from '$lib/data'
	import NewFileModal from '$lib/modals/NewFile.svelte'
	import SettingsModal from '$lib/modals/Settings.svelte'
	import FileDrop from 'svelte-tauri-filedrop'
	import { fade } from 'svelte/transition'
	import { page } from '$app/stores'

	// prevent history from being written, to hide context menu Back/Forwards buttons
	function go(e: MouseEvent) {
		if (e.target instanceof HTMLElement) {
			const href = e.target.getAttribute('href')
			if (href !== null) {
				e.preventDefault()
				e.stopPropagation()
				e.stopImmediatePropagation()
				goto(href)
			}
		}
	}

	let newFileModalVisible = false
	let settingsModalVisible = false

	async function open(path?: string) {
		await runCmd('open', { path })
	}
	async function save() {
		await runCmd('save', { saveAs: false })
	}
	async function saveAs() {
		await runCmd('save', { saveAs: true })
	}
	async function close() {
		await runCmd('close')
	}
	const unlistenFuture = event.listen('tauri://menu', async ({ payload }) => {
		if (payload === 'Dashboard') {
			goto('/')
		} else if (payload === 'Transactions') {
			goto('/transactions')
		} else if (payload === 'Reports') {
			goto('/reports')
		} else if (payload === 'New' && !$opened) {
			newFileModalVisible = true
		} else if (payload === 'Preferences...' && $opened) {
			settingsModalVisible = true
		} else if (payload === 'Open...') {
			open()
		} else if (payload === 'Save') {
			save()
		} else if (payload === 'Save As...') {
			saveAs()
		} else if (payload === 'Import...' && $opened) {
			goto('/import')
		} else if (payload === 'Export...') {
			await runCmd('export')
		} else if (payload === 'Close') {
			close()
		}
	})
	onDestroy(async () => {
		const unlisten = await unlistenFuture
		unlisten()
	})
</script>

{#if $opened}
	<nav class="z-10 flex h-12 select-none items-center space-x-2 px-4 text-sm">
		<a class="item" on:click={go} class:active={$page.route.id === '/'} href="/"
			><span>Dashboard</span></a
		>
		<a
			class="item"
			on:click={go}
			class:active={$page.route.id === '/transactions'}
			href="/transactions"><span>Transactions</span></a
		>
		<a class="item" on:click={go} class:active={$page.route.id === '/reports'} href="/reports"
			><span>Reports</span></a
		>
		<div class="nav-mid" />
		<span class="rounded border bg-white px-1.5 dark:bg-black">{$settings.base_currency}</span>
		<a class="item" on:click={go} class:active={$page.route.id === '/prices'} href="/prices"
			><span>Prices</span></a
		>
		<a class="item" on:click={go} class:active={$page.route.id === '/help'} href="/help"
			><span>Help</span></a
		>
		<button class="item" on:click={() => (settingsModalVisible = true)}>
			<span class="icon">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="currentColor"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					><path
						d="M24 14.187v-4.374c-2.148-.766-2.726-.802-3.027-1.529-.303-.729.083-1.169 1.059-3.223l-3.093-3.093c-2.026.963-2.488 1.364-3.224 1.059-.727-.302-.768-.889-1.527-3.027h-4.375c-.764 2.144-.8 2.725-1.529 3.027-.752.313-1.203-.1-3.223-1.059l-3.093 3.093c.977 2.055 1.362 2.493 1.059 3.224-.302.727-.881.764-3.027 1.528v4.375c2.139.76 2.725.8 3.027 1.528.304.734-.081 1.167-1.059 3.223l3.093 3.093c1.999-.95 2.47-1.373 3.223-1.059.728.302.764.88 1.529 3.027h4.374c.758-2.131.799-2.723 1.537-3.031.745-.308 1.186.099 3.215 1.062l3.093-3.093c-.975-2.05-1.362-2.492-1.059-3.223.3-.726.88-.763 3.027-1.528zm-4.875.764c-.577 1.394-.068 2.458.488 3.578l-1.084 1.084c-1.093-.543-2.161-1.076-3.573-.49-1.396.581-1.79 1.693-2.188 2.877h-1.534c-.398-1.185-.791-2.297-2.183-2.875-1.419-.588-2.507-.045-3.579.488l-1.083-1.084c.557-1.118 1.066-2.18.487-3.58-.579-1.391-1.691-1.784-2.876-2.182v-1.533c1.185-.398 2.297-.791 2.875-2.184.578-1.394.068-2.459-.488-3.579l1.084-1.084c1.082.538 2.162 1.077 3.58.488 1.392-.577 1.785-1.69 2.183-2.875h1.534c.398 1.185.792 2.297 2.184 2.875 1.419.588 2.506.045 3.579-.488l1.084 1.084c-.556 1.121-1.065 2.187-.488 3.58.577 1.391 1.689 1.784 2.875 2.183v1.534c-1.188.398-2.302.791-2.877 2.183zm-7.125-5.951c1.654 0 3 1.346 3 3s-1.346 3-3 3-3-1.346-3-3 1.346-3 3-3zm0-2c-2.762 0-5 2.238-5 5s2.238 5 5 5 5-2.238 5-5-2.238-5-5-5z"
					/></svg
				>
			</span>
		</button>
	</nav>

	<main class="h-0 flex-grow overflow-y-auto">
		<slot />
	</main>
{:else}
	<div class="start-page">
		<h1>Kryp</h1>
		<div class="buttons">
			<button type="button" class="button button-neutral" on:click={() => open()}>Open</button>
			<button
				type="button"
				class="button button-neutral"
				on:click={() => (newFileModalVisible = true)}>New</button
			>
		</div>
		<FileDrop extensions={['json']} handleOneFile={open} let:files>
			{#if files.length > 0}
				<div class="dropzone-overlay" transition:fade={{ duration: 100 }}>
					<h1>Drop to open</h1>
				</div>
			{/if}
		</FileDrop>
	</div>
{/if}

{#if newFileModalVisible}
	<NewFileModal onClose={() => (newFileModalVisible = false)} />
{/if}
{#if settingsModalVisible}
	<SettingsModal onClose={() => (settingsModalVisible = false)} />
{/if}

<style lang="sass">
	:root
		--accent: #3061F6
		--bg: #f8f9fc
		--bg-max: #ffffff
		--bg-modal: #f8f9fc
		--text: hsl(0, 0%, 27%)
		--text-50: hsla(0, 0%, 27%, 0.5)
		--selected-button-group: #191B20
		--input-border: hsla(222, 25%, 65%, 0.45)
		--input-invalid-bg: #fff0f5
	@media (prefers-color-scheme: dark)
		:root
			--bg: #0F0F0F
			--bg-max: #000000
			--bg-modal: #16181d
			--text: hsl(0, 0%, 90%)
			--text-50: hsla(0, 0%, 90%, 0.5)
			--selected-button-group: #e8e9f2
			--input-border: hsla(222, 25%, 65%, 0.45)
			--input-invalid-bg: hsl(340, 100%, 5%)
			--input-highlight-border: hsl(215, 98%, 49%)
			--input-highlight-shadow: hsla(215, 98%, 49%, 0.4)

	@media (prefers-color-scheme: dark)
		:root
			--date-picker-background: #000000
			--date-picker-foreground: #ffffff
			--date-picker-highlight-border: var(--input-highlight-border)
			--date-picker-highlight-shadow: var(--input-highlight-shadow)

	:global(body)
		background-color: var(--bg)
		color: var(--text)
		margin: 0px
		display: flex
		flex-direction: column
		height: 100vh
		@media (prefers-color-scheme: dark)
			color-scheme: dark
	:global(h1)
		font-size: 2rem
	:global(h2)
		font-size: 1.5rem
	:global(h3)
		font-size: 1.25rem
	:global(h4)
		font-size: 16px
	:global(h1), :global(h2), :global(h3)
		margin-top: 0px
		margin-bottom: 0.5em
		font-weight: 600
	:global(h4), :global(h5), :global(h6)
		margin-top: 0px
		margin-bottom: 0.5em
		font-weight: 600
	:global(body), :global(input)
		font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
		--ease: cubic-bezier(0.4, 0.0, 0.2, 1)
	:global([type='text'], [type='email'], [type='url'], [type='password'], [type='number'], [type='date'], [type='datetime-local'], [type='month'], [type='search'], [type='tel'], [type='time'], [type='week'], select)
		background-color: var(--bg-max)
		border-color: var(--input-border)
		color: var(--text)
	:global([type='checkbox'], [type='radio'])
		background-color: var(--bg-max)
		border-color: var(--input-border)
	:global([type='checkbox']:checked:hover, [type='checkbox']:checked:focus, [type='radio']:checked:hover, [type='radio']:checked:focus)
		background-color: var(--accent)
	nav .item
		--shadow-size: 5px
		color: var(--text-50)
		padding: var(--shadow-size)
		cursor: default
		span
			transition: all 100ms ease-out
			font-weight: 500
			padding: 1px 6px
			border-radius: 1px
			cursor: default
			display: block
		span.icon
			padding: 2px
		&:hover span
			background-color: hsla(215, 20%, 50%, 0.2)
			box-shadow: 0px 0px 0px var(--shadow-size) hsla(215, 20%, 50%, 0.2)
	// global to prevent treeshaking
	nav .item.active
		color: var(--text)
	.nav-mid
		width: 50px
		flex-grow: 1
	.start-page
		width: 100vw
		height: 100vh
		display: flex
		flex-direction: column
		align-items: center
		justify-content: center
	.dropzone-overlay
		position: absolute
		width: 100%
		height: 100%
		top: 0px
		left: 0px
		display: flex
		align-items: center
		justify-content: center
		h1
			margin: 0px
			background-color: var(--bg-max)
			border: 1px solid var(--input-border)
			padding: 35px 60px
			border-radius: 10px
</style>
