<template lang='pug'>
.container
  canvas
</template>

<script>
import Chart from 'chart.js'
export default {
  props: {
    options: Object,
  },
  mounted () {
    return new Chart(this.$el.children[0], this.options)
  },
}
</script>

<style lang='sass'>
.container
  position: relative
</style>
