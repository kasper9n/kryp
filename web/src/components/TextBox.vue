<template lang='pug'>
input(:name='name' :placeholder='placeholder' :type='type')
</template>

<script>
export default {
  props: {
    name: String,
    placeholder: String,
    type: String,
  },
}
</script>

<style lang='sass' scoped>
input
  margin: 10px 0px
  display: block
  height: 48px
  padding: 0px 16px
  box-sizing: border-box
  font-family: inherit
  font-size: inherit
  color: inherit
  font-weight: inherit
  border-radius: 3px
  border-width: 1px
  border-style: solid
  border-color: var(--line-color)
  background-color: var(--background-color-1)
  transition: all 0.2s var(--easing)
  &:focus, &:active
    border-color: var(--line-highlight-color)
    box-shadow: 0px 0px 0px 1px var(--line-highlight-color)
  &:hover:not(:active):not(:focus)
    border-color: var(--line-hover-color)
</style>
