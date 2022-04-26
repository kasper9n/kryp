<script lang="ts">
  import { runCmd } from '$lib/general'

  let year = new Date().getFullYear() - 1

  type Report = {
    total_income: string
    total_deductible: string
    total_realized_gain: string
    total_realized_loss: string
    total_realized: string
    records: [
      {
        name: string
        income: string
        deductible: string
        realized_gain: string
        realized_loss: string
        realized: string
      }
    ]
  }

  type DWTags = {
    deposit_tags: string[]
    withdrawal_tags: string[]
  } | null
  let tags: DWTags = null
  getDepositWithdrawalTags()
  async function getDepositWithdrawalTags() {
    tags = await runCmd('get_deposit_withdrawal_tags')
    console.log('tags', tags)
  }

  let deductibleTags = ['Lost']
  let incomeTags = ['Income', 'Interest']

  let report: Report | null = null
  $: if (tags) {
    getReport(year, deductibleTags, incomeTags)
  }
  async function getReport(year: number, deductibleTags: string[], incomeTags: string[]) {
    report = null
    report = await runCmd('get_report', {
      year: Number(year),
      deductibleTags,
      incomeTags,
    })
    console.log('report', report)
  }

  async function download() {
    report = await runCmd('download_report', {
      year: Number(year),
      deductibleTags,
      incomeTags,
    })
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

  <div class="flex">
    {#if tags}
      <div class="mx-4 my-4 w-1/2">
        <p class="mr-4">Income tags</p>
        <span class="text-sm opacity-75">In some cases, forks and airdrops may be income</span>
        {#each tags.deposit_tags.sort() as tag}
          <label class="flex items-center px-1">
            <input type="checkbox" class="mr-1" bind:group={incomeTags} value={tag} />
            <div>{tag}</div>
          </label>
        {/each}
      </div>

      <div class="mx-4 my-4 w-1/2">
        <p class="mr-4">Deductible tags</p>
        <span class="text-sm opacity-75"
          >Tags such as Lost that you may be able to deduct in your tax return. Gains are not
          realized for these</span
        >
        {#each tags.withdrawal_tags.sort() as tag}
          <label class="flex items-center px-1">
            <input type="checkbox" class="mr-1" bind:group={deductibleTags} value={tag} />
            <div>{tag}</div>
          </label>
        {/each}
      </div>
    {/if}
  </div>

  {#if report === null}
    Loading...
  {:else}
    <div class="m-4 px-4 py-2 bg-white rounded border border-slate-200 max-w-xl">
      <div class="my-1.5 flex">
        <div class="w-18 mr-auto">Realized gain</div>
        <div class="text-emerald-500 font-medium">+ {report.total_realized_gain}</div>
      </div>
      <div class="my-1.5 flex">
        <div class="w-18 mr-auto">Realized loss</div>
        <div class="text-red-500 font-medium">- {report.total_realized_loss}</div>
      </div>
      <div class="my-1.5 flex">
        <div class="w-18 mr-auto">Realized</div>
        <div class="text-blue-500 font-medium">{report.total_realized}</div>
      </div>
      <div class="border-t my-2" />
      <div class="my-1.5 flex">
        <div class="w-18 mr-auto">Income</div>
        <div class="text-emerald-500 font-medium">+ {report.total_income}</div>
      </div>
      <div class="my-1.5 flex">
        <div class="w-18 mr-auto">Deductible amount</div>
        <div class="text-red-500 font-medium">- {report.total_deductible}</div>
      </div>
    </div>
    <div class="m-4 bg-white py-2 rounded border border-slate-200">
      <table class="w-full px-4 border-separate text-sm">
        <thead class="flex my-1 font-bold text-left">
          <th class="w-0 flex-grow">Name</th>
          <th class="w-0 flex-grow">Income</th>
          <th class="w-0 flex-grow">Deductible</th>
          <th class="w-0 flex-grow">Realized</th>
          <th class="w-0 flex-grow">Realized Gain</th>
          <th class="w-0 flex-grow">Realized Loss</th>
        </thead>
        <tbody>
          {#each report.records as row}
            <tr class="flex py-1 border-t border-slate-200">
              <td class="w-0 flex-grow">{row.name}</td>
              <td class="w-0 flex-grow">{row.income}</td>
              <td class="w-0 flex-grow">{row.deductible}</td>
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
