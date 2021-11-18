<script lang="ts">
  import { onMount } from 'svelte'

  export let value = ''
  export let invalid = false
  export let noLeftBorder = false
  export let placeholder = ''
  export let style = ''
  let node: HTMLInputElement
  function beforeinput(eee: Event) {
    let e = eee as InputEvent
    e = e as InputEvent
    if (e.data === '.' && node.value.includes('.')) {
      e.preventDefault()
    }
  }

  let mounted: boolean
  onMount(() => {
    mounted = true
  })

  $: if (mounted && value !== '') filter()
  function filter() {
    const length = (value && value.length) || 0
    let start = node.selectionStart
    let end = node.selectionEnd
    let newValue = ''
    let hasPeriod = false
    for (let i = 0; i < length; i++) {
      let char = value.charAt(i)
      if (char === '.') {
        if (!hasPeriod) newValue += char
        hasPeriod = true
      } else if (/[0-9]/.test(char)) {
        newValue += char
      } else if (start !== null && end !== null) {
        start -= 1
        end -= 1
      }
    }
    node.value = newValue
    value = newValue
    if (start !== null && end !== null && node === document.activeElement) {
      node.setSelectionRange(start, end)
    }
  }
</script>

<input
  bind:this={node}
  type="text"
  on:beforeinput={beforeinput}
  class="numeric-input"
  class:invalid
  class:noLeftBorder
  bind:value
  {placeholder}
  {style} />

<style lang="sass">
  input
    min-width: 0px
    width: 100%
    padding: 4px 6px
    margin: 0px
    font-family: inherit
    font-size: inherit
    box-sizing: border-box
    border: 1px solid #c6cddd
    border-radius: 3px
    outline: none
    transition: all 80ms var(--ease)
    &:focus
      border-color: #0269f7
      box-shadow: 0px 0px 0px 2px rgba(#0269f7, 0.4)
  .noLeftBorder
    border-top-left-radius: 0px
    border-bottom-left-radius: 0px
  input:focus
    z-index: 1 // outline fix
  .invalid
    border: 1px solid rgba(#f92f72, 0.5)
    background-color: #fff0f5
</style>
