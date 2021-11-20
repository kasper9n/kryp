use crate::prices::PriceData;
use crate::transaction::Transaction;
use crate::{round_8, throw};
use atomicwrites::{AllowOverwrite, AtomicFile};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

#[cfg(test)]
use crate::transaction::{Deposit, Trade, Transfer, Withdrawal};

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
    let pos = self
      .transactions
      .binary_search_by(|current_tx| current_tx.date().cmp(&tx.date()))
      .unwrap_or_else(|pos| pos);
    self.transactions.insert(pos, tx);
    self.dirty = true;
    Ok(())
  }
  pub fn calculate(&mut self) -> Result<(), String> {
    let output = calculate(&mut self.transactions)?;
    self.balances = output.balances;
    self.realized_gains = output.realized_gains;
    self.dirty = true;
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
        let mut tax: Self = match serde_json::from_str(&mut json_str) {
          Ok(library) => library,
          Err(err) => {
            throw!("Error parsing file: {:?}", err.to_string());
          }
        };
        println!("Load library: {}ms", now.elapsed().as_millis());
        tax.calculate()?;
        return Ok(tax);
      }
      Err(e) => throw!("Error opening file: {}", e),
    }
  }

  #[cfg(test)]
  #[tokio::main]
  pub async fn new_trade(
    &mut self,
    date: i64,
    sent: (Decimal, &str, &str),
    recv: (Decimal, &str, &str),
  ) -> Transaction {
    let mut trade = Trade::default();
    trade.date = date;
    trade.sent_amount = sent.0;
    trade.sent_asset = sent.1.to_string();
    trade.sent_wallet = sent.2.to_string();
    trade.recv_amount = recv.0;
    trade.recv_asset = recv.1.to_string();
    trade.recv_wallet = recv.2.to_string();
    let mut tx = Transaction::Trade(trade);
    tx.refresh_cost(&mut self.price_data, &self.base_currency)
      .await;
    tx
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_transfer(
    &mut self,
    date: i64,
    sent: (Decimal, &str, &str),
    recv: (Decimal, &str, &str),
  ) -> Transaction {
    let mut transfer = Transfer::default();
    transfer.date = date;
    transfer.sent_amount = sent.0;
    transfer.sent_asset = sent.1.to_string();
    transfer.sent_wallet = sent.2.to_string();
    transfer.recv_amount = recv.0;
    transfer.recv_asset = recv.1.to_string();
    transfer.recv_wallet = recv.2.to_string();
    let mut tx = Transaction::Transfer(transfer);
    tx.refresh_cost(&mut self.price_data, &self.base_currency)
      .await;
    tx
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_deposit(&mut self, date: i64, recv: (Decimal, &str, &str)) -> Transaction {
    let mut deposit = Deposit::default();
    deposit.date = date;
    deposit.amount = recv.0;
    deposit.asset = recv.1.to_string();
    deposit.wallet = recv.2.to_string();
    let mut tx = Transaction::Deposit(deposit);
    tx.refresh_cost(&mut self.price_data, &self.base_currency)
      .await;
    tx
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_withdrawal(&mut self, date: i64, recv: (Decimal, &str, &str)) -> Transaction {
    let mut withdrawal = Withdrawal::default();
    withdrawal.date = date;
    withdrawal.amount = recv.0;
    withdrawal.asset = recv.1.to_string();
    withdrawal.wallet = recv.2.to_string();
    let mut tx = Transaction::Withdrawal(withdrawal);
    tx.refresh_cost(&mut self.price_data, &self.base_currency)
      .await;
    tx
  }
}

struct CalculationOutput {
  pub balances: Vec<Balance>,
  pub realized_gains: Vec<Realized>,
}

fn calculate(transactions: &mut Vec<Transaction>) -> Result<CalculationOutput, String> {
  let mut balances = Vec::new();
  let mut realized_gains = Vec::new();
  for transaction in transactions {
    match transaction {
      Transaction::Trade(trade) => {
        // deduct the sent amount from balance
        let mut balance_cost = deduct(
          &mut balances,
          &trade.sent_wallet,
          &trade.sent_asset,
          trade.sent_amount,
        )?;
        if trade.fee_asset != "" {
          balance_cost += deduct(
            &mut balances,
            &trade.sent_wallet,
            &trade.fee_asset,
            trade.fee_amount,
          )?;
        }
        // add realized gains/losses if the cost of the balance is different
        // from the worth of the transaction
        if balance_cost != trade.cost() {
          realized_gains.push(Realized {
            date: trade.date,
            input: balance_cost,
            output: trade.cost(),
            wallet: trade.sent_wallet.clone(),
          });
        }
        balances.push(Balance {
          acquire_date: trade.date,
          amount: trade.recv_amount,
          currency: trade.recv_asset.clone(),
          wallet: trade.sent_wallet.clone(),
          cost: trade.cost(),
        });
      }
      Transaction::Transfer(transaction) => {
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
      Transaction::Deposit(deposit) => {
        balances.push(Balance {
          acquire_date: deposit.date,
          amount: deposit.amount,
          currency: deposit.asset.clone(),
          wallet: deposit.wallet.clone(),
          cost: deposit.cost(),
        });
      }
      Transaction::Withdrawal(withdrawal) => {
        let cost = deduct(
          &mut balances,
          &withdrawal.wallet,
          &withdrawal.asset,
          withdrawal.amount,
        )?;
        if cost != withdrawal.cost() {
          realized_gains.push(Realized {
            date: withdrawal.date,
            input: cost,
            output: withdrawal.cost(),
            wallet: withdrawal.wallet.clone(),
          });
        }
      }
    }
  }
  return Ok(CalculationOutput {
    balances,
    realized_gains,
  });
}

/// Deduct from a balance. Returns the cost of the deducted amount.
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
    tax.new_deposit(1500000000000, (dec!(1000), "NOK", "Binance")),
    tax.new_trade(
      1500100000000,
      (dec!(800), "NOK", "Binance"),
      (dec!(0.5), "BTC", "Binance"),
    ),
    tax.new_transfer(
      1500200000000,
      (dec!(0.5), "BTC", "Binance"),
      (dec!(0.5), "BTC", "Coinbase"),
    ),
    tax.new_trade(
      1500300000000,
      (dec!(0.5), "BTC", "Coinbase"),
      (dec!(3), "ETH", "Coinbase"),
    ),
  ];
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
    tax.new_deposit(1500000000000, (dec!(1000), "NOK", "Binance")),
    tax.new_transfer(
      1500100000000,
      (dec!(1000), "NOK", "Binance"),
      (dec!(750), "NOK", "Coinbase"),
    ),
  ];
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
    tax.new_deposit(1500000000000, (dec!(2), "ETH", "Coinbase")),
    tax.new_withdrawal(
      1500100000000, //
      (dec!(1), "ETH", "Coinbase"),
    ),
  ];
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
