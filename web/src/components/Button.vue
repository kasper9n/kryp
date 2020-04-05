<template lang='pug'>
button(
  :type='type'
  :primary='primary'
  @click='$emit("click")'
  :data-variety='variety'
)
  span(tabindex='-1')
    slot
</template>

<script>
export default {
  props: {
    type: String,
    primary: Boolean,
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
  &:focus
    outline: none
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
    &:focus
      outline: none
  &:focus > span
    box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
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
//
//
//
//
//
// button
//   position: relative
//   z-index: 5
//   background: none
//   border: none
//   font-size: 13px
//   font-family: 'Muli'
//   font-weight: 700
//   margin: 10px
//   padding: 0px
//   cursor: pointer
//   &:focus
//     outline: none
//   span
//     display: inline-block
//     position: relative
//     height: 36px
//     width: 100%
//     box-sizing: border-box
//     line-height: 36px
//     padding: 0px 24px
//     white-space: nowrap
//     color: var(--button-text-color)
//     &:focus
//       outline: none
//     &::before
//       content: ''
//       position: absolute
//       z-index: -1
//       width: 100%
//       height: 100%
//       top: 0px
//       left: 0px
//       background-color: var(--accent-color)
//       border-radius: 3px
//       opacity: 0.9
//       transition: all 0.15s
//   &:focus span::before
//     box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
//   &:hover span::before
//     opacity: 1
//   &:active span::before
//     transform: scale(0.95)
//
//
//
//
//
//
//
//
//
//
//
//
// button
//   background: none
//   border: none
//   position: relative
//   font-size: 13px
//   font-family: 'Muli'
//   font-weight: 700
//   margin: 10px
//   padding: 0px
//   cursor: pointer
//   &:focus
//     outline: none
//   span
//     display: inline-block
//     height: 36px
//     width: 100%
//     box-sizing: border-box
//     line-height: 36px
//     padding: 0px 24px
//     border-radius: 3px
//     white-space: nowrap
//     color: var(--button-text-color)
//     background: var(--accent-color)
//     opacity: 0.9
//     transition: all 0.15s
//     &:focus
//       outline: none
//   &:focus > span
//     box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
//   &:hover span
//     opacity: 1
//   &:active span
//     transform: scale(0.95)
//
//
//
//
//
//
//
//
//
//
// button
//   outline: none
//   user-select: none
//   position: relative
//   background: none
//   border: none
//   padding: 0px
//   font-size: 13px
//   font-family: 'Muli'
//   font-weight: 700
//   margin: 5px
//   cursor: pointer
//   z-index: 1
//   border-radius: 3px
//   span
//     display: inline-block
//     box-sizing: border-box
//     white-space: nowrap
//     height: 36px
//     width: 100%
//     padding: 0px 24px
//     line-height: 36px
//     outline: none
//     color: var(--button-text-color)
//     &:hover
//       transform: scale()
//   &:focus span
//     box-shadow: 0px 0px 0px 3px var(--line-highlight-color)
//   &::before
//     outline: none
//     content: ''
//     z-index: -1
//     width: 100%
//     height: 100%
//     left: 0px
//     top: 0px
//     position: absolute
//     border-radius: 3px
//     background-color: var(--button-color)
//     opacity: 1
//     transition: all 0.2s var(--easing)
//   &:hover::before
//     opacity: 0.9
</style>
