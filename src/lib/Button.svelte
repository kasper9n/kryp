<script lang="ts">
  export let group: any[] | null = null
  export let selected = 0
  export let secondary = false
  export let disabled = false
  export let type = 'button'
</script>

{#if group}
  <div class="view">
    {#each group as value, i}
      <button class:secondary class:selected={selected === i} on:click={() => (selected = i)}
        >{value}</button>
    {/each}
  </div>
{:else}
  <button class:secondary class:disabled on:click {type} {...$$restProps}><slot /></button>
{/if}

<style lang="sass">
  $accent: #3061F6
  $border: #c6cddd
  button
    cursor: pointer
    user-select: none
    -webkit-user-select: none
    margin: 0px
    height: 36px
    padding: 0px 18px
    border: none
    font-size: 13px
    font-weight: 500
    background-color: $accent
    box-shadow: 0px 0px 2px 0px $accent
    color: #ffffff
    border-radius: 7px
    transition: 0.1s ease-in-out
    transition-property: transform, opacity
    &.secondary
      background-color: #ffffff
      color: #191B20
      border: 1px solid $border
      box-shadow: 0px 0px 2px 0px $border
    &.disabled
      background-color: #191B20
    &:active
      opacity: 0.95
      transform: scale(0.95)
  .view
    display: flex
    background-color: #ffffff
    border-radius: 7px
    box-shadow: 0px 0px 2px 0px $border
    height: 32px
    padding: 2px 2px
    border: 1px solid $border
    button
      box-shadow: none
      background-color: transparent
      border: none
      color: #252935
      height: 100%
      padding: 0px 16px
      // padding: 0px 0px
      // width: 60px
    .selected
      color: #ffffff
      background-color: #191B20
      box-shadow: 0px 0px 1px 0px #000000
      border-radius: 4px
</style>
