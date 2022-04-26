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

  async function download() {
    report = await runCmd('download_report', { year: Number(year) })
  }
</script>

<div class="mx-auto max-w-5xl overflow-hidden">
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
    <div class="m-4 bg-white py-2 rounded border border-slate-200">
      <table class="w-full px-4 border-separate text-sm">
        <thead class="flex my-1 font-bold text-left">
          <th class="w-0 flex-grow">Name</th>
          <th class="w-0 flex-grow">Realized</th>
          <th class="w-0 flex-grow">Realized Gain</th>
          <th class="w-0 flex-grow">Realized Loss</th>
        </thead>
        <tbody>
          {#each report.records as row}
            <tr class="flex py-1 border-t border-slate-200">
              <td class="w-0 flex-grow">{row.name}</td>
              <td class="w-0 flex-grow">{row.realized}</td>
              <td class="w-0 flex-grow">{row.realized_gain}</td>
              <td class="w-0 flex-grow">{row.realized_loss}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      <button type="button" class="my-4 text-blue-500 px-2 mx-2" on:click={download}
        >Download</button
      >
    </div>
  {/if}
</div>
