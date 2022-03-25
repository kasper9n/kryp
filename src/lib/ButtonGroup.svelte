<script lang="ts">
  type T = $$Generic
  export let values: T[]
  export let selected = 0
</script>

<div class="view">
  {#each values as value, i}
    <div class="wrapper" on:click={() => (selected = i)}>
      <button class:selected={selected === i}>{value}</button>
    </div>
  {/each}
</div>

<style lang="sass">
  $accent: #3061F6
  $neutral: #242429
  $border: #c6cddd
  .wrapper
    margin: 2px
  .view
    display: flex
    background-color: #ffffff
    border-radius: 7px
    box-shadow: 0px 0px 2px 0px $border
    height: 34px
    border: 1px solid $border
  button
    display: block
    cursor: pointer
    user-select: none
    font-size: 13px
    font-weight: 500
    background-color: transparent
    border: none
    color: #252935
    height: 100%
    padding: 0px 16px
    min-width: 60px
    transition: 0.1s var(--ease)
    transition-property: transform, opacity, box-shadow
    border-radius: 4px
    &:active
      opacity: 0.95
      transform: scale(0.95)
  .selected
    color: #ffffff
    background-color: #191B20
    box-shadow: 0px 0px 1px 0px #000000
</style>
