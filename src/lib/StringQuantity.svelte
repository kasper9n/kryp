<script lang="ts">
  export let quantity: string | null = null
  export let amount: string
  export let asset: string

  $: updateValues(quantity)
  $: updateQuantity(amount, asset)

  function setQuantity(newQuantity: string | null) {
    if (newQuantity !== quantity) {
      quantity = newQuantity
    }
  }
  function updateQuantity(amount: string, asset: string) {
    if (amount === '' && asset === '') {
      setQuantity(null)
    } else {
      setQuantity(amount + ' ' + asset)
    }
  }

  function setValues(newAmount: string, newAsset: string) {
    if (newAmount !== amount) amount = newAmount
    if (newAsset !== asset) asset = newAsset
  }

  function updateValues(quantity: string | null) {
    if (quantity === null) {
      setValues('', '')
    } else {
      quantity = quantity.trim()
      const spacePos = quantity.indexOf(' ')
      if (spacePos === -1) {
        setValues(quantity, '')
      } else {
        setValues(quantity.slice(0, spacePos), quantity.slice(spacePos + 1))
      }
    }
  }
</script>
