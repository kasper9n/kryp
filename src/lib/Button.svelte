<script lang="ts">
  export let group: unknown[] | null = null
  export let selected = 0
  export let secondary = false
  export let neutral = false
  export let disabled = false
  export let type = 'button'
</script>

{#if group}
  <div class="view">
    {#each group as value, i}
      <div class="wrapper" class:secondary on:click={() => (selected = i)}>
        <button class:selected={selected === i}>{value}</button>
      </div>
    {/each}
  </div>
{:else}
  <div class="wrapper" class:neutral class:secondary class:disabled on:click>
    <button {type} {...$$restProps}><slot /></button>
  </div>
{/if}

<style lang="sass">
  $accent: #3061F6
  $neutral: #242429
  $border: #c6cddd
  .wrapper
    display: inline-block
    vertical-align: middle
  button
    cursor: pointer
    user-select: none
    -webkit-user-select: none
    margin: 0px
    height: 34px
    padding: 0px 18px
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
    button
      background-color: $neutral
      height: 30px
      padding: 0px 22px
      box-shadow: 0px 0px 2px 0px $neutral
  .secondary button
    background-color: #ffffff
    color: #191B20
    border: 1px solid $border
    box-shadow: 0px 0px 2px 0px $border
  .disabled button
    background-color: #191B20
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
    .selected
      color: #ffffff
      background-color: #191B20
      box-shadow: 0px 0px 1px 0px #000000
      border-radius: 4px
</style>
