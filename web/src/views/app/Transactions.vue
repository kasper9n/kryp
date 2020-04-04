<template lang='pug'>
.page
  h1 Transactions
  Card
    TextBox.search(placeholder='Search' @input='search' compact)
    table(ref='table')
      tr.header
        td(v-for='column of columns' :data-align='column.align')
          | {{ column.title }}
      template(v-for='(row, index) of rows' v-if='row.hide !== true')
        tr.row
          td(v-for='column of columns' :data-align='column.align')
            | {{ row[column.key] }}
</template>

<script>
import Card from '@/components/Card.vue'
import TextBox from '@/components/TextBox.vue'

function debounce (func, wait, immediate) {
  var timeout
  return function () {
    var context = this; var args = arguments
    var later = function () {
      timeout = null
      if (!immediate) func.apply(context, args)
    }
    var callNow = immediate && !timeout
    clearTimeout(timeout)
    timeout = setTimeout(later, wait)
    if (callNow) func.apply(context, args)
  }
}

export default {
  components: {
    Card,
    TextBox,
  },
  data () {
    return {
      columns: [
        { key: 'type', title: 'Type', align: 'left' },
        { key: 'buy', title: 'Buy', align: 'right' },
        { key: 'buyAsset', title: 'Asset', align: 'left' },
        { key: 'sell', title: 'Sell', align: 'right' },
        { key: 'sellAsset', title: 'Asset', align: 'left' },
        { key: 'fee', title: 'Fee', align: 'right' },
        { key: 'feeAsset', title: 'Asset', align: 'left' },
        { key: 'account', title: 'Account', align: 'left' },
        { key: 'note', title: 'Note', align: 'left' },
        { key: 'time', title: 'Time', align: 'left' },
      ],
      rows: [
        {
          type: 'Trade',
          buy: '0.00000001',
          buyAsset: 'BTC',
          sell: '1.00000000',
          sellAsset: 'USD',
          fee: '0.01000000',
          feeAsset: 'USD',
          account: 'Binance',
          time: '2020-03-11 02:32:11',
          hide: false,
        },
        {
          type: 'Deposit',
          buy: '0.00000001',
          buyAsset: 'BTC',
          sell: '',
          sellAsset: '',
          fee: '0.01000000',
          feeAsset: 'USD',
          account: 'Binance',
          time: '2019-08-21 11:23:46',
          hide: false,
        },
        {
          type: 'Gateway Purchase',
          buy: '0.00000001',
          buyAsset: 'BTC',
          sell: '',
          sellAsset: '',
          fee: '0.01000000',
          feeAsset: 'USD',
          account: 'Binance',
          time: '2019-08-21 11:23:46',
          hide: false,
        },
      ],
    }
  },
  methods: {
    search: debounce(function (e) {
      const text = e.target.value.toLowerCase()
      this.rows.forEach((row, index) => {
        for (const cellKey in row) {
          if (!Object.prototype.hasOwnProperty.call(row, cellKey)) continue
          const cellText = String(row[cellKey])
          const match = cellText.toLowerCase().includes(text)
          if (match) {
            row.hide = false
            // this.hide =
            return
          }
        }
        row.hide = true
      })
      this.$forceUpdate()
    }, 60),
  },
}
</script>

<style lang='sass' scoped>
.page
  max-width: 1150px
h1
  margin-left: 8px
  margin-right: 8px
.search
  margin-left: auto
::v-deep tr td
  white-space: nowrap
  width: 10%
  // &:nth-child(9)
  //   min-width: 30%
  &:nth-child(10)
    width: 0px
table
  width: 100%
  margin-top: 10px
  font-size: 13px
  table-layout: auto
  border-collapse: collapse
  tr
    &.header
      font-weight: 700
    &:nth-child(2n)
      background-color: var(--table-alternating-color)
    &:nth-child(2n+1)
      background-color: var(--background-color)
    &.row:hover
      background-color: var(--table-hover-color)
    td
      padding: 7px 5px
      &:first-child
        padding-left: 15px
      &:last-child
        padding-right: 15px
      &[data-align='left']
        text-align: left
      &[data-align='right']
        text-align: right
      &[data-align='center']
        text-align: center
</style>
