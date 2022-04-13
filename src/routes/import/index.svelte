<script lang="ts" context="module">
  import type { Transaction } from '$lib/transactions'

  export type ImportTransaction = {
    transaction: Transaction
    cost: string | null
    error: string | null
  }
  export type ImportData = {
    transactions: ImportTransaction[]
    has_errors: boolean
    source: string
  }
</script>

<script lang="ts">
  import Button from '$lib/Button.svelte'
  import { runCmd } from '$lib/general'
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'
  import { router } from 'tinro'

  let source = 'Kryp'

  let timezone = ''
  runCmd('get_system_timezone').then((tz: string | null) => {
    if (tz) {
      timezone = tz
    }
  })

  async function importFile() {
    const cancelled = await runCmd('scan_import_file', { source, tz: timezone })
    if (!cancelled) {
      router.goto('/import/confirm')
    }
  }

  type ImportStatus = {
    index: number
  }
  let status: ImportStatus | null = null
  const statusUnlisten = event.listen('importStatus', (e) => {
    if (e.payload) {
      status = e.payload as ImportStatus
    }
  })

  onDestroy(async () => {
    const unlisten = await statusUnlisten
    unlisten()
  })
</script>

<h1 class="center">Import</h1>
<div class="page">
  {#if status}
    <p class="center">Scanned {status.index} transactions</p>
  {:else}
    <div class="my-4">
      <div class="mx-auto text-center">Type</div>
      <select class="mx-auto text-center block text-sm" bind:value={source}>
        <option value={'Kryp'}>Kryp</option>
        <option value={'Binance'}>Binance</option>
      </select>
    </div>
    <div class="my-4">
      <div class="mx-auto text-center">Timezone</div>
      {#if source === 'Kryp'}
        <input class="mx-auto block text-sm" type="text" bind:value={timezone} />
      {:else}
        <input class="mx-auto block text-sm" type="text" value="Auto" disabled />
      {/if}
    </div>
    <p class="center">Import a custom CSV or TSV file</p>
    {#if source === 'Binance'}
      <p class="text-red-500 text-center">
        Trades are not supported for Binance "All Statements" files. Trade history needs to be
        imported separately
      </p>
    {/if}
    <div class="center my-4">
      <Button on:click={importFile}>Import</Button>
    </div>
  {/if}
</div>

<style lang="sass">
  .center
    display: flex
    align-items: center
    justify-content: center
  .page
    margin: 10px
</style>
