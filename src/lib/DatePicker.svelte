<script lang="ts">
  import moment from 'moment'
  function twoDigit(value: number) {
    return ('0' + value.toString()).slice(-2)
  }
  export let width = '114px'
  export let value = new Date()
  let textHist = ['', '']
  let text = ''
  function externalUpdate(x: Date) {
    text = getText(x)
  }
  $: externalUpdate(value)

  function getText(date: Date) {
    return (
      date.getFullYear() +
      '-' +
      twoDigit(date.getMonth() + 1) +
      '-' +
      twoDigit(date.getDate()) +
      ' ' +
      twoDigit(date.getHours()) +
      ':' +
      twoDigit(date.getMinutes()) +
      ':' +
      twoDigit(date.getSeconds())
    )
  }

  export let invalid = true
  function textUpdate(text: string) {
    let mom = moment(text, 'YYYY-MM-DD H:mm:ss', true)
    invalid = !mom.isValid()
    textHist = [textHist[1], text]
    if (!invalid) value = mom.toDate()
  }
  $: textUpdate(text)
  function input(e: any) {
    let oldText = textHist[0]
    if (e.inputType === 'insertText' && /^[0-9]$/.test(e.data) && text === oldText + e.data) {
      if (moment(oldText, 'YYYY', true).isValid()) text = oldText + '-' + e.data
      if (moment(oldText, 'YYYY-MM', true).isValid()) text = oldText + '-' + e.data
      if (moment(oldText, 'YYYY-MM-DD', true).isValid()) text = oldText + ' ' + e.data
      if (moment(oldText, 'YYYY-MM-DD HH', true).isValid()) text = oldText + ':' + e.data
      if (moment(oldText, 'YYYY-MM-DD H:mm', true).isValid()) text = oldText + ':' + e.data
    }
  }
</script>

<input
  class:invalid
  type="text"
  bind:value={text}
  on:input={input}
  placeholder="2020-12-31 23:00:00"
  style={`width: ${width}`} />

<style lang="sass">
  input
    min-width: 0px
    box-sizing: border-box
    padding: 4px 6px
    margin: 0px
    font-family: inherit
    font-size: 12px
    border: 1px solid #c6cddd
    border-radius: 3px
    width: 114px
  .invalid
    border: 1px solid rgba(#f92f72, 0.5)
    background-color: #fff0f5
</style>
