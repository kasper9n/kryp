<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import { refresh, runCmd } from './lib/general'
  import { opened } from './lib/data'
  import NewFileModal from './modals/NewFile.svelte'
  import Button from './lib/Button.svelte'
  import { Route, active, router } from 'tinro'

  import DashboardPage from './routes/index.svelte'
  import TransactionsPage from './routes/transactions.svelte'
  import PricesPage from './routes/prices.svelte'
  import HelpPage from './routes/help.svelte'
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

  async function open(path?: string) {
    await runCmd('open', { path })
    refresh()
  }
  async function save() {
    await runCmd('save', { saveAs: false })
  }
  async function saveAs() {
    await runCmd('save', { saveAs: true })
  }
  async function close() {
    await runCmd('close')
    refresh()
  }
  const unlistenFuture = event.listen('menu', ({ payload }) => {
    if (payload === 'Dashboard') {
      router.goto('/', true)
    } else if (payload === 'Transactions') {
      router.goto('/transactions', true)
    } else if (payload === 'New') {
      newFileModalVisible = true
    } else if (payload === 'Open...') {
      open()
    } else if (payload === 'Save') {
      save()
    } else if (payload === 'Save As...') {
      saveAs()
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
    <a on:click={go} use:active data-exact href="/">Dashboard</a>
    <a on:click={go} use:active href="/transactions">Transactions</a>
    <div class="nav-mid" />
    <a on:click={go} use:active href="/prices">Prices</a>
    <a on:click={go} use:active href="/help">Help</a>
  </div>

  <Route path="/"><DashboardPage /></Route>
  <Route path="/transactions"><TransactionsPage /></Route>
  <Route path="/prices"><PricesPage /></Route>
  <Route path="/help"><HelpPage /></Route>
{:else}
  <div class="start-page">
    <h1>Kryp</h1>
    <div class="buttons">
      <Button neutral on:click={() => open()}>Open</Button>
      <Button neutral on:click={() => (newFileModalVisible = true)}>New</Button>
    </div>
  </div>
{/if}

<NewFileModal bind:visible={newFileModalVisible} />

<style lang="sass">
  :global(body)
    background-color: #f8f9fc
    color: #191c1f
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
    font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
    --ease: cubic-bezier(0.4, 0.0, 0.2, 1)
  .nav
    display: flex
    align-items: center
    user-select: none
    -webkit-user-select: none
    background-color: #ffffff
    border-bottom: 1px solid #e7e8e8
    padding: 0px 20px
  .nav-mid
    width: 50px
    flex-grow: 1
  .nav a
    background-color: transparent
    border: none
    font: inherit
    margin: 0px
    color: #676d7e
    font-weight: 600
    text-decoration: none
    padding: 15px
    &:global(.active)
      color: #000000
  .start-page
    width: 100vw
    height: 100vh
    display: flex
    flex-direction: column
    align-items: center
    justify-content: center
</style>
