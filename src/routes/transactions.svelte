<script lang="ts">
  enum TxType {
    Trade = 0,
    Transfer = 1,
    Deposit = 2,
    Withdrawal = 3,
  }
  function txTypeInfo(txType: TxType) {
    const map: Record<TxType, { text: string }> = {
      [TxType.Trade]: { text: 'Trade' },
      [TxType.Transfer]: { text: 'Transfer' },
      [TxType.Deposit]: { text: 'Deposit' },
      [TxType.Withdrawal]: { text: 'Withdrawal' },
    }
    return map[txType]
  }

  type Transaction = {
    type: TxType
    date: number
    note: string
    hash: string
    fromAmount: string
    fromAsset: string
    fromWallet: string
    toAmount: string
    toAsset: string
    toWallet: string
    feeAmount: string
    feeAsset: string
  }
  let transactions: Transaction[] = [
    {
      type: TxType.Deposit,
      date: 1500973488000,
      note: 'via Anycoin Direct',
      hash: '',
      fromAmount: '932.03',
      fromAsset: 'NOK',
      fromWallet: '',
      toAmount: '0.04073887',
      toAsset: 'BTC',
      toWallet: 'Bittrex',
      feeAmount: '',
      feeAsset: '',
    },
    {
      type: TxType.Trade,
      date: 1500973489000,
      note: '',
      hash: '',
      fromAmount: '0.02006253',
      fromAsset: 'BTC',
      fromWallet: 'Bittrex',
      toAmount: '6120.03355798',
      toAsset: 'SC',
      toWallet: 'Bittrex',
      feeAmount: '0.00005003',
      feeAsset: 'BTC',
    },
    {
      type: TxType.Trade,
      date: 1501184993000,
      note: '',
      hash: '',
      fromAmount: '0.02054666',
      fromAsset: 'BTC',
      fromWallet: 'Bittrex',
      toAmount: '304.87286253',
      toAsset: 'XRP',
      toWallet: 'Bittrex',
      feeAmount: '0.00005123',
      feeAsset: 'BTC',
    },
    {
      type: TxType.Deposit,
      date: 1501185655000,
      note: 'via Anycoin Direct',
      hash: '',
      fromAmount: '932',
      fromAsset: 'NOK',
      fromWallet: '',
      toAmount: '0.04240144',
      toAsset: 'BTC',
      toWallet: 'Bittrex',
      feeAmount: '',
      feeAsset: '',
    },
  ]

  function calculate() {
    const baseCurrency = 'NOK'
    const assets = {}
    for (const tx of transactions) {
      console.log(tx)
      if (tx.fromAsset) {
        if (!assets[tx.fromAsset]) assets[tx.fromAsset] = 0
        assets[tx.fromAsset] -= Number(tx.fromAmount)
      }
      if (tx.toAsset) {
        if (!assets[tx.toAsset]) assets[tx.toAsset] = 0
        assets[tx.toAsset] += Number(tx.toAmount)
      }
      if (tx.feeAsset) {
        if (!assets[tx.feeAsset]) assets[tx.feeAsset] = 0
        assets[tx.feeAsset] -= Number(tx.feeAmount)
      }
    }
    return { assets: assets }
  }
  const info = calculate()
  console.log(info)

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

<svelte:head>
  <title>Transactions - Kryp</title>
</svelte:head>
<div class="page">
  <input
    class="edit-cell"
    class:hide={hideEditBox}
    bind:this={editCell}
    contenteditable="true"
    on:keydown={keydown}
    on:blur={editBoxBlur}
    bind:value={editText} />

  <table>
    <thead>
      <tr>
        <th>Type</th>
        <th>From</th>
        <th>Cur.</th>
        <th>Wallet</th>
        <th>To</th>
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
      {#each transactions as tx, i}
        <tr class:odd={i % 2 === 0}>
          <td class="type" class:green={tx.type === 2} class:red={tx.type === 3}
            >{txTypeInfo(tx.type).text}</td>
          <td class="from amount">{tx.fromAmount}</td>
          <td class="from asset">{tx.fromAsset}</td>
          <td class="from wallet">{tx.fromWallet}</td>
          <td class="to amount">{tx.toAmount}</td>
          <td class="to asset">{tx.toAsset}</td>
          <td class="to wallet">{tx.toWallet}</td>
          <td class="fee amount">{tx.feeAmount}</td>
          <td class="fee asset">{tx.feeAsset}</td>
          <td class="note">{tx.note}</td>
          <td class="hash">{tx.hash}</td>
          <td class="date">{formatDate(new Date(tx.date))}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style lang="sass">
  .page
    font-size: 12px
  table
    margin: auto
    border-spacing: 0px
    border-collapse: collapse
    cursor: default
  thead
    font-weight: 600
  td
    padding: 6px 10px
    border: 1px solid #e7e8e8
  thead
    th
      text-align: center
      padding-bottom: 4px
  .odd
    background-color: #f6f6f9
  .edit-cell
    font-size: inherit
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
    color: #ff6176
  .green
    color: #34ad84
</style>
