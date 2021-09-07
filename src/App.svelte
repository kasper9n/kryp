<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import DashboardPage from './routes/index.svelte'
  import TransactionsPage from './routes/transactions.svelte'
  import PricesPage from './routes/prices.svelte'
  import HelpPage from './routes/help.svelte'
  import { refresh, popup } from './lib/general'
  import { opened } from './lib/data'
  import NewFileModal from './modals/NewFile.svelte'
  import Button from './lib/Button.svelte'
  const pages = [DashboardPage, TransactionsPage, PricesPage, HelpPage]
  let page = 0
  function link(node: HTMLElement, num: number) {
    function handler(e: MouseEvent) {
      e.preventDefault()
      page = num
    }
    node.addEventListener('click', handler)
    return {
      destroy() {
        node.removeEventListener('click', handler)
      },
    }
  }

  let newFileModalVisible = false

  async function open(path?: string) {
    await invoke('open', { path })
      .then(() => {
        refresh()
      })
      .catch(popup)
  }
  async function save() {
    await invoke('save', { saveAs: false }).catch(popup)
  }
  async function saveAs() {
    await invoke('save', { saveAs: true }).catch(popup)
  }
  async function close() {
    await invoke('close').catch(popup)
    refresh()
  }
  const unlistenFuture = event.listen('menu', ({ payload }) => {
    if (payload === 'Dashboard') {
      page = 0
    } else if (payload === 'Transactions') {
      page = 1
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
    <button class="link" use:link={0} class:current={page === 0}>Dashboard</button>
    <button class="link" use:link={1} class:current={page === 1}>Transactions</button>
    <div class="nav-mid" />
    <button class="link" use:link={2} class:current={page === 2}>Prices</button>
    <button class="link" use:link={3} class:current={page === 3}>Help</button>
    <button on:click={() => open()}>Load</button>
    <button on:click={save}>Save</button>
    <button on:click={saveAs}>Save As...</button>
  </div>

  <svelte:component this={pages[page]} />
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
  .link
    background-color: transparent
    border: none
    font: inherit
    margin: 0px
    color: #676d7e
    font-weight: 600
    text-decoration: none
    padding: 15px
    &.current
      color: #000000
  .start-page
    position: absolute
    width: 100%
    height: 100%
    display: flex
    flex-direction: column
    align-items: center
    justify-content: center
</style>
