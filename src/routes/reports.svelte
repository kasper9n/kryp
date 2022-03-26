<script lang="ts">
  import { runCmd } from '$lib/general'

  let year = new Date().getFullYear() - 1

  type Report = {
    realized_gain: string
    realized_loss: string
    realized: string
    records: [
      {
        name: string
        realized_gain: string
        realized_loss: string
        realized: string
      }
    ]
  }

  let report: Report | null = null
  $: getReport(year)
  async function getReport(year: number) {
    report = null
    report = await runCmd('get_report', { year: Number(year) })
    console.log('report', report)
  }
</script>

<div class="mx-auto max-w-5xl">
  <div class="mx-4 my-4 flex items-center">
    <p class="mr-4">Year</p>
    <input type="number" bind:value={year} />
  </div>
  <div class="mx-4 my-4 flex items-center">
    <p class="mr-4">Cost basis method</p>
    FIFO
  </div>

  {#if report === null}
    Loading...
  {:else}
    <div class="m-4 px-4 py-2 bg-white rounded border border-slate-200 max-w-xl">
      <div class="my-1 flex">
        <div class="w-18 mr-auto">Realized gain</div>
        <div class="text-emerald-400 font-medium">+ {report.realized_gain}</div>
      </div>
      <div class="my-1 flex">
        <div class="w-18 mr-auto">Realized loss</div>
        <div class="text-red-400 font-medium">- {report.realized_loss}</div>
      </div>
      <div class="my-1 flex">
        <div class="w-18 mr-auto">Realized</div>
        <div class="text-blue-400 font-medium">{report.realized}</div>
      </div>
    </div>
    <div class="m-4 px-4 py-2 bg-white rounded border border-slate-200 text-sm">
      <div class="flex my-1 font-bold">
        <div class="min-w-0 w-full flex-grow-1">Name</div>
        <div class="min-w-0 w-full flex-grow-1">Realized</div>
        <div class="min-w-0 w-full flex-grow-1">Realized Gain</div>
        <div class="min-w-0 w-full flex-grow-1">Realized Loss</div>
      </div>
      {#each report.records as row}
        <div class="flex py-1 border-t border-slate-200">
          <div class="min-w-0 w-full flex-grow-1">{row.name}</div>
          <div class="min-w-0 w-full flex-grow-1">{row.realized}</div>
          <div class="min-w-0 w-full flex-grow-1">{row.realized_gain}</div>
          <div class="min-w-0 w-full flex-grow-1">{row.realized_loss}</div>
        </div>
      {/each}
    </div>
  {/if}
</div>
