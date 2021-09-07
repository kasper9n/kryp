<script lang="ts">
  import { onMount } from 'svelte'

  export let value = ''
  export let invalid = false
  export let noLeftBorder = false
  export let placeholder = ''
  let node: HTMLInputElement
  function beforeinput(e: any) {
    e = e as InputEvent
    if (e.data === '.' && node.value.includes('.')) {
      e.preventDefault()
    }
  }

  let mounted: boolean
  onMount(() => {
    mounted = true
  })

  $: if (mounted) filter(value)
  function filter(_: string) {
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
      } else {
        start -= 1
        end -= 1
      }
    }
    node.value = newValue
    value = newValue
    node.setSelectionRange(start, end)
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
  {placeholder} />

<style lang="sass">
  input
    min-width: 0px
    width: 100%
    padding: 4px 6px
    margin: 0px
    font-family: inherit
    font-size: inherit
    border: 1px solid #c6cddd
    border-radius: 3px
  .noLeftBorder
    border-top-left-radius: 0px
    border-bottom-left-radius: 0px
  input:focus
    z-index: 1 // outline fix
  .invalid
    border: 1px solid rgba(#f92f72, 0.5)
    background-color: #fff0f5
</style>
