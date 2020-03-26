<style lang='sass'>
:root[data-theme='dark']
  // $accent-color: #ed127c
  // $accent-color: #ed125b
  // $accent-color: #1fbdbd
  $accent-color: #1fb782
  $accent-color: #5c5ee6
  --background-color-1: hsl(0, 0%, 0%)
  --background-color-2: hsl(225, 40%, 2%)
  --background-color-3: hsl(216, 16%, 6%)
  --background-color-4: hsl(228, 11%, 9%)

  --text-color: hsl(0, 0%, 100%)
  --secondary-text-color: hsl(0, 0%, 50%)

  --accent-color: #{ $accent-color }

  --button-text-color: hsl(0, 0%, 100%)
  --button-color: #{ transparentize($accent-color, 0.1) }

  --light-line-color: hsl(0, 0%, 10%)
  --line-color: hsl(210, 2%, 16%)
  --line-hover-color: hsl(220, 2%, 27%)
  --line-highlight-color: #{ transparentize($accent-color, 0.3) }

  --dropdown-background-color: hsl(0, 0%, 8%)

  --header-height: 70px

  --shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.1)
  --highlight-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.15)

  --page-form-width: 400px

  --easing: cubic-bezier(.4,0,.2,1)
</style>
