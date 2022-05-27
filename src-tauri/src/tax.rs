use crate::calc::{Balance, Calculation, Realized};
use crate::prices::{AssetKind, PriceData};
use crate::throw;
use crate::transaction::Transaction;
use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

#[cfg(test)]
use crate::transaction::{Deposit, Trade, Transfer, UncostedTransaction, Withdrawal};
#[cfg(test)]
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
  version: String,
  pub transactions: Vec<Transaction>,
  pub settings: TaxSettings,
  pub price_data: PriceData,
  realized_gains: Vec<Realized>,
  pub balances: Vec<Balance>,
  #[serde(skip)]
  pub dirty: bool,
}

impl Tax {
  pub fn new(base_currency: &str) -> Self {
    Tax {
      version: "0.1".to_string(),
      transactions: Vec::new(),
      settings: TaxSettings {
        base_currency: base_currency.to_string(),
        apis: vec![
          Api::new(ApiName::ExchangerateHost),
          Api::new(ApiName::CoinGecko),
          Api::new(ApiName::CryptoCompare),
        ],
      },
      price_data: PriceData::new(),
      realized_gains: Vec::new(),
      balances: Vec::new(),
      dirty: true,
    }
  }
  pub fn add_transaction(&mut self, tx: Transaction) {
    Tax::add_transaction_to_vec(&mut self.transactions, tx);
    self.dirty = true;
  }
  pub fn add_transaction_to_vec(transactions: &mut Vec<Transaction>, tx: Transaction) {
    let pos = transactions
      .binary_search_by(|current_tx| current_tx.date().cmp(&tx.date()))
      .unwrap_or_else(|pos| pos);
    transactions.insert(pos, tx);
  }
  pub fn calculate(&mut self) -> Result<(), String> {
    let output = Calculation::calculate(self.transactions.iter().collect())?;
    self.apply_calc_output(output);
    Ok(())
  }
  pub fn apply_calc_output(&mut self, calc: Calculation) {
    self.balances = calc.balances.to_inner();
    self.realized_gains = calc.realized_gains;
    self.dirty = true;
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
  ) -> Result<Transaction, String> {
    let mut trade = Trade::default();
    trade.date = date;
    trade.sent_amount = sent.0;
    trade.sent_asset = sent.1.to_string();
    trade.sent_wallet = sent.2.to_string();
    trade.recv_amount = recv.0;
    trade.recv_asset = recv.1.to_string();
    trade.recv_wallet = recv.2.to_string();
    let uncosted_tx = UncostedTransaction::Trade(trade);
    let base = &self.settings.base_currency;
    let tx = uncosted_tx
      .auto_cost_and_finalize(&mut self.price_data, &self.settings.apis, base)
      .await?;
    Ok(tx)
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_transfer(
    &mut self,
    date: i64,
    sent: (Decimal, &str, &str),
    recv: (Decimal, &str, &str),
  ) -> Result<Transaction, String> {
    let mut transfer = Transfer::default();
    transfer.date = date;
    transfer.sent_amount = sent.0;
    transfer.sent_asset = sent.1.to_string();
    transfer.sent_wallet = sent.2.to_string();
    transfer.recv_amount = recv.0;
    transfer.recv_asset = recv.1.to_string();
    transfer.recv_wallet = recv.2.to_string();
    let uncosted_tx = UncostedTransaction::Transfer(transfer);
    let base = &self.settings.base_currency;
    let tx = uncosted_tx
      .auto_cost_and_finalize(&mut self.price_data, &self.settings.apis, base)
      .await?;
    Ok(tx)
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_deposit(
    &mut self,
    date: i64,
    recv: (Decimal, &str, &str),
  ) -> Result<Transaction, String> {
    let mut deposit = Deposit::default();
    deposit.date = date;
    deposit.amount = recv.0;
    deposit.asset = recv.1.to_string();
    deposit.wallet = recv.2.to_string();
    let uncosted_tx = UncostedTransaction::Deposit(deposit);
    let base = &self.settings.base_currency;
    let tx = uncosted_tx
      .auto_cost_and_finalize(&mut self.price_data, &self.settings.apis, base)
      .await?;
    Ok(tx)
  }
  #[cfg(test)]
  #[tokio::main]
  pub async fn new_withdrawal(
    &mut self,
    date: i64,
    recv: (Decimal, &str, &str),
  ) -> Result<Transaction, String> {
    let mut withdrawal = Withdrawal::default();
    withdrawal.date = date;
    withdrawal.amount = recv.0;
    withdrawal.asset = recv.1.to_string();
    withdrawal.wallet = recv.2.to_string();
    let uncosted_tx = UncostedTransaction::Withdrawal(withdrawal);
    let base = &self.settings.base_currency;
    let tx = uncosted_tx
      .auto_cost_and_finalize(&mut self.price_data, &self.settings.apis, base)
      .await?;
    Ok(tx)
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaxSettings {
  pub base_currency: String,
  pub apis: Vec<Api>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Api {
  pub name: ApiName,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub key: Option<String>,
  pub disabled: bool,
}

impl Api {
  pub fn new(name: ApiName) -> Self {
    Api {
      name,
      key: None,
      disabled: false,
    }
  }
  pub fn asset_kind(&self) -> AssetKind {
    match self.name {
      ApiName::ExchangerateHost => AssetKind::Fiat,
      ApiName::CoinGecko => AssetKind::Crypto,
      ApiName::CryptoCompare => AssetKind::Crypto,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiName {
  ExchangerateHost,
  CoinGecko,
  CryptoCompare,
}

#[cfg(test)]
mod tests {
  use crate::tax::{Balance, Realized, Tax};
  use rust_decimal_macros::dec;

  #[test]
  pub fn trades() {
    let mut tax = Tax::load("./tests/kryp.json").unwrap();
    tax.transactions = vec![
      tax
        .new_deposit(1500000000000, (dec!(1000), "NOK", "Binance"))
        .unwrap(),
      tax
        .new_trade(
          1500100000000,
          (dec!(800), "NOK", "Binance"),
          (dec!(0.5), "BTC", "Binance"),
        )
        .unwrap(),
      tax
        .new_transfer(
          1500200000000,
          (dec!(0.5), "BTC", "Binance"),
          (dec!(0.5), "BTC", "Coinbase"),
        )
        .unwrap(),
      tax
        .new_trade(
          1500300000000,
          (dec!(0.5), "BTC", "Coinbase"),
          (dec!(3), "ETH", "Coinbase"),
        )
        .unwrap(),
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
      [
        Realized {
          tag: "Trade".to_string(),
          date: 1500100000000,
          input: dec!(800),
          asset: "NOK".to_string(),
          is_fee: false,
          output: dec!(800),
          wallet: "Binance".to_string()
        },
        Realized {
          tag: "Trade".to_string(),
          date: 1500300000000,
          input: dec!(800),
          asset: "BTC".to_string(),
          is_fee: false,
          output: dec!(9398.57934082),
          wallet: "Coinbase".to_string(),
        }
      ]
    );
  }

  #[test]
  pub fn transfer_fee() {
    let mut tax = Tax::load("./tests/kryp.json").unwrap();
    tax.transactions = vec![
      tax
        .new_deposit(1500000000000, (dec!(1000), "NOK", "Binance"))
        .unwrap(),
      tax
        .new_transfer(
          1500100000000,
          (dec!(1000), "NOK", "Binance"),
          (dec!(750), "NOK", "Coinbase"),
        )
        .unwrap(),
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
    assert_eq!(
      tax.realized_gains,
      [Realized {
        tag: "Transfer".to_string(),
        date: 1500100000000,
        input: dec!(250),
        asset: "NOK".to_string(),
        is_fee: true,
        output: dec!(250),
        wallet: "Binance".to_string()
      }]
    );
  }

  #[test]
  pub fn deposit_withdraw_crypto() {
    let mut tax = Tax::load("./tests/kryp.json").unwrap();
    tax.transactions = vec![
      tax
        .new_deposit(1500000000000, (dec!(2), "ETH", "Coinbase"))
        .unwrap(),
      tax
        .new_withdrawal(
          1500100000000, //
          (dec!(1), "ETH", "Coinbase"),
        )
        .unwrap(),
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
        tag: "Withdrawal".to_string(),
        date: 1500100000000,
        input: dec!(1633.83825099),
        asset: "ETH".to_string(),
        is_fee: false,
        output: dec!(1417.67606226),
        wallet: "Coinbase".to_string(),
      }]
    );
  }
}
