use crate::prices::{AssetKind, PriceData};
use crate::round_8;
use rust_decimal::Decimal;
#[cfg(test)]
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
  cat: String,
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
  /// Includes fee
  pub cost: Decimal,
}
impl Trade {
  #[cfg(test)]
  pub fn new(date: i64, sent: (Decimal, &str, &str), recv: (Decimal, &str, &str)) -> Self {
    Trade {
      cat: "Transfer".to_string(),
      date,
      note: "".to_string(),
      hash: "".to_string(),
      recv_amount: recv.0,
      recv_asset: recv.1.to_string(),
      recv_wallet: recv.2.to_string(),
      sent_amount: sent.0,
      sent_asset: sent.1.to_string(),
      sent_wallet: sent.2.to_string(),
      fee_amount: dec!(0),
      fee_asset: "".to_string(),
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
  cat: String,
  pub date: i64,
  note: String,
  hash: String,
  pub sent_amount: Decimal,
  pub sent_asset: String,
  pub sent_wallet: String,
  pub recv_amount: Decimal,
  pub recv_asset: String,
  pub recv_wallet: String,
  /// Includes fee
  pub cost: Decimal,
}
impl Transfer {
  #[cfg(test)]
  pub fn new(date: i64, sent: (Decimal, &str, &str), recv: (Decimal, &str, &str)) -> Self {
    Transfer {
      cat: "Transfer".to_string(),
      date,
      note: "".to_string(),
      hash: "".to_string(),
      recv_amount: recv.0,
      recv_asset: recv.1.to_string(),
      recv_wallet: recv.2.to_string(),
      sent_amount: sent.0,
      sent_asset: sent.1.to_string(),
      sent_wallet: sent.2.to_string(),
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deposit {
  cat: String,
  pub date: i64,
  note: String,
  hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  /// Includes fee
  pub cost: Decimal,
}
impl Deposit {
  #[cfg(test)]
  pub fn new(date: i64, recv: (Decimal, &str, &str)) -> Self {
    Deposit {
      cat: "Deposit".to_string(),
      date,
      note: "".to_string(),
      hash: "".to_string(),
      amount: recv.0,
      asset: recv.1.to_string(),
      wallet: recv.2.to_string(),
      cost: dec!(0),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Withdrawal {
  cat: String,
  pub date: i64,
  note: String,
  hash: String,
  pub amount: Decimal,
  pub asset: String,
  pub wallet: String,
  /// Includes fee
  pub cost: Decimal,
}
impl Withdrawal {
  #[cfg(test)]
  pub fn new(date: i64, sent: (Decimal, &str, &str)) -> Self {
    Withdrawal {
      cat: "Withdrawal".to_string(),
      date,
      note: "".to_string(),
      hash: "".to_string(),
      amount: sent.0,
      asset: sent.1.to_string(),
      wallet: sent.2.to_string(),
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
#[serde(tag = "base")]
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
  pub fn set_cost(&mut self, v: Decimal) {
    match self {
      Transaction::Trade(tx) => tx.cost = v,
      Transaction::Transfer(tx) => tx.cost = v,
      Transaction::Deposit(tx) => tx.cost = v,
      Transaction::Withdrawal(tx) => tx.cost = v,
    }
  }
  pub fn from_json(tx_type: TxType, json: &str) -> Result<Self, String> {
    match tx_type {
      TxType::Trade => {
        let tx: Result<Trade, _> = serde_json::from_str(&json);
        match tx {
          Err(e) => Err(e.to_string()),
          Ok(tx) => Ok(Transaction::Trade(tx)),
        }
      }
      TxType::Transfer => {
        let tx: Result<Transfer, _> = serde_json::from_str(&json);
        match tx {
          Err(e) => Err(e.to_string()),
          Ok(tx) => Ok(Transaction::Transfer(tx)),
        }
      }
      TxType::Deposit => {
        let tx: Result<Deposit, _> = serde_json::from_str(&json);
        match tx {
          Err(e) => Err(e.to_string()),
          Ok(tx) => Ok(Transaction::Deposit(tx)),
        }
      }
      TxType::Withdrawal => {
        let tx: Result<Withdrawal, _> = serde_json::from_str(&json);
        match tx {
          Err(e) => Err(e.to_string()),
          Ok(tx) => Ok(Transaction::Withdrawal(tx)),
        }
      }
    }
  }
  /// Calculates and returns the cost of the transaction
  pub fn calculate_cost(&mut self, price_data: &mut PriceData, base: &str) -> Decimal {
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
          cost = price_data.get_value(tx.recv_amount, &tx.recv_asset, tx.date, base);
        } else {
          cost = price_data.get_value(tx.sent_amount, &tx.sent_asset, tx.date, base);
        }
        if tx.fee_asset != "" {
          cost += price_data.get_value(tx.fee_amount, &tx.fee_asset, tx.date, base);
        }
      }
      Transaction::Transfer(tx) => {
        cost = price_data.get_value(tx.sent_amount, &tx.sent_asset, tx.date, base);
      }
      Transaction::Deposit(tx) => {
        cost = price_data.get_value(tx.amount, &tx.asset, tx.date, base);
      }
      Transaction::Withdrawal(tx) => {
        cost = price_data.get_value(tx.amount, &tx.asset, tx.date, base);
      }
    }
    return round_8(cost);
  }
}
