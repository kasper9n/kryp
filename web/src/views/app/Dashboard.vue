<template lang='pug'>
.dashboard
  .row
    Card
      p Current value
      h2 $541,199,959.54
    Card
      p Cost
      h2 $23,530.24
    Card
      p Unrealized gain
      h2.positive $23,530.24
    Card
      p Realized gain
      h2.negative -$39,529.05
  p Logged in: {{ $account.loggedIn }}
  p Email: {{ $account.email }}
  p firstName: {{ $account.firstName }}
  p lastName: {{ $account.lastName }}
</template>

<script>
import Card from '@/components/Card.vue'
export default {
  title: 'Dashboard',
  components: {
    Card,
  },
}
</script>

<style lang='sass' scoped>
.dashboard
  .row
    display: flex
    justify-content: center
    // flex-wrap: wrap
    padding: 20px 10px
    max-width: 1400px
    margin: auto
    p
      font-size: 14px
    @media (max-width: 1100px)
      .card h2
        font-size: 18px
    @media (max-width: 900px)
      flex-wrap: wrap
      div.card
        flex-basis: 40%
    @media (max-width: 450px)
      flex-wrap: wrap
      .card h2
        font-size: 14px
      .card p
        font-size: 12px
    .card
      flex-grow: 1
      flex-basis: 1%
      text-align: center
      border-radius: 3px
      border: 1px solid var(--light-line-color)
      > *
        margin: 0px
        margin-top: 2px
      > :first-child
        margin-top: 0px
      .positive
        color: var(--positive-color)
      .negative
        color: var(--negative-color)
</style>
