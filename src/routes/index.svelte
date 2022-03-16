<script lang="ts">
  import { runCmd } from '$lib/general'
  import { Pie } from 'svelte-chartjs'

  type ChartItem = {
    label: string
    value: number
  }
  type Holding = {
    asset: string
    amount: string
    cost: string
    value: string | null
    error: string | null
  }
  let holdings = [] as Holding[]
  let chartHoldings = [] as ChartItem[]
  async function getHoldings() {
    holdings = await runCmd('get_holdings')
    holdings = await runCmd('get_holdings_valued')

    chartHoldings = holdings
      .filter((holding) => holding.value !== null)
      .map((holding) => ({
        label: holding.asset,
        value: Number(holding.value),
      }))
  }
  getHoldings()

  const colors = [
    '#296ec2',
    '#1279f8',
    '#3bc8f7',
    '#5df6f8',
    '#85fac9',
    '#a7fbb8',
    '#85fac9',
    '#5df6f8',
    '#3bc8f7',
    '#1279f8',
    '#296ec2',
  ]
  function getColor(index: number) {
    return colors[index % colors.length]
  }

  $: data = {
    labels: chartHoldings.map((item) => item.label),
    datasets: [
      {
        label: 'Value',
        backgroundColor: chartHoldings.map((_, index) => getColor(index) + 'cc'),
        hoverBackgroundColor: chartHoldings.map((_, index) => getColor(index)),
        borderColor: '#ffffff',
        hoverBorderColor: '#ffffff',
        data: chartHoldings.map((item) => Number(item.value)),
        borderWidth: 2,
        hoverBorderWidth: 0,
      },
    ],
  }
  const options = {
    cutout: '65%',
    plugins: {
      legend: {
        display: false,
      },
      tooltip: {
        animation: {
          duration: 240,
        },
      },
    },
  }
</script>

<div class="page">
  <div class="card">
    <h3>Balance by Asset</h3>
    {#await holdings}
      Loading...
    {:then holdings}
      <div class="canvas-container">
        {#if chartHoldings.length >= 1}
          <Pie {data} {options} />
        {/if}
      </div>
      <table>
        <tr class="header">
          <td>Asset</td>
          <td>Amount</td>
          <td>Cost</td>
          <td>Value</td>
        </tr>
        {#each holdings as holding}
          <tr>
            <td>{holding.asset}</td>
            <td class="align-right">{holding.amount}</td>
            <td class="align-right">{holding.cost}</td>
            <td class="align-right">{holding.value || ''}</td>
          </tr>
        {/each}
      </table>
    {/await}
  </div>
</div>

<style lang="sass">
  h3
    margin: 0px
    display: block
    margin-bottom: 10px
  .page
    padding: 20px
    margin: auto
  .card
    font-size: 14px
    width: 70%
    padding: 15px
    border: 1px solid #e5e5e5
    border-radius: 3px
    background-color: #ffffff
  .canvas-container
    max-width: 150px
  .header
    font-weight: 600
</style>
