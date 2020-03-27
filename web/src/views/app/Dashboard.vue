<template lang='pug'>
.dashboard
  .row
    Card.card
      p Current value
      h2 $541,199,959.54
    Card.card
      p Cost
      h2 $23,530.24
    Card.card
      p Unrealized gain
      h2.positive $23,530.24
    Card.card
      p Realized gain
      h2.negative -$39,529.05
  .row.two
    Card.card.big
      p Assets
      Chart.chart(:options='assetChartOptions')
    Card.card.big
      p Accounts
      Chart.chart(:options='assetChartOptions')
</template>

<script>
import Card from '@/components/Card.vue'
import Chart from '@/components/Chart.vue'
export default {
  title: 'Dashboard',
  components: {
    Card,
    Chart,
  },
  data () {
    const assetChartOptions = {
      type: 'pie',
      data: {
        labels: ['BTC', 'ETH', 'XLM', 'NANO'],
        datasets: [
          {
            label: 'Value',
            data: [20, 45, 25, 10],
            backgroundColor: [
              // b3 = 70% opacity
              '#ff6384b3',
              '#36a2ebb3',
              '#cc65feb3',
              '#ffce56b3',
            ],
            hoverBackgroundColor: [
              '#ff6384',
              '#36a2eb',
              '#cc65fe',
              '#ffce56',
            ],
            borderColor: '#ffffff',
            hoverBorderColor: '#ffffff',
            borderWidth: 5,
            hoverBorderWidth: 2,
          },
        ],
      },
      options: {
        responsive: true,
        cutoutPercentage: 70,
        animation: {
          animateRotate: true,
          animateScale: true,
        },
      },
    }
    return {
      assetChartOptions,
    }
  },
}
</script>

<style lang='sass' scoped>
.dashboard
  padding-top: 20px
  .row
    display: flex
    justify-content: center
    // flex-wrap: wrap
    padding-left: 10px
    padding-right: 10px
    max-width: 1400px
    margin: auto
    p
      font-size: 14px
    @media (max-width: 1100px)
      .card h2
        font-size: 18px
    @media (max-width: 900px)
      flex-wrap: wrap
      div.card
        flex-basis: 40%
    @media (max-width: 450px)
      flex-wrap: wrap
      .card h2
        font-size: 14px
      .card p
        font-size: 12px
    .card
      flex-grow: 1
      flex-basis: 1%
      text-align: center
      border-radius: 3px
      > *
        margin: 0px
        margin-top: 2px
      > :first-child
        margin-top: 0px
      .positive
        color: var(--positive-color)
      .negative
        color: var(--negative-color)
  .row.two
    .big.card
      width: 1px
      flex-basis: 1%
      flex-grow: 1
      .chart
        width: 100%
    @media (max-width: 800px)
      flex-wrap: wrap
      .big.card
        flex-basis: 100%
</style>
