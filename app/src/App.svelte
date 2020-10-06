<script>
  export let name = '';
  import { Router } from "@roxi/routify";
  import { routes } from "../.routify/routes.js";
  import Nested from './Nested.svelte';
</script>

<style lang='sass'>
  .note
    color: #ff3e00
  h1
    color: blue
</style>


<template lang='pug'>
  h1.note This is the svelte-preproces example - built with svite
  hr
  h1 Hello!
  br
  input(bind:value="{name}")
  br
  Nested('{name}')
  Router('{routes}')
</template>
