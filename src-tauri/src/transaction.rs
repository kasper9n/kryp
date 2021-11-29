use crate::prices::{AssetKind, PriceData};
use crate::round_8;
use chrono::TimeZone;
use rust_decimal::Decimal;
#[cfg(test)]
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Trade {
  tag: String,
  pub date: i64,
  note: String,
  hash: String,
  pub sent_amount: Decimal,
  pub sent_asset: String,
  pub sent_wallet: String,
  pub recv_amount: Decimal,
  pub recv_asset: String,
  pub recv_wallet: String,
  pub fee_amount: Decimal,
  pub fee_asset: String,
  manual_worth_amount: Option<Decimal>,
  manual_worth_asset: Option<String>,
  /// Includes fee
  cost: Decimal,
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
      manual_worth_amount: None,
      manual_worth_asset: None,
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Transfer {
  tag: String,
  pub date: i64,
  note: String,
  hash: String,
  pub sent_amount: Decimal,
  pub sent_asset: String,
  pub sent_wallet: String,
  pub recv_amount: Decimal,
  pub recv_asset: String,
  pub recv_wallet: String,
  manual_worth_amount: Option<Decimal>,
  manual_worth_asset: Option<String>,
  /// Includes fee
  cost: Decimal,
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
      manual_worth_amount: None,
      manual_worth_asset: None,
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Deposit {
  tag: String,
  pub date: i64,
  note: String,
  hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  from_amount: Option<Decimal>,
  from_asset: Option<String>,
  /// Includes fee
  cost: Decimal,
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
      from_amount: None,
      from_asset: None,
      cost: dec!(0),
    }
  }
  pub fn cost(&self) -> Decimal {
    self.cost
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Withdrawal {
  tag: String,
  pub date: i64,
  note: String,
  hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  pub to_amount: Option<Decimal>,
  pub to_asset: Option<String>,
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
      to_amount: None,
      to_asset: None,
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TxType {
  Trade = 0,
  Transfer = 1,
  Deposit = 2,
  Withdrawal = 4,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Transaction {
  Trade(Trade),
  Transfer(Transfer),
  Deposit(Deposit),
  Withdrawal(Withdrawal),
}

impl Transaction {
  pub fn date(&self) -> i64 {
    match self {
      Transaction::Trade(tx) => tx.date,
      Transaction::Transfer(tx) => tx.date,
      Transaction::Deposit(tx) => tx.date,
      Transaction::Withdrawal(tx) => tx.date,
    }
  }
  pub async fn from_json(
    json: &str,
    price_data: &mut PriceData,
    base: &str,
  ) -> Result<Self, String> {
    let tx_result: Result<Self, _> = serde_json::from_str(&json);
    let mut tx = match tx_result {
      Err(e) => return Err(e.to_string()),
      Ok(tx) => tx,
    };
    tx.refresh_cost(price_data, base).await;
    Ok(tx)
  }
  pub fn to_csv_record<'a>(&'a self) -> Vec<String> {
    match self {
      Transaction::Trade(trade) => {
        // let date = chrono::Utc.timestamp_millis(trade.date);
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
          // manual_worth_amount: Option<Decimal>,
          // manual_worth_asset: Option<String>,
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
          // manual_worth_amount: Option<Decimal>,
          // manual_worth_asset: Option<String>,
          // cost: Decimal,
        ]
      }
      Transaction::Deposit(deposit) => {
        vec![
          deposit.tag.clone(),
          match deposit.from_amount {
            Some(amount) => amount.to_string(),
            None => "".to_string(),
          },
          deposit.from_asset.clone().unwrap_or("".to_string()),
          "".to_string(),
          deposit.amount.to_string(),
          deposit.asset.clone(),
          deposit.wallet.clone(),
          "".to_string(),
          "".to_string(),
          deposit.note.clone(),
          deposit.hash.clone(),
          chrono::Utc.timestamp_millis(deposit.date).to_string(),
          // manual_worth_amount: Option<Decimal>,
          // manual_worth_asset: Option<String>,
          // cost: Decimal,
        ]
      }
      Transaction::Withdrawal(withdrawal) => {
        vec![
          withdrawal.tag.clone(),
          withdrawal.amount.to_string(),
          withdrawal.asset.clone(),
          withdrawal.wallet.clone(),
          match withdrawal.to_amount {
            Some(amount) => amount.to_string(),
            None => "".to_string(),
          },
          withdrawal.to_asset.clone().unwrap_or("".to_string()),
          "".to_string(),
          "".to_string(),
          "".to_string(),
          withdrawal.note.clone(),
          withdrawal.hash.clone(),
          chrono::Utc.timestamp_millis(withdrawal.date).to_string(),
          // manual_worth_amount: Option<Decimal>,
          // manual_worth_asset: Option<String>,
          // cost: Decimal,
        ]
      }
    }
  }
  /// Gets the manual worth of a transaction.
  /// For deposits, this is the from_amount and from_asset.
  /// For withdrawals, this is the to_amount and to_asset.
  pub fn manual_worth(&self) -> Option<(Decimal, &String)> {
    match self {
      Transaction::Trade(tx) => {
        if let Some(manual_worth_amount) = tx.manual_worth_amount {
          if let Some(manual_worth_asset) = &tx.manual_worth_asset {
            return Some((manual_worth_amount, manual_worth_asset));
          }
        }
      }
      Transaction::Transfer(tx) => {
        if let Some(manual_worth_amount) = tx.manual_worth_amount {
          if let Some(manual_worth_asset) = &tx.manual_worth_asset {
            return Some((manual_worth_amount, manual_worth_asset));
          }
        }
      }
      Transaction::Deposit(tx) => {
        if let Some(from_amount) = tx.from_amount {
          if let Some(from_asset) = &tx.from_asset {
            return Some((from_amount, from_asset));
          }
        }
      }
      Transaction::Withdrawal(tx) => {
        if let Some(to_amount) = tx.to_amount {
          if let Some(to_asset) = &tx.to_asset {
            return Some((to_amount, to_asset));
          }
        }
      }
    }
    return None;
  }
  /// Set the cost. If a manual cost is set, that will be used.
  pub async fn refresh_cost(&mut self, price_data: &mut PriceData, base: &str) {
    let cost = self.determine_cost(price_data, base).await;
    match self {
      Transaction::Trade(tx) => tx.cost = cost,
      Transaction::Transfer(tx) => tx.cost = cost,
      Transaction::Deposit(tx) => tx.cost = cost,
      Transaction::Withdrawal(tx) => tx.cost = cost,
    }
  }
  /// Get the cost. If a manual cost is set, that will be used.
  async fn determine_cost(&mut self, price_data: &mut PriceData, base: &str) -> Decimal {
    if let Some((amount, asset)) = self.manual_worth() {
      if asset == base {
        amount.clone()
      } else {
        price_data
          .get_value(amount.clone(), &asset, self.date(), base)
          .await
      }
    } else {
      self.calculate_cost(price_data, base).await
    }
  }
  /// Calculates and returns the cost of the transaction
  async fn calculate_cost(&mut self, price_data: &mut PriceData, base: &str) -> Decimal {
    let mut cost;
    match self {
      Transaction::Trade(tx) => {
        let sent_kind = price_data.symbol_kind(&tx.sent_asset);
        let recv_kind = price_data.symbol_kind(&tx.recv_asset);
        // fiat -> fiat: fee+sent
        // fiat -> cryp: fee+sent
        // cryp -> cryp: fee+sent
        // cryp -> fiat: fee+recv
        if let (AssetKind::Crypto, AssetKind::Fiat) = (sent_kind, recv_kind) {
          cost = price_data
            .get_value(tx.recv_amount, &tx.recv_asset, tx.date, base)
            .await;
        } else {
          cost = price_data
            .get_value(tx.sent_amount, &tx.sent_asset, tx.date, base)
            .await;
        }
        if tx.fee_asset != "" {
          cost += price_data
            .get_value(tx.fee_amount, &tx.fee_asset, tx.date, base)
            .await;
        }
      }
      Transaction::Transfer(tx) => {
        cost = price_data
          .get_value(tx.sent_amount, &tx.sent_asset, tx.date, base)
          .await;
      }
      Transaction::Deposit(tx) => {
        cost = price_data
          .get_value(tx.amount, &tx.asset, tx.date, base)
          .await;
      }
      Transaction::Withdrawal(tx) => {
        cost = price_data
          .get_value(tx.amount, &tx.asset, tx.date, base)
          .await;
      }
    }
    return round_8(cost);
  }
}
