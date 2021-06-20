<script lang="ts">
  import * as dialog from '../../node_modules/@tauri-apps/api/dialog'
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
  import { page } from '$app/stores'
  import Button from '../lib/Button.svelte'
  import { transactions } from '../lib/transactions'

  function popup(msg: string) {
    invoke('error_popup', { msg })
  }

  invoke('load_file', {
    path: '/Users/kasper/Downloads/test.kryp',
  })
    .then(() => {
      transactions.refresh()
    })
    .catch(popup)

  async function load() {
    await invoke('open').catch(popup)
  }
  async function save() {
    await invoke('save', { saveAs: false }).catch(popup)
  }
  async function saveAs() {
    await invoke('save', { saveAs: true }).catch(popup)
  }
</script>

<div class="nav">
  <a href="/" class:current={$page.path === '/'}>Kryp</a>
  <a href="/transactions" class:current={$page.path === '/transactions'}>Transactions</a>
  <button on:click={load}>Load</button>
  <button on:click={save}>Save</button>
  <button on:click={saveAs}>Save As...</button>
</div>

<slot />

<style lang="sass">
  :global(body)
    background-color: #F5F7FB
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
