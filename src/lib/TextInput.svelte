<script lang="ts">
  export let value: string
  export let placeholder = ''
  export let maxlength: number | null = null
  export let action: (el: HTMLInputElement) => void = () => {
    /* noop */
  }
</script>

<input
  type="text"
  bind:value
  use:action
  {placeholder}
  {maxlength}
  class={`w-full relative text-sm py-1 px-1.5 m-0 outline-none transition-all duration-75 ease-md rounded border border-slate-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-40 ${$$props.class}`}
  {...$$restProps}
/>
