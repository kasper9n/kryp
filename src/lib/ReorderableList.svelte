<script lang="ts">
  type T = $$Generic
  interface $$Slots {
    default: {
      item: T
      index: number
    }
  }
  export let items: T[]
</script>

<ul>
  {#each items as item, i}
    <li>
      <slot {item} index={i} />
    </li>
  {/each}
</ul>

<style lang="sass">
  ul
    list-style: none
    margin: 0px
    padding: 0px
    border: 1px solid hsl(220, 20%, 88%)
    border-radius: 3px
</style>
