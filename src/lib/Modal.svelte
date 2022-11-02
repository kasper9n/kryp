<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte'

  let modalBg: HTMLDivElement
  $: if (modalBg) {
    modalBg.focus()
  }

  export let width = '580px'
  export let title: null | string = null
  export let closeIcon = true

  const dispatch = createEventDispatcher()
  function close() {
    dispatch('close')
  }

  function focus(el: HTMLElement) {
    let lastActiveElement = document.body
    if (document.activeElement instanceof HTMLElement) {
      lastActiveElement = document.activeElement
    }
    tick().then(() => {
      const tabbableEl = el.querySelector('a[href], area, button, input, object, select, textarea')
      if (tabbableEl instanceof HTMLElement) {
        tabbableEl.focus()
      } else {
        el.focus()
      }
    })
    return {
      destroy() {
        lastActiveElement.focus()
      },
    }
  }
</script>

<div class="modal cover" on:keydown tabindex="-1" use:focus>
  <div class="bg cover" on:click={close} tabindex="-1" bind:this={modalBg} />
  <div class="box" style="width: {width};">
    {#if closeIcon}
      <svg
        on:click={close}
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
    <slot />
  </div>
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
  svg
    fill: rgba(#6e6e8c, 1)
    transition: all 100ms ease-out
    &:hover
      fill: var(--text)
      transform: scale(1.1)
</style>
