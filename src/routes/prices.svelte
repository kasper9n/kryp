<script lang="ts">
  import { runCmd } from '../lib/general'
  import type { PriceData, PriceDataAsset } from '../lib/data'
  let assets: PriceDataAsset[] = []
  runCmd('get_prices').then((price_data: PriceData) => {
    assets = Object.values(price_data.assets)
    if (assets[0]) currentI = 0
  })
  function twoDigit(value: number) {
    return ('0' + value.toString()).slice(-2)
  }
  function formatDate(date: Date) {
    return (
      date.getFullYear() +
      '-' +
      twoDigit(date.getMonth()) +
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
  function formatTimestamp(ts: string) {
    return formatDate(new Date(Number(ts)))
  }
  let currentI: number | null = null
  $: current = currentI === null ? null : assets[currentI]
</script>

<div class="page">
  <p>
    This page contains the prices that have been fetched and saved.
    <br />
    Crypto prices are fetched from
    <a target="_blank" href="https://www.coingecko.com/en">CoinGecko</a>
    <br />
    Fiat prices are fetched from
    <a target="_blank" href="https://exchangerate.host">exchangerate.host</a>
  </p>
  <div class="card flex">
    {#if assets}
      <div>
        {#each assets as asset, i}
          <div class="asset-item" class:current={currentI === i} on:click={() => (currentI = i)}
            >{asset.symbol}</div>
        {/each}
      </div>
      {#if current}
        <div>
          <h2>{current.symbol}</h2>
          <p>
            Type: {current.kind}
            <br />
            Data interval: {current.interval}
          </p>
          <table>
            <thead>
              <tr>
                <td><b>Time</b></td>
                <td><b>Price (USD)</b></td>
              </tr>
            </thead>
            <tbody>
              {#each Object.entries(current.prices) as [timestamp, price]}
                <tr>
                  <td>{formatTimestamp(timestamp)}</td>
                  <td>{price}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style lang="sass">
  .page
    padding: 20px
    max-width: 1000px
    margin: auto
  .card
    font-size: 14px
    padding: 10px
    border: 1px solid #e7e8e8
    border-radius: 3px
    background-color: #ffffff
  .flex
    display: flex
  .asset-item
    padding: 5px 10px
    border-radius: 3px
    margin-right: 20px
    cursor: pointer
    transition: 0.1s var(--ease)
    transition-property: transform, opacity
    &:active
      opacity: 0.95
      transform: scale(0.95)
    &.current
      background-color: #191B20
      color: #ffffff
  table
    border-collapse: collapse
  td
    border: 1px solid #cccccc
    padding: 3px 8px
</style>
