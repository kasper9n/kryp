<script lang="ts">
  import { transactions, formatDate, formatTime } from '$lib/transactions'
</script>

<div class="list">
  {#each $transactions as tx}
    <div class="item">
      <div class="icon">
        {#if tx.type === 'Trade'}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="trade"
            height="24px"
            viewBox="0 0 24 24"
            width="24px"
            fill="#000000"
            ><path d="M0 0h24v24H0z" fill="none" /><path
              d="M6.99 11L3 15l3.99 4v-3H14v-2H6.99v-3zM21 9l-3.99-4v3H10v2h7.01v3L21 9z"
            /></svg
          >
        {:else if tx.type === 'Transfer'}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="transfer"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            ><path
              d="M10.024 4h6.015l7.961 8-7.961 8h-6.015l7.961-8-7.961-8zm-10.024 16h6.015l7.961-8-7.961-8h-6.015l7.961 8-7.961 8z"
            /></svg
          >
        {:else if tx.type === 'Withdrawal'}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="withdraw"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            ><path
              d="M13.025 1l-2.847 2.828 6.176 6.176h-16.354v3.992h16.354l-6.176 6.176 2.847 2.828 10.975-11z"
            /></svg
          >
        {:else if tx.type === 'Deposit'}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="deposit"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            ><path
              d="M13.025 1l-2.847 2.828 6.176 6.176h-16.354v3.992h16.354l-6.176 6.176 2.847 2.828 10.975-11z"
            /></svg
          >
        {/if}
      </div>
      <div class="kind">{tx.tag}</div>
      {#if tx.type === 'Deposit'}
        <div class="sent" />
      {:else if tx.type === 'Withdrawal'}
        <div class="sent">{tx.amount} {tx.asset} {tx.wallet}</div>
      {:else}
        <div class="sent">{tx.sent_amount} {tx.sent_asset} {tx.sent_wallet}</div>
      {/if}
      <!-- <div class="arrow">-></div> -->
      <div class="arrow">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
          ><path
            d="M13.025 1l-2.847 2.828 6.176 6.176h-16.354v3.992h16.354l-6.176 6.176 2.847 2.828 10.975-11z"
          /></svg
        >
      </div>
      {#if tx.type === 'Withdrawal'}
        <div class="recv" />
      {:else if tx.type === 'Deposit'}
        <div class="recv">{tx.amount} {tx.asset} {tx.wallet}</div>
      {:else}
        <div class="recv">{tx.recv_amount} {tx.recv_asset} {tx.recv_wallet}</div>
      {/if}
      <div class="right">
        <span class="date">{formatDate(new Date(tx.date))}</span>
        <span class="time">{formatTime(new Date(tx.date))}</span>
      </div>
    </div>
  {/each}
</div>

<style lang="sass">
  $kind-icon-width: 26px
  $kind-width: 50px
  .list
    border: 1px solid #e7e8e8
    background-color: #ffffff
    border-bottom: 0px
  .item
    font-size: 15px
    display: flex
    align-items: center
    border-bottom: 1px solid #e7e8e8
    padding: 0px 16px
  .icon
    width: $kind-icon-width
    display: block
    text-align: center
  .arrow, .icon
    font-size: 0px // fix bottom 4px
  svg.deposit
    width: 18px
    height: 18px
    transform: rotate(45deg)
    fill: #35d085
  svg.trade
    width: 26px
    height: 26px
    fill: #2ea8fa
  svg.withdraw
    width: 16px
    height: 16px
    transform: rotate(-45deg)
    fill: #f92f72
  svg.transfer
    width: 16px
    height: 16px
    fill: #b853ee
  .kind
    width: $kind-width + $kind-icon-width
    padding-left: 8px
    box-sizing: border-box
  .arrow svg
    width: 16px
    height: 16px
  .right
    width: 100px
    min-width: 100px
    text-align: right
    padding: 10px 0px
  .sent
    width: 0px
    flex-grow: 1
    text-align: right
  .recv
    width: 0px
    flex-grow: 1
  .arrow
    width: 50px
    min-width: 50px
    text-align: center
    svg
      fill: #4c4f57
  .time
    font-size: 13px
    color: #656e76
</style>
