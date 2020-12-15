<script>
  export let value = new Date().getTime()
  import { dateFormat, formatDate, parseDate } from '../stores.js'
  let textValue
  $: {
    try {
      textValue = formatDate(new Date(value), dateFormat)
    } catch(e) {
      null
    }
  }
  function change(e) {
    try {
      value = parseDate(e.target.value, dateFormat).getTime()
    } catch {
      value = null
    }
  }
</script>

<input type="text" value={textValue} on:input={change}>
