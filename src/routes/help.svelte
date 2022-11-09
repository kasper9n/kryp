<div class="page">
  <h1>Help</h1>

  <h2>Transaction Types</h2>
  <p>
    This section explains how Kryp's transaction types work. Some countries may have different rules
    than how Kryp works.
  </p>

  <h3>Trade</h3>
  <p>Exchange one asset for another. There are three types of trades:</p>
  <ul class="my-4 list-disc pl-10">
    <li>Buy transaction: Trade from your base currency. These are tax-free.</li>
    <li>
      Sell transaction: Trade into your base currency. These will realize capital gains taxes.
    </li>
    <li>
      Exchange transaction: Not to or from your base currency. These realize capital gains taxes on
      the amount you sold.
    </li>
  </ul>
  <p>
    If the transaction has a fee, that will be added to the cost of what you traded into. If the fee
    is not in your base currency, it will realize capital gains.
  </p>
  <p>
    Some countries have different tax rates depending on how long you hold your investments, and
    some have tax-free exchange transactions. Kryp currently doesn't support that.
  </p>

  <h3>Transfer</h3>
  <p>
    Move an asset from one wallet to another. Transfers are tax-free, but if you pay a fee for the
    transfer, capital gains will be realized for the fee. The fee is automatically calculated as the
    difference between the sent and received amount.
  </p>

  <h3>Deposit</h3>
  <p>
    Add an asset to a wallet. This will not realize any capital gains, however income tax may be
    applicable for some types of deposits. If a deposit is not in your base currency, it will be
    treated as a purchase of that asset with your base currency.
  </p>

  <h3>Withdrawal</h3>
  <p>
    Remove an asset from your wallet. By default this transaction will realize capital gains, unless
    the transaction has a tag that disables capital gains (like Lost).
  </p>
</div>

<style lang="sass">
  .page
    padding: 20px
    margin: auto
  // h3
  //   font-size: 1.25rem
  p
    margin: 16px 0px
</style>
