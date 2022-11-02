<script lang="ts">
  import './app.css'
  import { event } from '@tauri-apps/api'
  import { onDestroy, onMount } from 'svelte'
  import { Route, active, router } from 'tinro'
  import { runCmd } from '$lib/general'
  import { opened, settings } from '$lib/data'
  import NewFileModal from '$lib/modals/NewFile.svelte'
  import SettingsModal from '$lib/modals/Settings.svelte'
  import Button from '$lib/Button.svelte'
  import DashboardPage from '$routes/index.svelte'
  import TransactionsPage from '$routes/transactions.svelte'
  import ReportsPage from './routes/reports.svelte'
  import PricesPage from '$routes/prices.svelte'
  import HelpPage from '$routes/help.svelte'
  import ImportPage from '$routes/import/index.svelte'
  import ImportConfirmPage from '$routes/import/confirm.svelte'
  import FileDrop from 'svelte-tauri-filedrop'
  import { fade } from 'svelte/transition'

  // prevent history from being written, to hide context menu Back/Forwards buttons
  function go(e: MouseEvent) {
    if (e.target instanceof HTMLElement) {
      const href = e.target.getAttribute('href')
      if (href !== null) {
        e.preventDefault()
        e.stopPropagation()
        e.stopImmediatePropagation()
        router.goto(href, true)
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
      router.goto('/', true)
    } else if (payload === 'Transactions') {
      router.goto('/transactions', true)
    } else if (payload === 'Reports') {
      router.goto('/reports', true)
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
      router.goto('/import', true)
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

  const prefersDarkMQ = matchMedia('(prefers-color-scheme: dark)')
  let darkMode = prefersDarkMQ.matches
  function handler(e: { matches: boolean }) {
    darkMode = e.matches
  }
  // new onchange/addEventListenr api not supported in macOS Catalina
  prefersDarkMQ.addListener(handler)
  onDestroy(() => {
    prefersDarkMQ.removeListener(handler)
  })
</script>

{#if $opened}
  <nav class="h-12">
    <div class="z-10 flex h-12 select-none items-center space-x-2 px-4 text-sm">
      <a on:click={go} use:active data-exact href="/"><button>Dashboard</button></a>
      <a on:click={go} use:active href="/transactions"><button>Transactions</button></a>
      <a on:click={go} use:active href="/reports"><button>Reports</button></a>
      <div class="nav-mid" />
      <span class="rounded border bg-white px-1.5 dark:bg-black">{$settings.base_currency}</span>
      <a on:click={go} use:active href="/prices"><button>Prices</button></a>
      <a on:click={go} use:active href="/help"><button>Help</button></a>
    </div>
  </nav>

  <main class="h-0 flex-grow overflow-y-auto">
    <Route path="/"><DashboardPage {darkMode} /></Route>
    <Route path="/transactions"><TransactionsPage on:import={() => router.goto('/import')} /></Route
    >
    <Route path="/prices"><PricesPage /></Route>
    <Route path="/help"><HelpPage /></Route>
    <Route path="/import"><ImportPage /></Route>
    <Route path="/import/confirm"><ImportConfirmPage /></Route>
    <Route path="/reports"><ReportsPage /></Route>
    <Route fallback>404</Route>
  </main>
{:else}
  <div class="start-page">
    <h1>Kryp</h1>
    <div class="buttons">
      <Button neutral on:click={() => open()}>Open</Button>
      <Button neutral on:click={() => (newFileModalVisible = true)}>New</Button>
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
  <NewFileModal on:close={() => (newFileModalVisible = false)} />
{/if}
{#if settingsModalVisible}
  <SettingsModal on:close={() => (settingsModalVisible = false)} />
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
  :global(h2)
    font-size: 24px
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
  nav a
    --shadow-size: 5px
    color: var(--text-50)
    padding: var(--shadow-size)
    cursor: default
    button
      transition: all 100ms ease-out
      font-weight: 500
      padding: 1px 6px
      border-radius: 1px
      cursor: default
    &:hover
      button
        background-color: hsla(215, 20%, 50%, 0.2)
        box-shadow: 0px 0px 0px var(--shadow-size) hsla(215, 20%, 50%, 0.2)
  // global to prevent treeshaking
  nav :global(a.active)
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
