<template lang='pug'>
.container(:class='{"expandable-table": items.length > 5}')
  .chart-container
    canvas(ref='canvas')
  table(:class='{expanded: expanded}')
    tr.header
      td(v-for='tableColumn of tableColumns' :data-align='tableColumn.align')
        | {{ tableColumn.title }}
    template(v-for='(item, index) of items')
      transition(name='fade')
        tr.row(v-if='expanded || index < 5' :style='"transition-delay: "+5*(index-5)+"ms"')
          td(v-for='tableColumn of tableColumns' :data-align='tableColumn.align')
            | {{ item[tableColumn.key] }}
  Button.toggle-all(variety='stupid' @click='expanded = !expanded')
    | {{ expanded ? "Show less" : "Show all" }}
</template>

<script>
import Chart from 'chart.js'
import Button from '@/components/Button.vue'
Chart.defaults.global.defaultFontFamily = 'Muli'
export default {
  components: {
    Button,
  },
  data () {
    return {
      chart: null,
      expanded: false,
    }
  },
  props: {
    input: Array,
    tableColumns: Array,
    chartLabelKey: String,
    chartValueKey: String,
  },
  methods: {
    newChart () {
      if (this.chart !== null) this.chart.destroy()

      const root = getComputedStyle(document.body)
      const cardBackgroundColor = root.getPropertyValue('--card-background-color')
      const textColor = root.getPropertyValue('--text-color')

      let segment
      this.chart = new Chart(this.$refs.canvas, {
        type: 'pie',
        data: {
          labels: this.chartLabels,
          datasets: [
            {
              label: 'Value',
              data: this.chartValues,
              backgroundColor: [
                '#3d89e1' + 'd4',
                '#546c78' + 'd4',
                '#42ebc1' + 'd4',
                '#00c8ff' + 'd4',
                '#8da4b0' + 'd4',
                '#ce226f' + 'd4',
                '#e053f9' + 'd4',
                '#e9672f' + 'd4',
                '#fbd941' + 'd4',
                '#966a5a' + 'd4',
                '#63d969' + 'd4',
                '#664181' + 'd4',
                '#4450f8' + 'd4',
                '#f53d3d' + 'd4',
                '#c9c9c9' + 'd4',
              ],
              hoverBackgroundColor: [
                '#3d89e1',
                '#546c78',
                '#42ebc1',
                '#00c8ff',
                '#8da4b0',
                '#ce226f',
                '#e053f9',
                '#e9672f',
                '#fbd941',
                '#966a5a',
                '#63d969',
                '#664181',
                '#4450f8',
                '#f53d3d',
                '#c9c9c9',
              ],
              borderColor: cardBackgroundColor,
              hoverBorderColor: cardBackgroundColor,
              borderWidth: 0,
              hoverBorderWidth: 0,
            },
          ],
        },
        scales: {
          xAxes: [{
            type: 'category',
            afterFit: function (me) {
              me.paddingLeft = 0
              me.paddingRight = 0
            },
          }],
        },
        options: {
          onHover: function (evt, elements) {
            if (elements && elements.length) {
              segment = elements[0]
              this.chart.update()
              segment._model.outerRadius += 5
            } else {
              if (segment) {
                segment._model.outerRadius -= 5
              }
              segment = null
            }
          },
          layout: {
            padding: 5,
          },
          defaultFontFamily: 'Helvetica',
          legend: {
            display: false,
            labels: {
              fontColor: textColor,
            },
          },
          responsive: true,
          aspectRatio: 1,
          cutoutPercentage: 70,
          animation: {
            animateRotate: true,
            animateScale: true,
            duration: 250,
          },
        },
      })
    },
  },
  mounted () {
    this.newChart()
  },
  computed: {
    darkTheme () {
      return this.$pocket.darkTheme
    },
    items () {
      const input = this.input
      return input.slice(0).sort((a, b) => b.value - a.value)
    },
    chartLabels () {
      return this.items.map((item) => item[this.chartLabelKey])
    },
    chartValues () {
      return this.items.map((item) => item[this.chartValueKey])
    },
  },
  watch: {
    darkTheme () {
      this.newChart()
    },
  },
}
</script>

<style lang='sass' scoped>
.chart-container
  margin: auto
  width: 250px
  @media (max-width: 1000px)
    width: 200px
canvas
  margin-top: 10px
.toggle-all
  display: none
  margin-bottom: 0px
.expandable-table
  .toggle-all
    display: inline-block
table
  width: calc(100% - 20px)
  margin-left: auto
  margin-right: auto
  margin-top: 10px
  font-size: 13px
  table-layout: auto
  border-collapse: collapse
  tr
    border-bottom: 1px solid var(--light-line-color)
    &.header
      font-weight: 700
    &:nth-child(2n)
      // background-color: var(--background-color-2)
      background-color: var(--table-alternating-color)
    &:nth-child(2n+1)
      background-color: var(--background-color)
    &.row:hover
      background-color: var(--table-hover-color)
    td
      padding: 6px 5px
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
