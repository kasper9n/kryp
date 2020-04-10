<template lang='pug'>
.container(v-show='show' @click.self='close')
  .dialog(ref='dialog')
    h2 {{ title }}
    slot
    .bottom-bar
      slot(name='buttons')
</template>

<script>
export default {
  props: {
    title: String,
  },
  data () {
    return {
      show: false,
    }
  },
  methods: {
    close () {
      this.show = false
    },
    open () {
      this.show = true
    },
  },
}
</script>

<style lang='sass' scoped>
.container
  position: fixed
  width: 100%
  height: 100%
  top: 0px
  left: 0px
  background-color: rgba(#000000, 0.2)
  display: flex
  align-items: center
  justify-content: center
  .dialog
    user-select: auto
    background-color: var(--card-background-color)
    // border: 1px solid var(--light-line-color)
    padding: 35px 40px
    border-radius: 3px
    width: 450px
    @media (max-width: 600px)
      width: 300px
    > h2
      margin-top: 0px
      margin-bottom: 20px
  .bottom-bar
    display: flex
    justify-content: flex-end
    margin-top: 20px
    ::v-deep > *
      margin: 0px
      margin-left: 10px
</style>
