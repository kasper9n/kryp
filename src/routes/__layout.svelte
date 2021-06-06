<script lang="ts">
  import * as dialog from '../../node_modules/@tauri-apps/api/dialog'
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
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

<a href="/">Kryp</a>
<a href="/transactions">Transactions</a>

<button on:click={load}>Load</button>
<button on:click={save}>Save</button>
<button on:click={saveAs}>Save As...</button>

<slot />

<style lang="sass">
  :global(body)
    user-select: none
    -webkit-user-select: none
</style>
