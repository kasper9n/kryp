<template lang='pug'>
.page
  .row.cards
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
        :tableColumns=`[
          { key: 'asset', title: 'Asset', align: 'left' },
          { key: 'amount', title: 'Amount', align: 'right' },
          { key: 'value', title: 'Value NOK', align: 'right' },
          { key: 'percent', title: 'Share' , align: 'right'},
        ]`
        chartLabelKey='asset'
        chartValueKey='value'
        :input=`[
            { asset: "BCC", value: 3, amount: "10.32839173", percent: "14.91%" },
            { asset: "BTC", value: 3, amount: "1.32839173", percent: "14.91%" },
            { asset: "ETH", value: 7, amount: "1.32839173", percent: "14.91%" },
            { asset: "AUG", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "DOGE", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "LINK", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "XRP", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "SC", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "NANO", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "XMR", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "ZCH", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "XLM", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "TUSD", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "BCH", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "LTC", value: 1, amount: "1.32839173", percent: "14.91%" },
            { asset: "BNB", value: 1, amount: "1.32839173", percent: "14.91%" },
        ]`
      )
    Card.card.big
      p Accounts
      PieChart.chart(
        :tableColumns=`[
          { key: 'account', title: 'Account', align: 'left' },
          { key: 'value', title: 'Value', align: 'right' },
          { key: 'percent', title: 'Share', align: 'right' },
        ]`
        chartLabelKey='account'
        chartValueKey='value'
        :input=`[
          { account: "Binance", value: 350.81, percent: "50.13%" },
          { account: "Coinbase", value: 190.44, percent: "25.93%" },
          { account: "Bittrex", value: 90.91, percent: "6.19%" },
        ]`
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
.row
  display: grid
  .card
    text-align: center
    min-width: 0px
    > *
      margin: 0px
      margin-top: 2px
    > :first-child
      margin-top: 0px
    .positive
      color: var(--positive-color)
    .negative
      color: var(--negative-color)
.row.cards
  grid-template-columns: 1fr 1fr 1fr 1fr
  @media (max-width: 650px)
    grid-template-columns: 1fr 1fr
  .card
    background-color: transparent
    padding: 10px 0px
    border: none
  h2
    @media (max-width: 950px)
      font-size: 18px
.row.two
  grid-template-columns: 1fr 0.8fr
  @media (max-width: 800px)
    grid-template-columns: 1fr
</style>
