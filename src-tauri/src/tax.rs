use crate::prices::{AssetKind, PriceData};
use crate::{round_8, throw};
use atomicwrites::{AllowOverwrite, AtomicFile};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
  pub version: String,
  pub transactions: Vec<Transaction>,
  pub base_currency: String,
  pub price_data: PriceData,
  pub realized_gains: Vec<Realized>,
  pub balances: Vec<Balance>,
  #[serde(skip)]
  pub dirty: bool,
}

impl Tax {
  pub fn new(base_currency: &str) -> Self {
    Tax {
      version: "0.1".to_string(),
      transactions: Vec::new(),
      base_currency: base_currency.to_string(),
      price_data: PriceData::new(),
      realized_gains: Vec::new(),
      balances: Vec::new(),
      dirty: true,
    }
  }
  pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
    tx.validate()?;
    self.transactions.push(tx);
    Ok(())
  }
  pub fn calculate(&mut self) -> Result<(), String> {
    let (balances, realized_gains) = calculate(&mut self.transactions)?;
    self.balances = balances;
    self.realized_gains = realized_gains;
    Ok(())
  }
  pub fn save<P: AsRef<Path>>(&self, file_path: P) {
    let now = Instant::now();
    let mut json = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"	"); // tab
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    self.serialize(&mut ser).expect("Error saving content");
    println!("Stringify: {}ms", now.elapsed().as_millis());

    let af = AtomicFile::new(&file_path, AllowOverwrite);
    af.write(|f| f.write_all(&json)).expect("Error saving");
  }
  pub fn load<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
    let now = Instant::now();
    match File::open(file_path) {
      Ok(mut file) => {
        let mut json_str = String::new();
        match file.read_to_string(&mut json_str) {
          Ok(_) => {}
          Err(err) => throw!("Error reading file: {}", err),
        };
        let tax: Self = match serde_json::from_str(&mut json_str) {
          Ok(library) => library,
          Err(err) => {
            throw!("Error parsing file: {:?}", err.to_string());
          }
        };
        println!("Load library: {}ms", now.elapsed().as_millis());
        return Ok(tax);
      }
      Err(e) => throw!("Error opening file: {}", e),
    }
  }
}

fn calculate(transactions: &mut Vec<Transaction>) -> Result<(Vec<Balance>, Vec<Realized>), String> {
  let mut balances = Vec::new();
  let mut realized_gains = Vec::new();
  for transaction in transactions {
    match transaction.kind {
      TxType::Deposit => {
        balances.push(Balance {
          acquire_date: transaction.date,
          amount: transaction.recv_amount,
          currency: transaction.recv_asset.clone(),
          wallet: transaction.recv_wallet.clone(),
          cost: transaction.cost,
        });
      }
      TxType::Trade => {
        let mut cost = deduct(
          &mut balances,
          &transaction.sent_wallet,
          &transaction.sent_asset,
          transaction.sent_amount,
        )?;
        if transaction.fee_asset != "" {
          cost += deduct(
            &mut balances,
            &transaction.sent_wallet,
            &transaction.fee_asset,
            transaction.fee_amount,
          )?;
        }
        if cost != transaction.cost {
          realized_gains.push(Realized {
            date: transaction.date,
            input: cost,
            output: transaction.cost,
            wallet: transaction.sent_wallet.clone(),
          });
        }
        balances.push(Balance {
          acquire_date: transaction.date,
          amount: transaction.recv_amount,
          currency: transaction.recv_asset.clone(),
          wallet: transaction.sent_wallet.clone(),
          cost: transaction.cost,
        });
      }
      TxType::Transfer => {
        if transaction.sent_amount > transaction.recv_amount {
          let fee_amount = transaction.sent_amount - transaction.recv_amount;
          let cost = deduct(
            &mut balances,
            &transaction.sent_wallet,
            &transaction.sent_asset,
            fee_amount,
          )?;
          if cost != fee_amount {
            realized_gains.push(Realized {
              date: transaction.date,
              input: cost,
              output: fee_amount,
              wallet: transaction.sent_wallet.clone(),
            });
          }
        }
        transfer(
          &mut balances,
          transaction.recv_amount,
          &transaction.sent_asset,
          &transaction.sent_wallet,
          &transaction.recv_wallet,
        )?;
      }
      TxType::Withdrawal => {
        let cost = deduct(
          &mut balances,
          &transaction.sent_wallet,
          &transaction.sent_asset,
          transaction.sent_amount,
        )?;
        if cost != transaction.cost {
          realized_gains.push(Realized {
            date: transaction.date,
            input: cost,
            output: transaction.cost,
            wallet: transaction.sent_wallet.clone(),
          });
        }
      }
    }
  }
  return Ok((balances, realized_gains));
}

/// Returns the NOK cost of the deducted amount
fn deduct(
  balances: &mut Vec<Balance>,
  wallet: &str,
  asset: &str,
  amount: Decimal,
) -> Result<Decimal, String> {
  let mut cost = dec!(0);
  let mut amount_left = amount;
  for balance in balances {
    if balance.wallet == wallet && balance.currency == asset {
      if amount_left > balance.amount {
        cost += balance.cost;
        amount_left = amount_left - balance.amount;
        balance.cost = dec!(0);
        balance.amount = dec!(0);
      } else {
        let deduct_percent = amount_left / balance.amount;
        let cost_to_deduct = round_8(balance.cost * deduct_percent);
        cost += cost_to_deduct;
        balance.amount = balance.amount - amount_left;
        balance.cost = balance.cost - cost_to_deduct;
        return Ok(cost);
      }
    }
  }
  throw!("Insufficient balance to deduct amount from");
}

fn transfer(
  balances: &mut Vec<Balance>,
  amount: Decimal,
  asset: &str,
  from: &str,
  to: &str,
) -> Result<(), String> {
  let mut amount_left = amount;
  let mut to_insert = None;
  for (index, balance) in balances.iter_mut().enumerate() {
    if balance.wallet == from && balance.currency == asset {
      if amount_left > balance.amount {
        balance.wallet = to.to_string();
        amount_left = amount_left - balance.amount;
      } else if amount_left == balance.amount {
        balance.wallet = to.to_string();
        return Ok(());
      } else {
        // split into two
        let move_percent = amount_left / balance.amount;
        let cost_to_move = round_8(balance.cost * move_percent);
        let new_balance = Balance {
          acquire_date: balance.acquire_date,
          amount: amount_left,
          currency: balance.currency.clone(),
          wallet: to.to_string(),
          cost: cost_to_move,
        };
        to_insert = Some((index, new_balance));
        balance.amount = balance.amount - amount_left;
        balance.cost = balance.cost - cost_to_move;
      }
    }
  }
  if let Some((index_to_insert_at, new_balance)) = to_insert {
    balances.insert(index_to_insert_at, new_balance);
  } else {
    throw!("Insufficient balance to deduct amount from");
  }
  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TxType {
  Trade = 0,
  Transfer = 1,
  Deposit = 2,
  Withdrawal = 3,
  // ExternalBuy = 4,
  // ExternalSell = 5,
  // Gift = 6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
  kind: TxType,
  date: i64,
  note: String,
  hash: String,
  sent_amount: Decimal,
  sent_asset: String,
  sent_wallet: String,
  recv_amount: Decimal,
  recv_asset: String,
  recv_wallet: String,
  fee_amount: Decimal,
  fee_asset: String,
  /// Includes fee
  cost: Decimal,
}

impl Transaction {
  pub fn from_json(json: &str) -> Result<Self, String> {
    let tx: Result<Transaction, _> = serde_json::from_str(&json);
    match tx {
      Err(e) => Err(e.to_string()),
      Ok(tx) => Ok(tx),
    }
  }
  #[allow(dead_code)]
  pub fn new(kind: TxType) -> Self {
    Transaction {
      kind,
      date: 1500000000000,
      note: "".to_string(),
      hash: "".to_string(),
      sent_amount: dec!(0),
      sent_asset: "".to_string(),
      sent_wallet: "".to_string(),
      recv_amount: dec!(1000),
      recv_asset: "".to_string(),
      recv_wallet: "".to_string(),
      fee_amount: dec!(0),
      fee_asset: "".to_string(),
      cost: dec!(0),
    }
  }
  #[allow(dead_code)]
  pub fn date(mut self, date: i64) -> Self {
    self.date = date;
    self
  }
  #[allow(dead_code)]
  pub fn requires_sent(&self) -> bool {
    match self.kind {
      TxType::Trade | TxType::Transfer | TxType::Withdrawal => true,
      TxType::Deposit => false,
    }
  }
  #[allow(dead_code)]
  pub fn sent<S: Into<String>>(mut self, amount: Decimal, asset: S, wallet: S) -> Self {
    assert!(self.requires_sent());
    self.sent_amount = amount;
    self.sent_asset = asset.into();
    self.sent_wallet = wallet.into();
    self
  }
  #[allow(dead_code)]
  pub fn requires_recv(&self) -> bool {
    match self.kind {
      TxType::Trade | TxType::Transfer | TxType::Deposit => true,
      TxType::Withdrawal => false,
    }
  }
  #[allow(dead_code)]
  pub fn recv<S: Into<String>>(mut self, amount: Decimal, asset: S, wallet: S) -> Self {
    assert!(self.requires_recv());
    self.recv_amount = amount;
    self.recv_asset = asset.into();
    self.recv_wallet = wallet.into();
    self
  }
  #[allow(dead_code)]
  pub fn allows_fee(&self) -> bool {
    match self.kind {
      TxType::Trade => true,
      TxType::Transfer | TxType::Deposit | TxType::Withdrawal => false,
    }
  }
  #[allow(dead_code)]
  pub fn fee<S: Into<String>>(mut self, amount: Decimal, asset: S) -> Self {
    assert!(self.allows_fee());
    self.fee_amount = amount;
    self.fee_asset = asset.into();
    self
  }
  pub fn cost(mut self, cost: Decimal) -> Self {
    self.cost = cost;
    self
  }
  pub fn validate(&self) -> Result<(), String> {
    fn ensure(condition: bool, failure_msg: &str) -> Result<(), String> {
      match condition {
        true => Ok(()),
        false => Err(failure_msg.to_string()),
      }
    }
    let zero = dec!(0);
    match self.kind {
      TxType::Trade => {
        ensure(self.sent_asset != "", "Required: sent_asset")?;
        ensure(self.sent_wallet != "", "Required: sent_wallet")?;
        ensure(self.recv_asset != "", "Required: recv_asset")?;
        ensure(self.recv_wallet != "", "Required: recv_wallet")?;
      }
      TxType::Transfer => {
        ensure(self.sent_asset != "", "Required: sent_asset")?;
        ensure(self.sent_wallet != "", "Required: sent_wallet")?;
        ensure(self.recv_wallet != "", "Required: recv_wallet")?;
        ensure(self.fee_amount == zero, "Must be empty: fee_amount")?;
        ensure(self.fee_asset == "", "Must be empty: fee_asset")?;
      }
      TxType::Deposit => {
        ensure(self.sent_amount == zero, "Must be empty: sent_amount")?;
        ensure(self.sent_asset == "", "Must be empty: sent_asset")?;
        ensure(self.sent_wallet == "", "Must be empty: sent_wallet")?;
        ensure(self.recv_asset != "", "Required")?;
        ensure(self.recv_wallet != "", "Required")?;
        ensure(self.fee_amount == zero, "Must be empty: fee_amount")?;
        ensure(self.fee_asset == "", "Must be empty: fee_asset")?;
      }
      TxType::Withdrawal => {
        ensure(self.sent_asset != "", "Required")?;
        ensure(self.sent_wallet != "", "Required")?;
        ensure(self.recv_amount == zero, "Must be empty: recv_amount")?;
        ensure(self.recv_asset == "", "Must be empty: recv_asset")?;
        ensure(self.recv_wallet == "", "Must be empty: recv_wallet")?;
        ensure(self.fee_amount == zero, "Must be empty: fee_amount")?;
        ensure(self.fee_asset == "", "Must be empty: fee_asset")?;
      }
    }
    Ok(())
  }
  /// Calculates and returns the cost of the transaction
  pub fn calculate_cost(&mut self, price_data: &mut PriceData, base: &str) -> Decimal {
    let mut cost;
    match self.kind {
      TxType::Trade => {
        let sent_kind = price_data.symbol_kind(&self.sent_asset);
        let recv_kind = price_data.symbol_kind(&self.recv_asset);
        // fiat -> fiat: fee+sent
        // fiat -> cryp: fee+sent
        // cryp -> cryp: fee+sent
        // cryp -> fiat: fee+recv
        if let (AssetKind::Crypto, AssetKind::Fiat) = (sent_kind, recv_kind) {
          cost = price_data.get_value(self.recv_amount, &self.recv_asset, self.date, base);
        } else {
          cost = price_data.get_value(self.sent_amount, &self.sent_asset, self.date, base);
        }
        if self.fee_asset != "" {
          cost += price_data.get_value(self.fee_amount, &self.fee_asset, self.date, base);
        }
      }
      TxType::Transfer => {
        cost = price_data.get_value(self.sent_amount, &self.sent_asset, self.date, base);
      }
      TxType::Deposit => {
        cost = price_data.get_value(self.recv_amount, &self.recv_asset, self.date, base);
      }
      TxType::Withdrawal => {
        cost = price_data.get_value(self.sent_amount, &self.sent_asset, self.date, base);
      }
    }
    return round_8(cost);
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Balance {
  pub acquire_date: i64,
  pub amount: Decimal,
  pub currency: String,
  pub wallet: String,
  pub cost: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Realized {
  pub date: i64,
  pub input: Decimal,
  pub output: Decimal,
  pub wallet: String,
}

#[test]
pub fn trades() {
  let mut tax = Tax::load("./tests/taxes.kryp").unwrap();
  tax.transactions = vec![
    Transaction::new(TxType::Deposit)
      .date(1500000000000)
      .recv(dec!(1000), "NOK", "Binance"),
    Transaction::new(TxType::Trade)
      .date(1500100000000)
      .sent(dec!(800), "NOK", "Binance")
      .recv(dec!(0.5), "BTC", "Binance"),
    Transaction::new(TxType::Transfer)
      .date(1500200000000)
      .sent(dec!(0.5), "BTC", "Binance")
      .recv(dec!(0.5), "BTC", "Coinbase"),
    Transaction::new(TxType::Trade)
      .date(1500300000000)
      .sent(dec!(0.5), "BTC", "Coinbase")
      .recv(dec!(3), "ETH", "Coinbase"),
  ];
  for transaction in tax.transactions.iter_mut() {
    transaction.cost = transaction.calculate_cost(&mut tax.price_data, &tax.base_currency);
  }
  tax.calculate().unwrap();
  assert_eq!(
    tax.balances,
    [
      Balance {
        acquire_date: 1500000000000,
        amount: dec!(200),
        currency: "NOK".to_string(),
        wallet: "Binance".to_string(),
        cost: dec!(200),
      },
      Balance {
        acquire_date: 1500100000000,
        amount: dec!(0),
        currency: "BTC".to_string(),
        wallet: "Coinbase".to_string(),
        cost: dec!(0),
      },
      Balance {
        acquire_date: 1500300000000,
        amount: dec!(3),
        currency: "ETH".to_string(),
        wallet: "Coinbase".to_string(),
        cost: dec!(9398.57934082), // = 0.5 BTC
      }
    ]
  );
  assert_eq!(
    tax.realized_gains,
    [Realized {
      date: 1500300000000,
      input: dec!(800),
      output: dec!(9398.57934082),
      wallet: "Coinbase".to_string(),
    }]
  );
}

#[test]
pub fn transfer_fee() {
  let mut tax = Tax::load("./tests/taxes.kryp").unwrap();
  tax.transactions = vec![
    Transaction::new(TxType::Deposit)
      .date(1500000000000)
      .recv(dec!(1000), "NOK", "Binance"),
    Transaction::new(TxType::Transfer)
      .date(1500100000000)
      .sent(dec!(1000), "NOK", "Binance")
      .recv(dec!(750), "NOK", "Coinbase"),
  ];
  for transaction in tax.transactions.iter_mut() {
    transaction.cost = transaction.calculate_cost(&mut tax.price_data, &tax.base_currency);
  }
  tax.calculate().unwrap();
  assert_eq!(
    tax.balances,
    [Balance {
      acquire_date: 1500000000000,
      amount: dec!(750),
      currency: "NOK".to_string(),
      wallet: "Coinbase".to_string(),
      cost: dec!(750),
    }]
  );
  assert_eq!(tax.realized_gains, []);
}

#[test]
pub fn deposit_withdraw_crypto() {
  let mut tax = Tax::load("./tests/taxes.kryp").unwrap();
  tax.transactions = vec![
    Transaction::new(TxType::Deposit)
      .date(1500000000000)
      .recv(dec!(2), "ETH", "Coinbase"),
    Transaction::new(TxType::Withdrawal)
      .date(1500100000000)
      .sent(dec!(1), "ETH", "Coinbase"),
  ];
  for transaction in tax.transactions.iter_mut() {
    transaction.cost = transaction.calculate_cost(&mut tax.price_data, &tax.base_currency);
  }
  tax.calculate().unwrap();
  println!("{:?}", tax.balances);
  println!("{:?}", tax.realized_gains);
  assert_eq!(
    tax.balances,
    [Balance {
      acquire_date: 1500000000000,
      amount: dec!(1),
      currency: "ETH".to_string(),
      wallet: "Coinbase".to_string(),
      cost: dec!(1633.83825099),
    }]
  );
  assert_eq!(
    tax.realized_gains,
    [Realized {
      date: 1500100000000,
      input: dec!(1633.83825099),
      output: dec!(1417.67606226),
      wallet: "Coinbase".to_string(),
    }]
  );
}
