<style lang='sass'>
:root[data-theme='light']
  // $accent-color: #ed127c
  // $accent-color: #ed125b
  // $accent-color: #1fbdbd
  $accent-color: #5c5ee6
  // $accent-color: #1fb782
  // $accent-color: #2b43f7
  // $accent-color: #f7622b
  --background-color-1: #ffffff
  --background-color-2: #f8f9fc
  --background-color-3: #edeff2
  --background-color-4: #e6e7eb
  --card-background-color: #ffffff

  --positive-color: #15b28d
  --negative-color: #e73253

  --text-color: #444444
  --secondary-text-color: hsl(0, 0%, 50%)

  --accent-color: #{ $accent-color }

  --button-text-color: #ffffff
  --button-color: #{ transparentize($accent-color, 0.1) }
  --medium-button-hover-color: #{ transparentize($accent-color, 0.9) }
  --stupid-button-border-color: #999999
  --stupid-button-text-color: #555555

  --table-hover-color: #e7e9ee
  --table-alternating-color: #f8f9fc

  --light-line-color: #e5e5e5
  --line-color: #d5d6d7
  --line-hover-color: #b9babc
  --line-highlight-color: #{ transparentize($accent-color, 0.5) }

  --dropdown-background-color: #ffffff

  --header-height: 70px

  --shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.1)
  --highlight-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.15)

  --easing: cubic-bezier(.4,0,.2,1)
</style>
