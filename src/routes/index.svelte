<script lang="ts">
  import { runCmd } from '$lib/general'
  import { Chart, registerables } from 'chart.js'
  Chart.register(...registerables)

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

  let chart: Chart<'pie', string[], string> | null = null
  function createChart(el: HTMLCanvasElement, holdings: Holding[]) {
    let ctx = el.getContext('2d')
    if (!ctx) {
      return
    }
    if (chart !== null) {
      chart.destroy()
    }
    chart = new Chart(ctx, {
      type: 'pie',
      data: {
        labels: holdings.map((item) => item.key),
        datasets: [
          {
            label: 'Value',
            backgroundColor: holdings.map((_, index) => getColor(index) + 'cc'),
            hoverBackgroundColor: holdings.map((_, index) => getColor(index)),
            borderColor: '#ffffff',
            hoverBorderColor: '#ffffff',
            data: holdings.map((item) => item.value),
            borderWidth: 2,
            hoverBorderWidth: 0,
          },
        ],
      },
      options: {
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
      },
    })
  }

  type Holding = {
    key: string
    amount: string
    cost: string
    value: string
  }
  const holdings: Promise<Holding[]> = runCmd('get_holdings')
</script>

<div class="page">
  <div class="card">
    <h3>Balance by Currency</h3>
    {#await holdings then holdings}
      <div class="canvas-container">
        <canvas use:createChart={holdings} />
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
            <td>{holding.key}</td>
            <td class="align-right">{holding.amount}</td>
            <td class="align-right">{holding.cost}</td>
            <td class="align-right">{holding.value}</td>
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
