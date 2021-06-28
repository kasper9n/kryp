<script lang="ts">
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
  import { page } from '$app/stores'
  import { refresh, popup } from '../lib/general'

  let fileOpened = false

  // open('/Users/kasper/Downloads/test.kryp')

  async function open(path?: string) {
    await invoke('open', { path })
      .then(() => {
        refresh()
        fileOpened = true
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

{#if fileOpened}
  <div class="nav">
    <a href="/" class:current={$page.path === '/'}>Dashboard</a>
    <a href="/transactions" class:current={$page.path === '/transactions'}>Transactions</a>
    <button on:click={onOpenClick}>Load</button>
    <button on:click={save}>Save</button>
    <button on:click={saveAs}>Save As...</button>
  </div>

  <slot />
{:else}
  This is where we create or open a file
  <button on:click={onOpenClick}>Open</button>
{/if}

<style lang="sass">
  :global(body)
    background-color: #f8f9fc
  :global(h1, h2, h3)
    margin-top: 0px
    margin-bottom: 1em
    font-weight: 600
  :global(h4, h5, h6)
    margin-top: 0px
    margin-bottom: 1em
    font-weight: 600
  .nav
    display: flex
    align-items: center
    user-select: none
    -webkit-user-select: none
    background-color: #ffffff
    border-bottom: 1px solid #e7e8e8
    padding: 0px 20px
  a
    color: #676d7e
    font-weight: 600
    text-decoration: none
    padding: 15px
    &.current
      color: #000000
</style>
