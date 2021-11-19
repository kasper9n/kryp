<script lang="ts">
  import { runCmd } from '../lib/general'
  type Holding = {
    key: string
    amount: string
    cost: string
  }

  const holdings: Promise<Holding[]> = runCmd('get_holdings')
</script>

<div class="page">
  <div class="card">
    <h3>Balance by Currency</h3>
    {#await holdings then holdings}
      <table>
        <tr class="header">
          <td>Asset</td>
          <td>Amount</td>
          <td>Cost</td>
        </tr>
        {#each holdings as holding}
          <tr>
            <td>{holding.key}</td>
            <td class="align-right">{holding.amount}</td>
            <td class="align-right">{holding.cost}</td>
          </tr>
        {/each}
      </table>
    {/await}
  </div>
</div>

<style lang="sass">
  h3
    margin: 0px
    display: block
    margin-bottom: 10px
  .page
    padding: 20px
    margin: auto
  .card
    font-size: 14px
    padding: 10px
    border: 1px solid #e7e8e8
    border-radius: 3px
    background-color: #ffffff
    box-shadow: 0px 0px 10px 0px hsla(0, 0%, 50%, 0.1)
  .header
    font-weight: 600
    color: #444444
</style>
