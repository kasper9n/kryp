<script lang="ts">
  import { transactions, formatDate, formatTime } from './transactions'
</script>

{#each $transactions as tx}
  <div class="date">{formatDate(new Date(tx.date))}</div>
  <div class="item">
    <div class="side">
      <div class="kind">{tx.kind}</div>
      <div class="time">{formatTime(new Date(tx.date))}</div>
    </div>
    {#if tx.kind === 'Deposit'}
      <div class="sent" />
    {:else}
      <div class="sent">{tx.sent_amount} {tx.sent_asset} {tx.sent_wallet}</div>
    {/if}
    <div class="arrow">-></div>
    {#if tx.kind === 'Withdrawal'}
      <div class="recv" />
    {:else}
      <div class="recv">{tx.recv_amount} {tx.recv_asset} {tx.recv_wallet}</div>
    {/if}
    <div class="side" />
  </div>
{/each}

<style lang="sass">
  .item, .date
    user-select: auto
    -webkit-user-select: auto
  .item
    font-size: 15px
    display: flex
    background-color: #f0f0f4
    border: 1px solid #e6e6e6
    border-radius: 5px
    padding: 8px 16px
    .time
      font-size: 13px
      color: #656e76
    .side
      width: 100px
      min-width: 100px
    .sent
      width: 100%
      text-align: right
    .recv
      width: 100%
    .arrow
      width: 30px
      min-width: 30px
      text-align: center
</style>
