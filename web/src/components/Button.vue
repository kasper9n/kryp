<template lang='pug'>
button(:type='type' :primary='primary' @click='$emit("click")')
  span(tabindex='-1')
    slot
</template>

<script>
export default {
  props: {
    type: String,
    primary: Boolean,
  },
}
</script>

<style lang='sass' scoped>
button
  outline: none
  background: none
  color: var(--button-text-color)
  font-weight: inherit
  font-family: inherit
  font-size: inherit
  user-select: none
  margin: 10px
  padding: 0px
  border: none
  border-radius: 3px
  cursor: pointer
  white-space: nowrap
  box-shadow: none
  transition: all 0.15s
  opacity: 1
  span
    transition: all 0.15s
    outline: none
    background-color: var(--button-color)
    width: 100%
    box-sizing: border-box
    padding: 9px 24px
    // line-height: 36px
    display: flex
    justify-content: center
    align-items: center
    border-radius: 3px
  &:hover span
    opacity: 0.9
  &:focus span
    box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
</style>
