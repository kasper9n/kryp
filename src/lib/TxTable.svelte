<script lang="ts">
  import { transactions, formatDateTime } from '../lib/transactions'
  let hideEditBox = true
  let editText = ''
  let editCell
  function tbodyClick(e: MouseEvent) {
    const tbody = e.target as HTMLTableSectionElement
    if (tbody.nodeName === 'TD') {
      const rect = tbody.getBoundingClientRect()
      rect.width += 1 // compensate for table border-collapse
      rect.height += 1 // compensate for table border-collapse
      editCell.style.left = rect.x + 'px'
      editCell.style.top = rect.y + 'px'
      editCell.style.width = rect.width + 'px'
      editCell.style.height = rect.height + 'px'
      hideEditBox = false
      editText = tbody.innerText
      editCell.focus()
    }
  }
  function keydown(e) {
    if (e.key === 'Escape') editBoxBlur()
  }
  function editBoxBlur() {
    hideEditBox = true
  }
</script>

<input
  class="edit-cell"
  class:hide={hideEditBox}
  bind:this={editCell}
  on:keydown={keydown}
  on:blur={editBoxBlur}
  bind:value={editText} />

<table>
  <thead>
    <tr>
      <th>Type</th>
      <th>Sent</th>
      <th>Cur.</th>
      <th>Wallet</th>
      <th>Received</th>
      <th>Cur.</th>
      <th>Wallet</th>
      <th>Fee</th>
      <th>Cur.</th>
      <th>Note</th>
      <th>Hash</th>
      <th>Date</th>
    </tr>
  </thead>
  <tbody on:click={tbodyClick}>
    {#each $transactions as tx, i}
      <tr class:odd={i % 2 === 0}>
        <td
          class="type"
          class:green={tx.kind === 'Deposit' || tx.kind === 'Income'}
          class:red={tx.kind === 'Withdrawal'}>{tx.kind}</td>
        <td class="sent amount">{tx.sent_amount}</td>
        <td class="sent asset">{tx.sent_asset}</td>
        <td class="sent wallet">{tx.sent_wallet}</td>
        <td class="recv amount">{tx.recv_amount}</td>
        <td class="recv asset">{tx.recv_asset}</td>
        <td class="recv wallet">{tx.recv_wallet}</td>
        <td class="fee amount">{tx.fee_amount}</td>
        <td class="fee asset">{tx.fee_asset}</td>
        <td class="note">{tx.note}</td>
        <td class="hash">{tx.hash}</td>
        <td class="date">{formatDateTime(new Date(tx.date))}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style lang="sass">
  table
    margin: auto
    border-spacing: 0px
    border-collapse: collapse
    cursor: default
    font-size: 13px
  thead
    font-weight: 600
  td
    padding: 6px 10px
    border: 1px solid #e7e8e8
    &.amount
      text-align: right
  thead
    th
      text-align: center
      padding-bottom: 4px
  tbody
    background-color: #f6f6f9
  .odd
    background-color: #ffffff
  .edit-cell
    font-size: 13px
    position: absolute
    background: #ffffff
    box-sizing: border-box
    padding: 6px 10px
    padding-right: 0px
    border: 1px solid transparent
    outline: 2px solid #1073fe
    outline-offset: -2px
    &.hide
      opacity: 0
      pointer-events: none
  .red
    color: #f92f72
  .green
    color: #25b670
</style>
