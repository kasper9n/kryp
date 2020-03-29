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
      PieChart.chart(
        :labels='["BTC", "ETH", "XLM", "NANO"]',
        :values='[45, 25, 20, 1]',
      )
    Card.card.big
      p Accounts
      PieChart.chart(
        :labels='["Binance", "Coinbase", "Bittrex"]',
        :values='[45, 25, 20]',
      )
</template>

<script>
import Card from '@/components/Card.vue'
import PieChart from '@/components/PieChart.vue'
export default {
  title: 'Dashboard',
  components: {
    Card,
    PieChart,
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
