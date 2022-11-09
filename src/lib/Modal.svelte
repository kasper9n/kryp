<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { checkShortcut } from './general'

  export let width = '580px'
  export let title: null | string = null
  export let form: (() => void) | undefined = undefined
  $: tag = form === undefined ? 'div' : 'form'
  export let closeIcon = true
  export let cancelOnEscape = true

  const dispatch = createEventDispatcher()
  function onCancel() {
    dispatch('close')
  }

  let lastActiveElement: Element | null = null

  function focus(el: HTMLElement) {
    if (lastActiveElement === null) {
      lastActiveElement = document.activeElement
    }
    el.focus()
  }

  function focusTrap(el: HTMLElement) {
    function getFocusElements() {
      // return el.querySelectorAll(
      //   'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      // )
      return el.querySelectorAll(
        `a[href]:not([tabindex='-1']),\n\
        area[href]:not([tabindex='-1']),\n\
        input:not([disabled]):not([tabindex='-1']),\n\
        select:not([disabled]):not([tabindex='-1']),\n\
        textarea:not([disabled]):not([tabindex='-1']),\n\
        button:not([disabled]):not([tabindex='-1']),\n\
        iframe:not([tabindex='-1']),\n\
        [tabindex]:not([tabindex='-1']),\n\
        [contentEditable=true]:not([tabindex='-1'])`
      )
    }

    if (lastActiveElement === null) {
      lastActiveElement = document.activeElement || document.body
      el.focus()
    }

    function handleKeydown(e: KeyboardEvent) {
      if (checkShortcut(e, 'Tab', { shift: true })) {
        const focusElements = getFocusElements()
        const lastFocusElement = focusElements[focusElements.length - 1]
        if (
          focusElements[0] &&
          document.activeElement?.isSameNode(focusElements[0]) &&
          lastFocusElement instanceof HTMLElement
        ) {
          lastFocusElement.focus()
          e.preventDefault()
        }
      } else if (checkShortcut(e, 'Tab')) {
        const focusElements = getFocusElements()
        const lastFocusElement = focusElements[focusElements.length - 1]
        if (
          document.activeElement?.isSameNode(lastFocusElement) &&
          focusElements[0] instanceof HTMLElement
        ) {
          focusElements[0].focus()
          e.preventDefault()
        }
      } else if (checkShortcut(e, 'Escape') && cancelOnEscape) {
        onCancel()
      }
    }
    el.addEventListener('keydown', handleKeydown)
    return {
      destroy() {
        el.removeEventListener('keydown', handleKeydown)
        if (lastActiveElement instanceof HTMLElement) {
          lastActiveElement.focus()
        }
      },
    }
  }
</script>

<div class="modal cover" on:keydown>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="bg cover" on:click={onCancel} on:mousedown|preventDefault />
  <svelte:element this={tag} class="box" style="width: {width};" use:focusTrap tabindex="-1">
    {#if closeIcon}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <svg
        on:click={onCancel}
        class="absolute right-3 top-3 h-6 p-1.5"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        ><path
          d="M23.954 21.03l-9.184-9.095 9.092-9.174-2.832-2.807-9.09 9.179-9.176-9.088-2.81 2.81 9.186 9.105-9.095 9.184 2.81 2.81 9.112-9.192 9.18 9.1z"
        /></svg
      >
    {/if}
    {#if title !== null}
      <h2>{title}</h2>
    {/if}
    <slot {focus} />
  </svelte:element>
</div>

<style lang="sass">
  .cover
    position: fixed
    width: 100%
    height: 100%
    top: 0px
    left: 0px
  .modal
    display: flex
    align-items: center
    justify-content: center
    padding: 20px
    box-sizing: border-box
    z-index: 10
  .bg
    background-color: rgba(#000000, 0.5)
  .box
    position: relative
    background-color: var(--bg-modal)
    border: 1px solid var(--input-border)
    max-width: 100%
    max-height: 100%
    padding: 22px
    border-radius: 7px
    box-shadow: 0px 0px 30px 0px rgba(#000000, 0.5)
    overflow: auto
    outline: none
  svg
    fill: rgba(#6e6e8c, 1)
    transition: all 100ms ease-out
    &:hover
      fill: var(--text)
      transform: scale(1.1)
</style>
