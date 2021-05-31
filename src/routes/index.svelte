<script lang="ts">
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
  import * as dialog from '../../node_modules/@tauri-apps/api/dialog'

  function popup(msg: string) {
    invoke('error_popup', { msg })
  }

  async function load() {
    let file = await dialog.open({
      directory: false,
      filters: [{ name: 'Kryp', extensions: ['krypj'] }],
      multiple: false,
    })
    if (typeof file === 'string') {
      await invoke('open', {
        filePath: file,
      }).catch(popup)
    }
  }

  async function calculate() {
    await invoke('calculate')
  }
</script>

<svelte:head>
  <title>Kryp</title>
</svelte:head>

<p>Dashboard</p>
<button on:click={load}>Load</button>
<button on:click={calculate}>Calculate</button>
