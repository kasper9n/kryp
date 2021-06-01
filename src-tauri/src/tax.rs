use crate::prices::{AssetKind, PriceData};
use crate::throw;
use atomicwrites::{AllowOverwrite, AtomicFile};
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

fn round_8(num: Decimal) -> Decimal {
  return num.round_dp_with_strategy(8, RoundingStrategy::MidpointAwayFromZero);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
  transactions: Vec<Transaction>,
  base_currency: String,
  price_data: PriceData,
  reazlied_gains: Vec<Realized>,
  balances: Vec<Balance>,
}

impl Tax {
  pub fn calculate(&mut self) {
    let transactions = &mut self.transactions;
    transactions.sort_by(|a, b| a.date.cmp(&b.date));
    for transaction in transactions.iter_mut() {
      match transaction.kind {
        TxType::Deposit => {
          self.balances.push(Balance {
            acquire_date: transaction.date,
            amount: transaction.to_amount,
            currency: transaction.to_asset.clone(),
            wallet: transaction.to_wallet.clone(),
            cost: transaction.cost,
          });
        }
        TxType::Trade => {
          let mut cost = deduct(
            &mut self.balances,
            &transaction.from_wallet,
            &transaction.from_asset,
            transaction.from_amount,
          );
          if transaction.fee_asset != "" {
            cost += deduct(
              &mut self.balances,
              &transaction.from_wallet,
              &transaction.fee_asset,
              transaction.fee_amount,
            );
          }
          self.reazlied_gains.push(Realized {
            date: transaction.date,
            input: cost,
            output: transaction.cost,
            wallet: transaction.from_wallet.clone(),
          });
          self.balances.push(Balance {
            acquire_date: transaction.date,
            amount: transaction.to_amount,
            currency: transaction.to_asset.clone(),
            wallet: transaction.from_wallet.clone(),
            cost: transaction.cost,
          });
        }
        TxType::Transfer => {
          if transaction.fee_asset != "" {
            panic!("Unsupported: Transfer fee");
          }
          if transaction.from_amount > transaction.to_amount {
            let fee_amount = transaction.from_amount - transaction.to_amount;
            let cost = deduct(
              &mut self.balances,
              &transaction.from_wallet,
              &transaction.from_asset,
              fee_amount,
            );
            self.reazlied_gains.push(Realized {
              date: transaction.date,
              input: cost,
              output: fee_amount,
              wallet: transaction.from_wallet.clone(),
            });
          }
          transfer(
            &mut self.balances,
            transaction.from_amount,
            &transaction.from_asset,
            &transaction.from_wallet,
            &transaction.to_wallet,
          );
        }
        TxType::Withdrawal => {
          let cost = deduct(
            &mut self.balances,
            &transaction.from_wallet,
            &transaction.from_asset,
            transaction.from_amount,
          );
          self.reazlied_gains.push(Realized {
            date: transaction.date,
            input: cost,
            output: transaction.cost,
            wallet: transaction.from_wallet.clone(),
          });
        }
      }
    }
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
          Err(err) => throw!("Error parsing file: {:?}", err),
        };
        println!("Load library: {}ms", now.elapsed().as_millis());
        return Ok(tax);
      }
      Err(e) => throw!("Error opening file: {}", e),
    }
  }
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
  pub kind: TxType,
  pub date: i64,
  pub note: String,
  pub hash: String,
  pub from_amount: Decimal,
  pub from_asset: String,
  pub from_wallet: String,
  pub to_amount: Decimal,
  pub to_asset: String,
  pub to_wallet: String,
  pub fee_amount: Decimal,
  pub fee_asset: String,
  /// Includes fee
  cost: Decimal,
}

impl Transaction {
  pub fn calculate_cost(&mut self, price_data: &mut PriceData, base: &str) -> Decimal {
    let mut cost;
    match self.kind {
      TxType::Trade => {
        let from_kind = price_data.symbol_kind(&self.from_asset);
        let to_kind = price_data.symbol_kind(&self.to_asset);
        // fiat -> fiat: fee+from
        // fiat -> cryp: fee+from
        // cryp -> cryp: fee+from
        // cryp -> fiat: fee+to
        if let (AssetKind::Crypto, AssetKind::Fiat) = (from_kind, to_kind) {
          cost = price_data.get_value(self.to_amount, &self.to_asset, self.date, base);
        } else {
          cost = price_data.get_value(self.from_amount, &self.from_asset, self.date, base);
        }
        if self.fee_asset != "" {
          cost += price_data.get_value(self.fee_amount, &self.fee_asset, self.date, base);
        }
      }
      TxType::Transfer => {
        cost = price_data.get_value(self.from_amount, &self.from_asset, self.date, base);
      }
      TxType::Deposit => {
        cost = price_data.get_value(self.to_amount, &self.to_asset, self.date, base);
      }
      TxType::Withdrawal => {
        cost = price_data.get_value(self.from_amount, &self.from_asset, self.date, base);
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

fn deduct(balances: &mut Vec<Balance>, wallet: &str, asset: &str, amount: Decimal) -> Decimal {
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
        return cost;
      }
    }
  }
  panic!("Insufficient balance to deduct amount from");
}

fn transfer(balances: &mut Vec<Balance>, amount: Decimal, asset: &str, from: &str, to: &str) {
  let mut amount_left = amount;
  let mut to_insert = None;
  for (index, balance) in balances.iter_mut().enumerate() {
    if balance.wallet == from && balance.currency == asset {
      if amount_left > balance.amount {
        balance.wallet = to.to_string();
        amount_left = amount_left - balance.amount;
      } else if amount_left == balance.amount {
        balance.wallet = to.to_string();
        return;
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
    panic!("Insufficient balance to deduct amount from");
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Realized {
  pub date: i64,
  pub input: Decimal,
  pub output: Decimal,
  pub wallet: String,
}

#[test]
pub fn trade() {
  let mut tax = Tax::load("./tests/taxes.krypj").unwrap();
  tax.transactions = vec![
    Transaction {
      kind: TxType::Deposit,
      date: 1500000000000,
      note: "".to_string(),
      hash: "".to_string(),
      from_amount: dec!(0),
      from_asset: "".to_string(),
      from_wallet: "".to_string(),
      to_amount: dec!(1000),
      to_asset: "NOK".to_string(),
      to_wallet: "Binance".to_string(),
      fee_amount: dec!(0),
      fee_asset: "".to_string(),
      cost: dec!(0),
    },
    Transaction {
      kind: TxType::Trade,
      date: 1500100000000,
      note: "".to_string(),
      hash: "".to_string(),
      from_amount: dec!(800),
      from_asset: "NOK".to_string(),
      from_wallet: "Binance".to_string(),
      to_amount: dec!(0.5),
      to_asset: "BTC".to_string(),
      to_wallet: "Binance".to_string(),
      fee_amount: dec!(0),
      fee_asset: "".to_string(),
      cost: dec!(0),
    },
  ];
  for transaction in tax.transactions.iter_mut() {
    transaction.cost = transaction.calculate_cost(&mut tax.price_data, &tax.base_currency);
  }
  tax.calculate();
  assert_eq!(
    tax.balances[0],
    Balance {
      acquire_date: 1500000000000,
      amount: dec!(200),
      currency: "NOK".to_string(),
      wallet: "Binance".to_string(),
      cost: dec!(200),
    }
  );
  assert_eq!(
    tax.balances[1],
    Balance {
      acquire_date: 1500100000000,
      amount: dec!(0.5),
      currency: "BTC".to_string(),
      wallet: "Binance".to_string(),
      cost: dec!(800),
    }
  );
  println!("tx: {:#?}", &tax.transactions);
  println!("balances: {:#?}", &tax.balances);
  println!("realized: {:#?}", &tax.reazlied_gains);
}
