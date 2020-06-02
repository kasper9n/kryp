<style lang='sass'>
:root[data-theme='dark']
  // $accent-color: #ed127c
  // $accent-color: #ed125b
  // $accent-color: #1fbdbd
  $accent-color: #5c5ee6
  // $accent-color: #1fb782
  // $accent-color: #2b43f7
  // $accent-color: #f7622b
  --background-color-1: hsl(0, 0%, 5%)
  --background-color-2: hsl(225, 40%, 0%)
  --background-color-3: hsl(216, 16%, 0%)
  --background-color-4: hsl(228, 11%, 0%)
  --card-background-color: #181820

  --positive-color: #15b28d
  --negative-color: #e73253

  --text-color: #ffffff
  --secondary-text-color: hsl(0, 0%, 50%)

  --accent-color: #{ $accent-color }

  --button-text-color: hsl(0, 0%, 100%)
  --button-color: #{ transparentize($accent-color, 0.1) }
  --medium-button-hover-color: #{ transparentize($accent-color, 0.9) }
  --stupid-button-border-color: #dddddd
  --stupid-button-text-color: #ffffff

  --table-hover-color: #31313f
  --table-alternating-color: #1e1f2a

  --light-line-color: hsl(0, 0%, 15%)
  --line-color: hsl(210, 2%, 20%)
  --line-hover-color: hsl(220, 2%, 31%)
  --line-highlight-color: #{ transparentize($accent-color, 0.3) }

  --error-color: hsl(349, 79%, 55%)
  --error-color-light: hsl(349, 77%, 40%)
  --error-background-color: hsl(349, 89%, 10%)

  --dropdown-background-color: hsl(0, 0%, 5%)

  --header-height: 70px

  --shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.1)
  --highlight-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.5)

  --easing: cubic-bezier(.4,0,.2,1)
</style>
