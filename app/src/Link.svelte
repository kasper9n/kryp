<script lang='ts'>
  import { goto } from '@roxi/routify'
  export let to = './'
  function handleClick() {
    console.log(to)
    $goto(to)
  }
</script>

<template lang='pug'>
  div(on:click="{handleClick}")
    slot
</template>
