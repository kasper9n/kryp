<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import DashboardPage from './routes/index.svelte'
  import TransactionsPage from './routes/transactions.svelte'
  import PricesPage from './routes/prices.svelte'
  import { refresh, popup } from './lib/general'
  import { opened } from './lib/data'
  import NewFileModal from './modals/NewFile.svelte'
  const pages = [DashboardPage, TransactionsPage, PricesPage]
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
    <button class="link" use:link={2} class:current={page === 2}>Prices</button>
    <button on:click={() => open()}>Load</button>
    <button on:click={save}>Save</button>
    <button on:click={saveAs}>Save As...</button>
  </div>

  <svelte:component this={pages[page]} />
{:else}
  This is where we create or open a file
  <button on:click={() => open()}>Open</button>
  <button on:click={() => (newFileModalVisible = true)}>New</button>
{/if}

<NewFileModal bind:visible={newFileModalVisible} />

<style lang="sass">
  :global(body)
    background-color: #f8f9fc
    margin: 0px
  :global(h1), :global(h2), :global(h3)
    margin-top: 0px
    margin-bottom: 1em
    font-weight: 600
  :global(h4), :global(h5), :global(h6)
    margin-top: 0px
    margin-bottom: 1em
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
</style>
