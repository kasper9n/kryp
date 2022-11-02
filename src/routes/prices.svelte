<script lang="ts">
  import { runCmd } from '$lib/general'
  import type { PriceDataAsset } from '$lib/data'

  let assets: string[] = []
  let symbol: string | null = null
  let asset: PriceDataAsset | null = null

  runCmd('list_assets').then((result: string[]) => {
    assets = result
    if (assets[0]) {
      symbol = assets[0]
    }
  })

  $: if (symbol) {
    runCmd('get_prices', { symbol }).then((result: PriceDataAsset) => {
      asset = result
    })
  }

  function twoDigit(value: number) {
    return ('0' + value.toString()).slice(-2)
  }
  function formatDate(date: Date) {
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
  function formatTimestamp(ts: string) {
    return formatDate(new Date(Number(ts)))
  }
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
        {#each assets as asset}
          <div
            class="asset-item"
            class:current={symbol === asset}
            on:click={() => (symbol = asset)}
          >
            {asset}
          </div>
        {/each}
      </div>
      {#if asset}
        <div>
          <h2>{asset.symbol}</h2>
          <p>
            Type: {asset.kind}
            <br />
            Data interval: {asset.interval}
          </p>
          <table>
            <thead>
              <tr>
                <td><b>Time</b></td>
                <td><b>Price (USD)</b></td>
              </tr>
            </thead>
            <tbody>
              {#each Object.entries(asset.prices) as [timestamp, price]}
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
    margin: auto
  .card
    font-size: 14px
    padding: 10px
    border: 1px solid var(--input-border)
    border-radius: 3px
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
