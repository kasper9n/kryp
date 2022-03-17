use std::str::{Chars, FromStr};

use crate::prices::{symbol_kind, AssetKind, PriceData};
use crate::tax::Api;
use crate::{round_8, throw};
use chrono::{Local, TimeZone};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use rust_decimal_macros::dec;

pub fn format_date(ts: i64) -> String {
  let dt = Local.timestamp_millis(ts);
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Trade {
  pub tag: String,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub sent_amount: Decimal,
  pub sent_asset: String,
  pub sent_wallet: String,
  pub recv_amount: Decimal,
  pub recv_asset: String,
  pub recv_wallet: String,
  pub fee_amount: Decimal,
  pub fee_asset: String,
  pub manual_worth: Option<String>,
  /// Includes fee
  pub cost: Decimal,
}
impl Trade {
  pub fn cost(&self) -> Decimal {
    self.cost
  }
  #[cfg(test)]
  pub fn default() -> Self {
    Trade {
      tag: "Trade".to_string(),
      date: 0,
      note: "".to_string(),
      hash: "".to_string(),
      recv_amount: dec!(0),
      recv_asset: "".to_string(),
      recv_wallet: "".to_string(),
      sent_amount: dec!(0),
      sent_asset: "".to_string(),
      sent_wallet: "".to_string(),
      fee_amount: dec!(0),
      fee_asset: "".to_string(),
      manual_worth: None,
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
  pub tag: String,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub sent_amount: Decimal,
  pub sent_asset: String,
  pub sent_wallet: String,
  pub recv_amount: Decimal,
  pub recv_asset: String,
  pub recv_wallet: String,
  pub manual_worth: Option<String>,
  /// Includes fee
  pub cost: Decimal,
}
impl Transfer {
  #[cfg(test)]
  pub fn default() -> Self {
    Transfer {
      tag: "Transfer".to_string(),
      date: 0,
      note: "".to_string(),
      hash: "".to_string(),
      recv_amount: dec!(0),
      recv_asset: "".to_string(),
      recv_wallet: "".to_string(),
      sent_amount: dec!(0),
      sent_asset: "".to_string(),
      sent_wallet: "".to_string(),
      manual_worth: None,
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Deposit {
  pub tag: String,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  pub manual_worth: Option<String>,
  /// Includes fee
  pub cost: Decimal,
}
impl Deposit {
  #[cfg(test)]
  pub fn default() -> Self {
    Deposit {
      tag: "Deposit".to_string(),
      date: 0,
      note: "".to_string(),
      hash: "".to_string(),
      amount: dec!(0),
      asset: "".to_string(),
      wallet: "".to_string(),
      manual_worth: None,
      cost: dec!(0),
    }
  }
  pub fn cost(&self) -> Decimal {
    self.cost
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Withdrawal {
  pub tag: String,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  pub manual_worth: Option<String>,
  /// Includes fee
  pub cost: Decimal,
}
impl Withdrawal {
  pub fn cost(&self) -> Decimal {
    self.cost
  }
  #[cfg(test)]
  pub fn default() -> Self {
    Withdrawal {
      tag: "Withdrawal".to_string(),
      date: 0,
      note: "".to_string(),
      hash: "".to_string(),
      amount: dec!(0),
      asset: "".to_string(),
      wallet: "".to_string(),
      manual_worth: None,
      cost: dec!(0),
    }
  }
}

/// A transaction without a final cost set
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UncostedTransaction {
  Trade(Trade),
  Transfer(Transfer),
  Deposit(Deposit),
  Withdrawal(Withdrawal),
}
impl UncostedTransaction {
  pub fn date(&self) -> i64 {
    match self {
      UncostedTransaction::Trade(tx) => tx.date,
      UncostedTransaction::Transfer(tx) => tx.date,
      UncostedTransaction::Deposit(tx) => tx.date,
      UncostedTransaction::Withdrawal(tx) => tx.date,
    }
  }
  pub fn from_json(json: &str) -> Result<Self, String> {
    let tx_result: Result<Self, _> = serde_json::from_str(&json);
    let tx = match tx_result {
      Err(e) => return Err(e.to_string()),
      Ok(tx) => tx,
    };
    Ok(tx)
  }
  /// Figure out the cost. If a manual cost is set, that will be used.
  pub async fn auto_cost_and_finalize(
    self,
    price_data: &mut PriceData,
    apis: &[Api],
    base: &str,
  ) -> Result<Transaction, String> {
    let cost = self.get_or_calculate_cost(price_data, apis, base).await?;
    Ok(self.finalize(cost))
  }
  /// Get the manual cost, or calculate it if it's not set
  pub async fn get_or_calculate_cost(
    &self,
    price_data: &mut PriceData,
    apis: &[Api],
    base: &str,
  ) -> Result<Decimal, String> {
    if let Some(mw) = self.manual_worth_qty()? {
      if mw.asset == base {
        Ok(mw.amount.clone())
      } else {
        let cost = price_data
          .get_value(mw.amount.clone(), &mw.asset, self.date(), apis, base)
          .await?;
        Ok(cost)
      }
    } else {
      Ok(self.calculate_cost(price_data, apis, base).await?)
    }
  }
  pub fn finalize(mut self, cost: Decimal) -> Transaction {
    match &mut self {
      UncostedTransaction::Trade(tx) => tx.cost = cost,
      UncostedTransaction::Transfer(tx) => tx.cost = cost,
      UncostedTransaction::Deposit(tx) => tx.cost = cost,
      UncostedTransaction::Withdrawal(tx) => tx.cost = cost,
    }
    let final_tx = match self {
      UncostedTransaction::Trade(tx) => Transaction::Trade(tx),
      UncostedTransaction::Transfer(tx) => Transaction::Transfer(tx),
      UncostedTransaction::Deposit(tx) => Transaction::Deposit(tx),
      UncostedTransaction::Withdrawal(tx) => Transaction::Withdrawal(tx),
    };
    final_tx
  }
  /// Gets the manual worth of a transaction.
  pub fn manual_worth(&self) -> &Option<String> {
    match self {
      UncostedTransaction::Trade(tx) => &tx.manual_worth,
      UncostedTransaction::Transfer(tx) => &tx.manual_worth,
      UncostedTransaction::Deposit(tx) => &tx.manual_worth,
      UncostedTransaction::Withdrawal(tx) => &tx.manual_worth,
    }
  }
  /// Gets the manual worth of a transaction as a Quantity
  pub fn manual_worth_qty(&self) -> Result<Option<Quantity>, String> {
    match self.manual_worth() {
      Some(manual_worth) => Quantity::parse(&manual_worth),
      None => Ok(None),
    }
  }
  /// Calculates and returns the cost of the transaction, regardless of whether a manual worth is set
  async fn calculate_cost(
    &self,
    price_data: &mut PriceData,
    apis: &[Api],
    base: &str,
  ) -> Result<Decimal, String> {
    let mut cost;
    match self {
      UncostedTransaction::Trade(tx) => {
        let sent_kind = symbol_kind(&tx.sent_asset);
        let recv_kind = symbol_kind(&tx.recv_asset);
        // cryp -> fiat: fee+recv
        // cryp -> cryp: fee+sent (or fallback to fee+recv)
        // fiat -> fiat: fee+sent
        // fiat -> cryp: fee+sent
        if sent_kind == Some(AssetKind::Crypto) && recv_kind == Some(AssetKind::Fiat) {
          cost = price_data
            .get_value(tx.recv_amount, &tx.recv_asset, tx.date, apis, base)
            .await?;
        } else if sent_kind == None && recv_kind == Some(AssetKind::Crypto) {
          cost = price_data
            .get_value(tx.recv_amount, &tx.recv_asset, tx.date, apis, base)
            .await?;
        } else {
          cost = price_data
            .get_value(tx.sent_amount, &tx.sent_asset, tx.date, apis, base)
            .await?;
        }
        if tx.fee_asset != "" {
          cost += price_data
            .get_value(tx.fee_amount, &tx.fee_asset, tx.date, apis, base)
            .await?;
        }
      }
      UncostedTransaction::Transfer(tx) => {
        cost = price_data
          .get_value(tx.sent_amount, &tx.sent_asset, tx.date, apis, base)
          .await?;
      }
      UncostedTransaction::Deposit(tx) => {
        cost = price_data
          .get_value(tx.amount, &tx.asset, tx.date, apis, base)
          .await?;
      }
      UncostedTransaction::Withdrawal(tx) => {
        cost = price_data
          .get_value(tx.amount, &tx.asset, tx.date, apis, base)
          .await?;
      }
    }
    Ok(round_8(cost))
  }
}

/// A transaction with a final cost set. This should not be directly created
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Transaction {
  Trade(Trade),
  Transfer(Transfer),
  Deposit(Deposit),
  Withdrawal(Withdrawal),
}

impl Transaction {
  pub fn tag(&self) -> &str {
    match self {
      Transaction::Trade(tx) => &tx.tag,
      Transaction::Transfer(tx) => &tx.tag,
      Transaction::Deposit(tx) => &tx.tag,
      Transaction::Withdrawal(tx) => &tx.tag,
    }
  }
  pub fn date(&self) -> i64 {
    match self {
      Transaction::Trade(tx) => tx.date,
      Transaction::Transfer(tx) => tx.date,
      Transaction::Deposit(tx) => tx.date,
      Transaction::Withdrawal(tx) => tx.date,
    }
  }
  pub fn manual_worth<'a>(&'a self) -> &'a Option<String> {
    match self {
      Transaction::Trade(tx) => &tx.manual_worth,
      Transaction::Transfer(tx) => &tx.manual_worth,
      Transaction::Deposit(tx) => &tx.manual_worth,
      Transaction::Withdrawal(tx) => &tx.manual_worth,
    }
  }
  pub fn to_csv_record<'a>(&'a self) -> Vec<String> {
    let empty = "".to_string();
    let manual_worth = self.manual_worth().as_ref().unwrap_or(&empty);
    match self {
      Transaction::Trade(trade) => {
        vec![
          trade.tag.clone(),
          trade.sent_amount.to_string(),
          trade.sent_asset.clone(),
          trade.sent_wallet.clone(),
          trade.recv_amount.to_string(),
          trade.recv_asset.clone(),
          trade.recv_wallet.clone(),
          trade.fee_amount.to_string(),
          trade.fee_asset.clone(),
          trade.note.clone(),
          trade.hash.clone(),
          chrono::Utc.timestamp_millis(trade.date).to_string(),
          manual_worth.to_string(),
          // cost: Decimal,
        ]
      }
      Transaction::Transfer(transfer) => {
        vec![
          transfer.tag.clone(),
          transfer.sent_amount.to_string(),
          transfer.sent_asset.clone(),
          transfer.sent_wallet.clone(),
          transfer.recv_amount.to_string(),
          transfer.recv_asset.clone(),
          transfer.recv_wallet.clone(),
          "".to_string(),
          "".to_string(),
          transfer.note.clone(),
          transfer.hash.clone(),
          chrono::Utc.timestamp_millis(transfer.date).to_string(),
          manual_worth.to_string(),
          // cost: Decimal,
        ]
      }
      Transaction::Deposit(deposit) => {
        vec![
          deposit.tag.clone(),
          "".to_string(),
          deposit.amount.to_string(),
          deposit.asset.clone(),
          deposit.wallet.clone(),
          "".to_string(),
          "".to_string(),
          deposit.note.clone(),
          deposit.hash.clone(),
          chrono::Utc.timestamp_millis(deposit.date).to_string(),
          manual_worth.to_string(),
          // cost: Decimal,
        ]
      }
      Transaction::Withdrawal(withdrawal) => {
        vec![
          withdrawal.tag.clone(),
          withdrawal.amount.to_string(),
          withdrawal.asset.clone(),
          withdrawal.wallet.clone(),
          "".to_string(),
          "".to_string(),
          "".to_string(),
          withdrawal.note.clone(),
          withdrawal.hash.clone(),
          chrono::Utc.timestamp_millis(withdrawal.date).to_string(),
          manual_worth.to_string(),
          // cost: Decimal,
        ]
      }
    }
  }
}

pub struct CoreTransaction {
  pub tag: String,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub sent_amount: Option<Decimal>,
  pub sent_asset: Option<String>,
  pub sent_wallet: Option<String>,
  pub recv_amount: Option<Decimal>,
  pub recv_asset: Option<String>,
  pub recv_wallet: Option<String>,
  pub fee_amount: Option<Decimal>,
  pub fee_asset: Option<String>,
  // pub manual_worth_amount: Option<Decimal>,
  // pub manual_worth_asset: Option<String>,
  // pub cost: Decimal,
}
impl CoreTransaction {
  pub fn from_transaction(tx: Transaction) -> Self {
    match tx {
      Transaction::Trade(tx) => CoreTransaction {
        tag: tx.tag,
        date: tx.date,
        note: tx.note,
        hash: tx.hash,
        sent_amount: Some(tx.sent_amount),
        sent_asset: Some(tx.sent_asset),
        sent_wallet: Some(tx.sent_wallet),
        recv_amount: Some(tx.recv_amount),
        recv_asset: Some(tx.recv_asset),
        recv_wallet: Some(tx.recv_wallet),
        fee_amount: Some(tx.fee_amount),
        fee_asset: Some(tx.fee_asset),
      },
      Transaction::Transfer(tx) => CoreTransaction {
        tag: tx.tag,
        date: tx.date,
        note: tx.note,
        hash: tx.hash,
        sent_amount: Some(tx.sent_amount),
        sent_asset: Some(tx.sent_asset),
        sent_wallet: Some(tx.sent_wallet),
        recv_amount: Some(tx.recv_amount),
        recv_asset: Some(tx.recv_asset),
        recv_wallet: Some(tx.recv_wallet),
        fee_amount: None,
        fee_asset: None,
      },
      Transaction::Deposit(tx) => CoreTransaction {
        tag: tx.tag,
        date: tx.date,
        note: tx.note,
        hash: tx.hash,
        sent_amount: None,
        sent_asset: None,
        sent_wallet: None,
        recv_amount: Some(tx.amount),
        recv_asset: Some(tx.asset),
        recv_wallet: Some(tx.wallet),
        fee_amount: None,
        fee_asset: None,
      },
      Transaction::Withdrawal(tx) => CoreTransaction {
        tag: tx.tag,
        date: tx.date,
        note: tx.note,
        hash: tx.hash,
        sent_amount: Some(tx.amount),
        sent_asset: Some(tx.asset),
        sent_wallet: Some(tx.wallet),
        recv_amount: None,
        recv_asset: None,
        recv_wallet: None,
        fee_amount: None,
        fee_asset: None,
      },
    }
  }
}

fn take_decimal_from_chars(chars: &mut Chars) -> Option<Decimal> {
  let mut num_str = "".to_string();
  let mut found_period = false;
  loop {
    let c = chars.next()?;
    if c.is_ascii_digit() {
      num_str.push(c);
    } else if c == '.' && !found_period {
      num_str.push(c);
      found_period = true;
    } else {
      break;
    }
  }
  let amount = Decimal::from_str(&num_str).ok()?;
  Some(amount)
}

pub struct Quantity {
  pub amount: Decimal,
  pub asset: String,
}
impl Quantity {
  pub fn new(amount: String, asset: String) -> Result<Option<Self>, String> {
    if amount == "" && asset == "" {
      Ok(None)
    } else if amount == "" || asset == "" {
      throw!("Invalid amount/asset: {} {}", amount, asset);
    } else {
      let num = match Decimal::from_str(&amount) {
        Ok(d) => d,
        Err(_) => throw!("Invalid number \"{}\"", amount),
      };
      Ok(Some(Self {
        amount: num,
        asset: asset,
      }))
    }
  }
  pub fn parse(value: &str) -> Result<Option<Self>, String> {
    if value.trim() == "" {
      return Ok(None);
    }
    let mut chars = value.chars();
    let amount = match take_decimal_from_chars(&mut chars) {
      Some(amount) => amount,
      None => throw!("Invalid manual worth \"{}\"", value),
    };

    let asset_str: String = chars.collect();
    let asset = asset_str.trim().to_string();

    Ok(Some(Self { amount, asset }))
  }
  pub fn to_string(&self) -> String {
    self.amount.to_string() + " " + &self.asset
  }
}
pub struct Value {
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
}
impl Value {
  pub fn new(amount: String, asset: String, wallet: String) -> Result<Option<Value>, String> {
    let quantity = Quantity::new(amount, asset)?;
    if let Some(quantity) = quantity {
      if wallet == "" {
        throw!("The amount {} {} has no wallet", quantity.amount, quantity.asset);
      } else {
        Ok(Some(Self {
          amount: quantity.amount,
          asset: quantity.asset,
          wallet: wallet,
        }))
      }
    } else if wallet == "" {
      Ok(None)
    } else {
      throw!("Wallet \"{}\" specified without any amount/asset", wallet);
    }
  }
}
