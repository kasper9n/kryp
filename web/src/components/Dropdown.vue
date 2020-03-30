<template lang='pug'>
.dropdown(
  v-on-clickaway='hide'
  @keydown.esc='hide'
  @keydown.prevent.down='focusNext'
  @keydown.prevent.up='focusPrevious'
  @keydown.enter='selectHighlighted'
  :data-visible='visible'
  @blur='hide()'
  @focus='show()'
  tabindex='0'
)
  .textbox(@mousedown='toggle()')
    .text {{ options[selectedIndex].text }}
    ChevronDownIcon.chevron(size='18')
  .box(v-show='visible')
    template(v-for='(option, index) in options')
      .separator(v-if='option.type === "separator"')
      .text(v-else-if='option.type === "text"') {{ option.text }}
      .space(v-else-if='option.type === "space"')
      .button(
        v-else-if='option.type === "button"'
        :class='{highlight: index === focuedIndex}'
        @mouseover='focuedIndex = index'
        @mousedown='focuedIndex = index'
        @mouseout='focuedIndex = null'
      ) {{ option.text }}
      .option(
        v-else
        :class='{highlight: index === focuedIndex}'
        @mouseover='focuedIndex = index'
        @mousedown='focuedIndex = index'
        @mouseout='focuedIndex = null'
        @click='select(index)'
      ) {{ option.text }}
</template>

<script>
import { mixin as clickaway } from 'vue-clickaway'
import { ChevronDownIcon } from 'vue-feather-icons'

export default {
  mixins: [clickaway],
  components: {
    ChevronDownIcon,
  },
  data: function () {
    const data = {
      visible: false,
    }

    // get the index of the defaultText prop
    for (var i = 0; i < this.options.length; i++) {
      if (this.options[i].text === this.defaultText) {
        data.selectedIndex = i
        data.focuedIndex = i
      }
    }

    return data
  },
  props: {
    options: Array,
    defaultText: String,
  },
  methods: {
    toggle () {
      this.visible = !this.visible
      this.focuedIndex = this.selectedIndex
    },
    hide () {
      this.visible = false
    },
    show () {
      this.visible = true
      this.focuedIndex = this.selectedIndex
    },
    select (index) {
      this.selectedIndex = index
      this.$emit('change', this.options[index])
      this.hide()
    },
    selectHighlighted () {
      this.selectedIndex = this.focuedIndex
      this.$emit('change', this.options[this.focuedIndex])
      this.hide()
    },
    // direction: 1 or -1
    moveFocus (direction) {
      let index = this.focuedIndex

      // start from top/bottom if nothing is focused
      if (index === null && direction === 1) index = -1
      if (index === null && direction === -1) index = this.options.length

      while (true) {
        const nextIndex = index + direction
        const nextOption = this.options[nextIndex]
        if (!nextOption) break
        else if (!nextOption.type || nextOption.type === 'button') {
          this.focuedIndex = nextIndex
          break
        }
        index = nextIndex
      }
    },
    focusNext () {
      this.moveFocus(1)
    },
    focusPrevious () {
      this.moveFocus(-1)
    },
  },
}
</script>

<style lang='sass' scoped>
.dropdown
  width: 200px
  position: relative
  outline: none
  $border-width-increase: 1px
  &[data-visible="true"]
    .textbox
      border-color: var(--line-highlight-color)
      box-shadow: 0px 0px 0px $border-width-increase var(--line-highlight-color)
  &:not([data-visible="true"])
    .textbox:hover
      border-color: var(--line-color)
  .textbox
    display: flex
    align-items: center
    width: auto
    cursor: pointer
    padding: 8px 0px
    height: 36px
    box-sizing: border-box
    transition: all 0.15s
    border-radius: 3px
    border-width: 1px
    border-style: solid
    border-color: transparent
    .text
      overflow: hidden
      text-overflow: ellipsis
      margin-left: 16px
    .chevron
      margin-left: 2px
      margin-right: 10px
      margin-top: 2px
  .box
    position: absolute
    padding: 0px 0px
    background-color: var(--dropdown-background-color)
    box-shadow: var(--highlight-shadow)
    border-radius: 3px
    min-width: 100%
    margin-top: $border-width-increase
    border: 1px solid var(--line-color)
    font-size: 14px
    .space
      height: 8px
    .text
      padding: 6px 16px
      font-size: 12px
      opacity: 0.5
    .separator
      width: 100%
      height: 1px
      margin: 8px 0px
      background-color: var(--light-line-color)
    .button
      padding: 8px 16px
      cursor: pointer
      color: var(--button-color)
      font-weight: 700
      &.highlight
        background-color: var(--button-color)
        color: var(--button-text-color)
    .option
      padding: 8px 16px
      cursor: pointer
      &.highlight
        background-color: var(--button-color)
        color: var(--button-text-color)
</style>
