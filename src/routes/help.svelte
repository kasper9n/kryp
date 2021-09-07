<div class="page">
  <h1>Help</h1>

  <h2>Transaction Types</h2>
  <p>
    This section explains how Kryp's transaction types work. Some countries may have different rules
    than how Kryp works.</p>

  <h3>Trade</h3>
  <p>Exchange one asset for another. There are three types of trades:</p>
  <ul>
    <li>Buy transaction: Trade from your base currency. These are tax-free.</li>
    <li>
      Sell transaction: Trade into your base currency. These will realize capital gains taxes.</li>
    <li>
      Exchange transaction: Not to or from your base currency. These realize capital gains taxes on
      the amount you sold.</li>
  </ul>
  <p>
    If the transaction has a fee, that will be added to the cost of what you traded into. If the fee
    is not in your base currency, it will realize capital gains.</p>
  <p>
    Some countries have different tax rates depending on how long you hold your investments, and
    some have tax-free exchange transactions. Kryp currently doesn't support that.</p>

  <h3>Transfer</h3>
  <p>
    Move an asset from one wallet to another. Transfers are tax-free, but if you pay a fee for the
    transfer, this fee will realize capital gains. The fee is automatically calculated as the
    difference between the sent and received amount.</p>

  <h3>Deposit</h3>
  <p>
    Add an asset to a wallet. By default this transaction does not result in any taxes. If certain
    tags are added, it will generate income tax (like Airdrop, Fork, Mining, Staking, Referrals,
    Loan interest, etc). If the deposit is not in your base currency, it will be treated as a
    purchase, using the price at the time of the deposit.</p>

  <h3>Withdrawal</h3>
  <p>
    Remove an asset from your wallet. By default this transaction will realize capital gains, unless
    the transaction has a tag that disables capital gains (like Lost, Gift, Donation).</p>
</div>

<style lang="sass">
  .page
    padding: 20px
    max-width: 1000px
    margin: auto
</style>
