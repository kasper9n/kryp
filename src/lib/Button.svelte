<script lang="ts">
  export let secondary = false
  export let neutral = false
  export let disabled = false
  export let type = 'button'
</script>

<button {type} class:neutral class:secondary class:disabled on:click {disabled} {...$$restProps}
  ><slot /></button
>

<style lang="sass">
  $accent: #3061F6
  $neutral: #242429
  $border: #c6cddd
  button
    cursor: pointer
    user-select: none
    margin: 0px
    height: 32px
    padding: 0px 18px
    min-width: 70px
    border: none
    font-size: 13px
    font-weight: 500
    background-color: $accent
    box-shadow: 0px 0px 2px 0px $accent
    color: #ffffff
    border-radius: 7px
    transition: 0.1s var(--ease)
    transition-property: transform, opacity, box-shadow
    &:active
      opacity: 0.95
      transform: scale(0.95)
  .neutral
    margin: 0px 5px
    background-color: $neutral
    height: 30px
    padding: 0px 22px
    box-shadow: 0px 0px 2px 0px $neutral
  .secondary
    background-color: #ffffff
    color: #191B20
    border: 1px solid $border
    box-shadow: 0px 0px 2px 0px $border
  .disabled
    background-color: #191B20
</style>
