<script>
  import { Link } from "svelte-routing";

  export let to = "";

  function getProps({ location, href, isPartiallyCurrent, isCurrent }) {
    const isActive = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent
    if (isActive) return { class: "active" }
    else return {}
  }
</script>

<style>
  :global(.active) {
    color: #0bcb9b;
  }
</style>

<Link to="{to}" getProps="{getProps}">
  <slot />
</Link>
