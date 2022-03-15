use crate::prices::{symbol_kind, AssetKind, PriceData};
use crate::round_8;
use crate::tax::Api;
use chrono::TimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use rust_decimal_macros::dec;

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
  pub manual_worth_amount: Option<Decimal>,
  pub manual_worth_asset: Option<String>,
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
      manual_worth_amount: None,
      manual_worth_asset: None,
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
  pub manual_worth_amount: Option<Decimal>,
  pub manual_worth_asset: Option<String>,
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
      manual_worth_amount: None,
      manual_worth_asset: None,
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
  pub from_amount: Option<Decimal>,
  pub from_asset: Option<String>,
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
      from_amount: None,
      from_asset: None,
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
    if let Some((amount, asset)) = self.manual_worth() {
      if asset == base {
        Ok(amount.clone())
      } else {
        let cost = price_data
          .get_value(amount.clone(), &asset, self.date(), apis, base)
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
  /// For deposits, this is the from_amount and from_asset.
  /// For withdrawals, this is the to_amount and to_asset.
  pub fn manual_worth(&self) -> Option<(Decimal, &String)> {
    match self {
      UncostedTransaction::Trade(tx) => {
        if let Some(manual_worth_amount) = tx.manual_worth_amount {
          if let Some(manual_worth_asset) = &tx.manual_worth_asset {
            return Some((manual_worth_amount, manual_worth_asset));
          }
        }
      }
      UncostedTransaction::Transfer(tx) => {
        if let Some(manual_worth_amount) = tx.manual_worth_amount {
          if let Some(manual_worth_asset) = &tx.manual_worth_asset {
            return Some((manual_worth_amount, manual_worth_asset));
          }
        }
      }
      UncostedTransaction::Deposit(tx) => {
        if let Some(from_amount) = tx.from_amount {
          if let Some(from_asset) = &tx.from_asset {
            return Some((from_amount, from_asset));
          }
        }
      }
      UncostedTransaction::Withdrawal(tx) => {
        if let Some(to_amount) = tx.to_amount {
          if let Some(to_asset) = &tx.to_asset {
            return Some((to_amount, to_asset));
          }
        }
      }
    }
    return None;
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
}
