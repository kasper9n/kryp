<script lang="ts">
  import { runCmd } from '$lib/general'
  import { Pie } from 'svelte-chartjs'
  import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, CategoryScale } from 'chart.js'
  import { darkMode } from '$lib/DarkMode'

  ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale)

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
  type Holdings = {
    list: Holding[]
    total_cost: string
    total_value: string | null
  }
  type WalletHoldings = {
    name: string
    holdings: Holdings
  }

  let holdings: Holdings = { list: [], total_cost: '', total_value: null }
  let chartHoldings = [] as ChartItem[]
  let walletHoldings = {} as { [wallet: string]: WalletHoldings }

  async function getHoldings() {
    holdings = await runCmd('get_holdings')
    holdings = await runCmd('get_holdings_valued')

    chartHoldings = holdings.list
      .filter((holding) => holding.value !== null)
      .map((holding) => ({
        label: holding.asset,
        value: Number(holding.value),
      }))

    walletHoldings = await runCmd('get_holdings_by_wallet')
  }
  getHoldings()

  function getValue(value: string | null) {
    if (value) {
      return (Number(value) + Number.EPSILON).toFixed(2)
    } else {
      return ''
    }
  }

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
        borderColor: $darkMode ? '#000000' : '#ffffff',
        hoverBorderColor: $darkMode ? '#000000' : '#ffffff',
        data: chartHoldings.map((item) => Number(item.value)),
        borderWidth: 2,
        hoverBorderWidth: 0,
      },
    ],
  }

  const options = {
    cutout: '75%',
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
  <div class="row">
    <div class="big-card max-card">
      <h3>Balance by Asset</h3>
      {#await holdings}
        Loading...
      {:then holdings}
        <div>
          <div class="tr header">
            <div class="asset">Asset</div>
            <div class="align-right amount">Amount</div>
            <div class="align-right cost">Cost</div>
            <div class="align-right value">Value</div>
          </div>
          {#each holdings.list as holding}
            <div class="tr">
              <div class="asset">{holding.asset}</div>
              <div class="align-right amount">{holding.amount}</div>
              <div class="align-right cost">{holding.cost}</div>
              <div class="align-right value">{getValue(holding.value)}</div>
            </div>
          {/each}
        </div>
      {/await}
    </div>
    <div class="sidebar">
      {#if chartHoldings.length >= 1}
        <div class="center">
          <Pie {data} {options} />
        </div>
      {/if}
    </div>
  </div>
  <div class="row">
    <div class="big-card">
      <h3>Balance by Wallet</h3>
      {#await walletHoldings}
        Loading...
      {:then walletHoldings}
        {#each Object.values(walletHoldings) as wallet}
          <div>{wallet.name}</div>
          <div class="wallet">
            <div class="header tr">
              <div class="asset">Asset</div>
              <div class="align-right amount">Amount</div>
              <div class="align-right cost">Cost</div>
            </div>
            {#each wallet.holdings.list as holding}
              <div class="tr">
                <td class="asset">{holding.asset}</td>
                <td class="align-right amount">{holding.amount}</td>
                <td class="align-right cost">{holding.cost}</td>
              </div>
            {/each}
          </div>
        {/each}
      {/await}
    </div>
    <div class="sidebar" />
  </div>
</div>

<style lang="sass">
  h3
    margin: 0px
    display: block
    margin-bottom: 10px
    font-size: 16px
  .align-right
    text-align: right
  .page
    margin: 20px auto
    max-width: 950px
  .row
    display: flex
    flex-wrap: wrap
    margin: 20px
  .big-card
    font-size: 14px
    padding: 15px
    border: 1px solid hsla(0, 0%, 50%, 0.2)
    border-radius: 3px
    width: 550px
    flex-grow: 1
    max-width: 650px
    margin: 0px auto
  .sidebar
    height: 100%
    display: flex
    align-items: center
    width: 180px
    margin: 20px
    margin-right: 0px
    flex-grow: 1
  .tr
    display: flex
  .center
    width: 100%
    max-width: 300px
    margin: auto
  .header
    font-weight: 600
  .asset
    width: 0px
    flex-grow: 6
  .amount
    width: 0px
    flex-grow: 10
  .cost
    width: 0px
    flex-grow: 10
  .value
    width: 0px
    flex-grow: 10
  .wallet
    padding: 10px
</style>
