<script lang="ts">
  import { checkShortcut } from './general'

  export let value: Option
  let text = value.name
  type Option = { name: string; [key: string]: unknown }
  export let options: Option[]
  let dragSwitching = false
  export let menuMaxHeight = 300
  function filterOptions(options: Option[], text: string) {
    let filtered = []
    const lcText = text.toLowerCase()
    for (const option of options) {
      let lcOption = option.name.toLowerCase()
      if (lcOption.startsWith(lcText)) {
        filtered.push(option)
      }
    }
    return filtered
  }
  $: filteredOptions = filterOptions(options, text)
  $: if (text.length > 0 && filteredOptions.length > 0) {
    selected = filteredOptions[0]
  }
  let visible = false
  let focused = false
  let selected: Option = { name: '' }
  function onInput(eee: Event) {
    let e = eee as InputEvent
    if (focused && !visible) {
      if (e.inputType === 'insertText' && e?.data) {
        open()
        text = e.data
      } else {
        e.preventDefault()
      }
    }
  }

  let inputEl: HTMLInputElement
  function open() {
    visible = true
    focused = true
    text = ''
    selected = value
    inputEl.focus()
  }
  function close() {
    visible = false
    text = value.name
  }
  function focus() {
    focused = true
  }
  function defocus() {
    focused = false
    close()
  }
  function fieldMouseDown() {
    if (visible) {
      close()
    } else {
      open()
      dragSwitching = true
    }
  }
  function itemMouseUp(newValue: Option) {
    if (dragSwitching) pick(newValue)
  }
  function pick(newValue: Option) {
    value = newValue
    text = newValue.name
    close()
  }
  let menuEl: HTMLDivElement
  function scrollTo(index: number) {
    let itemEl = menuEl.querySelector(`div:nth-child(${index + 1})`)
    if (itemEl) {
      let menuRect = menuEl.getBoundingClientRect()
      let itemRect = itemEl.getBoundingClientRect()
      if (menuRect.bottom < itemRect.bottom) {
        menuEl.scrollTop += itemRect.bottom - menuRect.bottom
      } else if (menuRect.top > itemRect.top) {
        menuEl.scrollTop -= menuRect.top - itemRect.top
      }
    }
  }
  function keydown(e: KeyboardEvent) {
    if (checkShortcut(e, 'ArrowUp')) {
      let i = filteredOptions.findIndex((o) => o.name === selected.name)
      if (i >= 1) {
        selected = filteredOptions[i - 1]
        scrollTo(i - 1)
      }
      if (!visible) open()
    } else if (checkShortcut(e, 'ArrowDown')) {
      let i = filteredOptions.findIndex((o) => o.name === selected.name)
      if (i < filteredOptions.length - 1) {
        selected = filteredOptions[i + 1]
        scrollTo(i + 1)
      }
      if (!visible) open()
    } else if (checkShortcut(e, 'Enter')) {
      if (selected.name !== '') {
        pick(selected)
      }
    } else if (checkShortcut(e, ' ')) {
      if (visible && text === '') {
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
    if (menu) {
      menu.scrollTo({ left: 0, top: el.clientTop + el.clientHeight })
    }
  }
</script>

<svelte:window on:mouseup={() => (dragSwitching = false)} />
<div class="dropdown" class:focused on:keydown={keydown}>
  <div class="field" on:mousedown|preventDefault={fieldMouseDown}>
    <input
      bind:this={inputEl}
      bind:value={text}
      on:focus={focus}
      on:blur={defocus}
      placeholder={value.name}
      class:hide-cursor={focused && !visible}
      on:beforeinput={onInput} />
    <div class="icon">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
        ><path d="M5 3l3.057-3 11.943 12-11.943 12-3.057-3 9-9z" /></svg>
    </div>
  </div>
  <div
    class="menu"
    class:visible
    bind:this={menuEl}
    on:mouseout={() => (selected = { name: '' })}
    on:blur={() => (selected = { name: '' })}
    style="max-height: {menuMaxHeight}px">
    {#each filteredOptions as option}
      <div
        class="item"
        on:mousedown|preventDefault
        on:mouseup={() => itemMouseUp(option)}
        on:click={() => pick(option)}
        on:mouseover={() => (selected = option)}
        on:focus={() => (selected = option)}>
        <slot {option} selected={option === selected}>
          <div class="default-item" class:selected={option === selected}>{option.name}</div>
        </slot>
      </div>
    {/each}
    {#if filteredOptions.length === 0}
      <div class="no-options">No options</div>
    {/if}
  </div>
</div>

<style lang="sass">
  .dropdown
    position: relative
    width: var(--dropdown-width, 100%)
  input
    font-size: 12px
    padding: 4px 8px
  .field
    display: flex
    align-items: center
    margin: 0px
    background-color: #ffffff
    border: 1px solid #c6cddd
    border-radius: 3px
    transition: all 80ms var(--ease)
    &:active .icon
      transform: translateY(1px)
    :active .icon
      // For some weird reason on macOS 10.15.7, if you trigger the transition,
      // defocus the dropdown and switch to a diferent window, the transition
      // will stop working. This line fixes that (don't ask me why)
      transform: translateY(1px)
    .icon
      transition: transform 120ms var(--ease)
      display: flex
      padding: 4px
      padding-right: 6px
      svg
        width: 10px
        height: 10px
        opacity: 0.8
        background: #e7e9f4
        padding: 3px
        border-radius: 4px
        transform: rotate(90deg)
  .focused .field
    border-color: #0269f7
    box-shadow: 0px 0px 0px 2px rgba(#0269f7, 0.4)
  input
    min-width: 10px
    width: 100%
    outline: none
    border: none
    user-select: text
    -webkit-user-select: text
    background-color: transparent
    cursor: default
    margin: 0px
    &.hide-cursor
      color: transparent
      text-shadow: 0 0 0 #000000
      user-select: none
      -webkit-user-select: none
  .menu
    display: none
    font-size: 12px
    position: absolute
    margin-top: 1px
    width: 100%
    background-color: #ffffff
    border: 1px solid #c6cddd
    box-sizing: border-box
    border-top: none
    border-radius: 3px
    overflow: auto
    box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.2)
    z-index: 10
    &.visible
      display: block
  .default-item
    padding: 4px 8px
    &.selected
      background-color: #3C63ED
      color: #ffffff
  .no-options
    color: #a6a6a6
    text-align: center
</style>
