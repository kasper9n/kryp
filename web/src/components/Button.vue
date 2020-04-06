<template lang='pug'>
button.outline-parent(
  :type='type'
  @click='$emit("click")'
  :data-variety='variety'
)
  span.outline-child(tabindex='-1')
    slot
</template>

<script>
export default {
  props: {
    type: String,
    variety: String,
  },
}
</script>

<style lang='sass' scoped>
button
  background-color: transparent
  border: none
  position: relative
  font-size: 14px
  font-family: 'Muli'
  font-weight: 700
  margin: 10px
  padding: 0px
  cursor: pointer
  span
    display: flex
    align-items: center
    justify-content: center
    height: 36px
    width: 100%
    box-sizing: border-box
    padding: 0px 24px
    white-space: nowrap
    color: var(--button-text-color)
    background-color: var(--accent-color)
    border-radius: 3px
    opacity: 0.9
    transition: all 0.15s
    transform: translate3d(0, 0, 0) scale(1)
  &:hover > span
    opacity: 1
  &:active > span
    transform: translate3d(0, 0, 0) scale(0.95)
button[data-variety='stupid']
  span
    background-color: transparent
    border: 2px solid var(--stupid-button-border-color)
    color: var(--stupid-button-text-color)
    border-radius: 1000px
    padding: 0px 18px
    height: 32px
button[data-variety='medium']
  span
    background-color: transparent
    border: 2px solid var(--button-color)
    color: var(--button-color)
  &:hover > span
    background-color: var(--medium-button-hover-color)

.outline-parent:focus, .outline-parent > .outline-child:focus
    outline: none
.outline-parent:focus > .outline-child
    box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
</style>
