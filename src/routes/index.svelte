<script lang="ts">
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri'
  import { tax } from '../lib/tax'
  import { popup, refresher } from '../lib/general'
  type Holding = {
    key: string
    amount: string
    cost: string
  }
  let holdings = []
  refresher.subscribe(() => {
    invoke('get_balances_by_asset')
      .then((v: Holding[]) => {
        console.log(v)
        holdings = v
      })
      .catch(popup)
  })
</script>

<svelte:head>
  <title>Dashboard - Kryp</title>
</svelte:head>

<div class="page">
  <div class="card">
    <h3>Current Balance</h3>
    <table>
      <tr class="header">
        <td>Asset</td>
        <td>Amount</td>
        <td>Cost</td>
        <td>Wallet</td>
      </tr>
      {#each $tax.balances as balance}
        <tr>
          <td>{balance.currency}</td>
          <td class="align-right">{balance.amount}</td>
          <td class="align-right">{balance.cost}</td>
          <td>{balance.wallet}</td>
        </tr>
      {/each}
    </table>
  </div>
</div>

<style lang="sass">
  h3
    margin: 0px
    display: block
    // color: #444444
    font-weight: 600
    margin-bottom: 10px
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
  .header
    font-weight: 600
    color: #444444
</style>
