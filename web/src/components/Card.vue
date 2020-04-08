<template lang='pug'>
.card
  slot
</template>

<style lang='sass' scoped>
.card
  padding: 20px 20px
  background-color: var(--card-background-color)
  margin: 8px 8px
  border: 1px solid var(--light-line-color)
  border-radius: 3px
</style>
