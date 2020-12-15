<script>
  export let value = new Date().getTime()
  import { format, parse } from 'date-fns'
  let textValue
  const dateFormat = 'yyyy/MM/dd h:mm a'
  const referenceDate = new Date(1577833200000)
  $: {
    try {
      textValue = format(new Date(value), dateFormat)
    } catch(e) {
      null
    }
  }
  function change(e) {
    try {
      value = parse(e.target.value, dateFormat, referenceDate).getTime()
    } catch {
      value = null
    }
  }
</script>

<input type="text" value={textValue} on:input={change}>
