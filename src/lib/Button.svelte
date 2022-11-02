<script lang="ts">
  export let secondary = false
  export let neutral = false
  export let disabled = false
  export let slim = false
  export let type = 'button'
</script>

<button
  {type}
  class:neutral
  class:slim
  class:secondary
  class:disabled
  on:click
  {disabled}
  {...$$restProps}><slot /></button
>

<style lang="sass">
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
    $accent: #3061F6
    background-color: $accent
    box-shadow: 0px 0px 2px 0px $accent
    color: #ffffff
    border-radius: 7px
    transition: 100ms ease-out
    transition-property: transform, opacity, box-shadow
    &:active
      opacity: 0.95
      transform: scale(0.95)
  .neutral
    margin: 0px 5px
    $neutral: #242429
    background-color: $neutral
    box-shadow: 0px 0px 2px 0px $neutral
    height: 30px
    padding: 0px 22px
  .secondary
    background-color: var(--bg-max)
    color: var(--text)
    border: 1px solid var(--input-border)
    box-shadow: 0px 0px 2px 0px var(--input-border)
  .slim
    height: 28px
  .disabled
    background-color: #191B20
</style>
