<template lang='pug'>
canvas
</template>

<script>
import Chart from 'chart.js'
export default {
  data () {
    return {
      chart: null,
    }
  },
  props: {
    labels: Array,
    values: Array,
  },
  methods: {
    newChart () {
      if (this.chart !== null) this.chart.destroy()

      const root = getComputedStyle(document.body)
      const cardBackgroundColor = root.getPropertyValue('--card-background-color')
      const textColor = root.getPropertyValue('--text-color')

      let segment
      const hoverExpansion = 3
      this.chart = new Chart(this.$el, {
        type: 'pie',
        data: {
          labels: this.labels,
          datasets: [
            {
              label: 'Value',
              data: this.values,
              backgroundColor: [
                // b3 = 70% opacity
                '#36a2ebb3',
                '#ff6384b3',
                '#cc65feb3',
                '#ffce56b3',
              ],
              hoverBackgroundColor: [
                '#36a2eb',
                '#ff6384',
                '#cc65fe',
                '#ffce56',
              ],
              borderColor: cardBackgroundColor,
              hoverBorderColor: cardBackgroundColor,
              borderWidth: 0,
              hoverBorderWidth: 0,
            },
          ],
        },
        options: {
          onHover: function (evt, elements) {
            if (elements && elements.length) {
              segment = elements[0]
              this.chart.update()
              // selectedIndex = segment._index
              segment._model.outerRadius += hoverExpansion
            } else {
              if (segment) {
                segment._model.outerRadius -= hoverExpansion
              }
              segment = null
            }
          },
          layout: {
            padding: hoverExpansion,
          },
          hover: {
            mode: 'nearest',
          },
          legend: {
            labels: {
              fontColor: textColor,
            },
          },
          responsive: true,
          cutoutPercentage: 70,
          animation: {
            animateRotate: true,
            animateScale: true,
            duration: 250,
            easing: 'easeOutCubic',
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
  },
  watch: {
    darkTheme () {
      this.newChart()
    },
  },
}
</script>
