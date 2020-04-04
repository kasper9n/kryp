<template lang='pug'>
.container(:class='{"expandable-table": items.length > 5}')
  .chart-container
    canvas(ref='canvas')
  table(:class='{expanded: expanded}')
    tr.header
      td(v-for='tableColumn of tableColumns' :data-align='tableColumn.align')
        | {{ tableColumn.title }}
    template(v-for='(item, index) of items')
      tr.row(
        v-if='expanded || index < 5'
        :class='{ highlighted: index == hoveredIndex }'
        @mouseover='tableRowHover($event, index)'
        @mouseout='tableRowHover($event, index)'
      )
        //- :style='"transition-delay: "+5*(index-5)+"ms"'
        td(v-for='tableColumn of tableColumns' :data-align='tableColumn.align')
          | {{ item[tableColumn.key] }}
  Button.toggle-all(variety='stupid' @click='expanded = !expanded')
    | {{ expanded ? "Show less" : "Show all" }}
</template>

<script>
import Chart from 'chart.js'
import Button from '@/components/Button.vue'
Chart.defaults.global.defaultFontFamily = 'Muli'

function getChartColors (suffix = '') {
  return [
    '#3d89e1' + suffix,
    '#546c78' + suffix,
    '#42ebc1' + suffix,
    '#00c8ff' + suffix,
    '#8da4b0' + suffix,
    '#ce226f' + suffix,
    '#e053f9' + suffix,
    '#e9672f' + suffix,
    '#fbd941' + suffix,
    '#966a5a' + suffix,
    '#63d969' + suffix,
    '#664181' + suffix,
    '#4450f8' + suffix,
    '#f53d3d' + suffix,
    '#c9c9c9' + suffix,
  ]
}

export default {
  components: {
    Button,
  },
  data () {
    return {
      chart: null,
      expanded: false,
      hoveredIndex: null,
      tableHovered: false,
    }
  },
  props: {
    input: Array,
    tableColumns: Array,
    chartLabelKey: String,
    chartValueKey: String,
  },
  methods: {
    tableRowHover (e, index) {
      if (e.type === 'mouseover') {
        const backgroundColors = getChartColors('4d')
        backgroundColors[index] = backgroundColors[index].slice(0, -2)
        this.chart.data.datasets[0].backgroundColor = backgroundColors
        // temporarily disable tooltip to about tooltip glitching/blinking
        this.chart.options.tooltips.enabled = false
        this.chart.update()

        // increase radios of the corresponding chart model
        const meta = this.chart.getDatasetMeta(0)
        const model = meta.data[index]._model
        model.outerRadius += 5
        this.tableHovered = true
      } else if (this.tableHovered === true) {
        this.tableHovered = false
        this.chart.data.datasets[0].backgroundColor = getChartColors('d4')
        this.chart.options.tooltips.enabled = true
        this.chart.update()
      }
    },
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
              backgroundColor: getChartColors('d4'),
              hoverBackgroundColor: getChartColors(),
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
          onHover: (evt, elements) => {
            if (elements && elements.length) {
              segment = elements[0]

              // highlight table row
              this.hoveredIndex = segment._index

              // reset all radiuses
              this.chart.update()
              // fade all models' backgroundColor
              this.chart.data.datasets[0].backgroundColor = getChartColors('4d')
              // increase radius of current model
              segment._model.outerRadius += 5
            } else if (segment) {
              if (segment) {
                segment._model.outerRadius -= 5
              }
              segment = null
              this.hoveredIndex = null
              this.chart.data.datasets[0].backgroundColor = getChartColors('d4')
              this.chart.update()
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
    &.header
      font-weight: 700
    &:nth-child(2n)
      // background-color: var(--background-color-2)
      background-color: var(--table-alternating-color)
    &:nth-child(2n+1)
      background-color: var(--background-color)
    &.row:hover, &.row.highlighted
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
