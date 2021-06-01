<script lang="ts">
  // import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
  import * as dialog from '../../node_modules/@tauri-apps/api/dialog'
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'

  function popup(msg: string) {
    invoke('error_popup', { msg })
  }

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

<svelte:head>
  <title>Kryp</title>
</svelte:head>

<p>Dashboard</p>
<button on:click={load}>Load</button>
<button on:click={save}>Save</button>
<button on:click={saveAs}>Save As...</button>
