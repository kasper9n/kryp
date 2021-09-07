<script lang="ts">
  import { checkShortcut } from './general'

  export let value: string
  export let options: string[]
  export let menuMaxHeight = 140
  function filterOptions(options: string[], text: string) {
    let filtered = []
    for (const option of options) {
      if (option.includes(text) && option !== text) {
        filtered.push(option)
      }
    }
    return filtered
  }
  $: filteredOptions = filterOptions(options, value)
  let visible = false
  let selected = ''

  function open() {
    if (!visible) {
      visible = true
      selected = value
    }
  }
  function close() {
    if (visible) {
      visible = false
    }
  }
  function pick(newValue: any) {
    value = newValue
    close()
  }
  let menuEl: HTMLDivElement
  function scrollTo(index: number) {
    let itemEl = menuEl.querySelector(`div:nth-child(${index + 1})`)
    let menuRect = menuEl.getBoundingClientRect()
    let itemRect = itemEl.getBoundingClientRect()
    if (menuRect.bottom < itemRect.bottom) {
      menuEl.scrollTop += itemRect.bottom - menuRect.bottom
    } else if (menuRect.top > itemRect.top) {
      menuEl.scrollTop -= menuRect.top - itemRect.top
    }
  }
  function keydown(e: KeyboardEvent) {
    if (checkShortcut(e, 'ArrowUp')) {
      let i = filteredOptions.findIndex((o) => o === selected)
      if (i >= 1) {
        selected = filteredOptions[i - 1]
        scrollTo(i - 1)
      }
      if (!visible) open()
    } else if (checkShortcut(e, 'ArrowDown')) {
      let i = filteredOptions.findIndex((o) => o === selected)
      if (i < filteredOptions.length - 1) {
        selected = filteredOptions[i + 1]
        scrollTo(i + 1)
      }
      if (!visible) open()
    } else if (checkShortcut(e, 'Enter')) {
      pick(selected)
    } else if (checkShortcut(e, ' ')) {
      if (visible && value === '') {
        pick(selected)
      } else if (!visible) {
        open()
      }
    } else {
      return
    }
    e.preventDefault()
    let el = e.target as HTMLElement
    let menu = el.parentElement
    menu.scrollTo({ left: 0, top: el.clientTop + el.clientHeight })
  }
</script>

<div class="dropdown" class:visible on:keydown={keydown}>
  <input type="text" bind:value on:mousedown={open} on:focus={open} on:blur={close} />
  {#if visible}
    <div
      class="menu"
      bind:this={menuEl}
      on:mouseout={() => (selected = '')}
      on:blur={null}
      style="max-height: {menuMaxHeight}px">
      {#each filteredOptions as option}
        <div
          class="item"
          on:mousedown|preventDefault
          on:click={() => pick(option)}
          on:mouseover={() => (selected = option)}
          on:focus={null}
          class:selected={option === selected}>{option}</div>
      {/each}
    </div>
  {/if}
</div>

<style lang="sass">
  .dropdown
    position: relative
  input, .item
    font-size: 12px
    padding: 4px 6px
  input
    margin: 0px
    border: 1px solid #c6cddd
    border-radius: 3px
  .menu
    position: absolute
    width: 100%
    background-color: #ffffff
    border: 1px solid #c6cddd
    border-top: none
    overflow: auto
    box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.2)
  .item
    padding: 4px 6px
    &.selected
      background-color: #3C63ED
      color: #ffffff
</style>
