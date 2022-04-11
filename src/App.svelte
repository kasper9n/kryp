<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
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
  import './app.css'

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
</script>

{#if $opened}
  <div class="nav">
    <a on:click={go} use:active data-exact href="/"><button>Dashboard</button></a>
    <a on:click={go} use:active href="/transactions"><button>Transactions</button></a>
    <a on:click={go} use:active href="/reports"><button>Reports</button></a>
    <div class="nav-mid" />
    <span class="base">{$settings.base_currency}</span>
    <a on:click={go} use:active href="/prices"><button>Prices</button></a>
    <a on:click={go} use:active href="/help"><button>Help</button></a>
  </div>

  <Route path="/"><DashboardPage /></Route>
  <Route path="/transactions"><TransactionsPage on:import={() => router.goto('/import')} /></Route>
  <Route path="/prices"><PricesPage /></Route>
  <Route path="/help"><HelpPage /></Route>
  <Route path="/import"><ImportPage /></Route>
  <Route path="/import/confirm"><ImportConfirmPage /></Route>
  <Route path="/reports"><ReportsPage /></Route>
  <Route fallback>404</Route>
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
  :global(body)
    background-color: #f8f9fc
    color: hsl(0, 0%, 27%)
    margin: 0px
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
  .nav
    display: flex
    align-items: center
    user-select: none
    padding: 0px 20px
    a, .base
      font-size: 15px
      padding: 10px 5px
      margin: 5px
      cursor: default
    a
      border: none
      font: inherit
      font-weight: 500
      text-decoration: none
      transition: all 120ms var(--ease)
      color: hsl(226, 5%, 20%)
      &:hover
        color: hsl(226, 5%, 28%)
        opacity: 0.8
      &:global(.active)
        color: #15b28d
        opacity: 1
      button
        border: none
        font: inherit
        background-color: transparent
        color: inherit
        padding: 0px
        margin: 0px
    .base
      background-color: #ffffff
      padding: 2px 5px
      border-radius: 4px
      border: 1px solid #e7e8e8
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
      background-color: #ffffff
      border: 1px solid #e7e8e8
      padding: 35px 60px
      border-radius: 10px
</style>
