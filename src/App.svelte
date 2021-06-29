<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import DashboardPage from './routes/index.svelte'
  import TransactionsPage from './routes/transactions.svelte'
  import { refresh, popup } from './lib/general'
  import { opened } from './lib/data'
  const pages = [DashboardPage, TransactionsPage]
  let page = 0
  function link(node: HTMLElement, num: number) {
    function handler() {
      page = num
    }
    node.addEventListener('click', handler)
    return {
      destroy() {
        node.removeEventListener('click', handler)
      },
    }
  }

  async function open(path?: string) {
    await invoke('open', { path })
      .then(() => {
        refresh()
      })
      .catch(popup)
  }
  function onOpenClick() {
    open()
  }
  async function save() {
    await invoke('save', { saveAs: false }).catch(popup)
  }
  async function saveAs() {
    await invoke('save', { saveAs: true }).catch(popup)
  }
</script>

{#if $opened}
  <div class="nav">
    <div class="link" use:link={0} class:current={page === 0}>Dashboard</div>
    <div class="link" use:link={1} class:current={page === 1}>Transactions</div>
    <button on:click={onOpenClick}>Load</button>
    <button on:click={save}>Save</button>
    <button on:click={saveAs}>Save As...</button>
  </div>

  <svelte:component this={pages[page]} />
{:else}
  This is where we create or open a file
  <button on:click={onOpenClick}>Open</button>
{/if}

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
    cursor: pointer
    color: #676d7e
    font-weight: 600
    text-decoration: none
    padding: 15px
    &.current
      color: #000000
</style>
