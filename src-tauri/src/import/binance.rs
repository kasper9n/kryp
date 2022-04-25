use super::csv::{lowercase_header_contains, read_csv};
use crate::transaction::{BaseTransaction, Quantity, UncostedTransaction, Value};
use crate::{err, throw};
use chrono::{TimeZone, Utc};
use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub async fn read(path: PathBuf) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut csv = read_csv(path)?;
  if lowercase_header_contains(&mut csv, "price") {
    return parse_trade_history(csv).await;
  } else {
    return parse_all_statements(csv).await;
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllStatementsRow {
  #[serde(rename = "User_ID")]
  user_id: String,
  #[serde(rename = "UTC_Time")]
  utc_time: String,
  account: String,
  operation: String,
  coin: String,
  change: String,
  remark: String,
}

enum TradeSide {
  Buy,
  Sell,
  Fee,
}
struct AllStatementsTradeRow {
  user_id: String,
  timestamp: i64,
  account: String,
  side: TradeSide,
  quantity: Quantity,
  remark: String,
}

#[derive(Hash, Eq, PartialEq)]
struct AllStatementsTradeGroupKey {
  user_id: String,
  timestamp: i64,
  account: String,
}
#[derive(Default)]
struct AllStatementsTradeGroup {
  buys: Vec<AllStatementsTradeRow>,
  sells: Vec<AllStatementsTradeRow>,
  fees: Vec<AllStatementsTradeRow>,
}

enum ParsedAllStatementsRow {
  NonTrade(UncostedTransaction),
  Trade(AllStatementsTradeRow),
}

async fn parse_all_statements(
  mut csv: Reader<File>,
) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut uncosted_transactions = Vec::new();

  // let mut incomplete_trades: TradeCorrelationMap = HashMap::new();
  let mut incomplete_trades = Vec::new();

  for (i, row) in csv.deserialize().enumerate() {
    let parsed_row = match parse_all_statements_row(row?).await {
      Ok(Some(tx)) => tx,
      Ok(None) => continue,
      Err(e) => return err!("Error in row {}: {}", i + 2, e),
    };
    match parsed_row {
      ParsedAllStatementsRow::NonTrade(non_trade) => {
        uncosted_transactions.push(non_trade);
      }
      ParsedAllStatementsRow::Trade(trade) => {
        incomplete_trades.push(trade);
      }
    }
  }

  let mut uncosted_trades = correlate_trade_rows(incomplete_trades)?;
  uncosted_transactions.append(&mut uncosted_trades);

  // loop through hashmap

  Ok(uncosted_transactions)
}

fn join_non_empty(values: &[&String], sep: &str) -> String {
  let mut joined = String::new();
  for value in values {
    if value != &"" {
      if value != &"" {
        joined += sep;
      }
      joined += value;
    }
  }
  joined
}

fn correlate_trade_rows(
  rows: Vec<AllStatementsTradeRow>,
) -> Result<Vec<UncostedTransaction>, String> {
  type TradeGroupMap = HashMap<AllStatementsTradeGroupKey, AllStatementsTradeGroup>;

  // group trades with same user_id/utc_time/account
  let mut grouped_trades: TradeGroupMap = HashMap::new();
  for row in rows {
    let key = AllStatementsTradeGroupKey {
      user_id: row.user_id.clone(),
      timestamp: row.timestamp,
      account: row.account.clone(),
    };
    let trade_group = grouped_trades.entry(key).or_insert(Default::default());
    match row.side {
      TradeSide::Buy => trade_group.buys.push(row),
      TradeSide::Sell => trade_group.sells.push(row),
      TradeSide::Fee => trade_group.fees.push(row),
    }
  }

  // combine rows into transactions
  let mut uncosted_transactions: Vec<UncostedTransaction> = Vec::new();
  for (key, trade_group) in grouped_trades {
    if trade_group.buys.len() != trade_group.sells.len() {
      throw!(
        "File has a mismatch of {} Buy rows and {} \"Transaction Related\" rows at {}",
        trade_group.buys.len(),
        trade_group.sells.len(),
        Utc
          .timestamp_millis(key.timestamp)
          .format("%Y-%m-%d %H:%M:%S UTC")
      );
    }
    if trade_group.fees.len() > trade_group.buys.len() {
      throw!("More fee than buy rows");
    }
    let mut group_base_transactions: Vec<BaseTransaction> = Vec::new();

    let buys = trade_group.buys.into_iter();
    let sells = trade_group.sells.into_iter();
    let mut fees = trade_group.fees.into_iter();
    for (buy, sell) in buys.zip(sells) {
      // match buy/sell/fee rows
      let fee = fees.next();
      let fee_remark = fee.as_ref().map(|f| f.remark.clone()).unwrap_or_default();
      let fee_asset = fee.as_ref().map(|f| f.quantity.asset.to_string());
      let base_transaction = BaseTransaction {
        tag: "Trade".to_string(),
        date: key.timestamp,
        note: join_non_empty(&[&buy.remark, &sell.remark, &fee_remark], ", "),
        hash: "".to_string(),
        sent: Some(sell.quantity.with_wallet("Binance")),
        recv: Some(buy.quantity.with_wallet("Binance")),
        fee: fee.map(|f| f.quantity),
        manual_worth: None,
      };

      let mut was_combined = false;
      for group_base_transaction in &mut group_base_transactions {
        let sent = base_transaction.sent.as_ref().unwrap();
        let recv = base_transaction.recv.as_ref().unwrap();
        let fee = base_transaction.fee.as_ref();

        let group_sent = group_base_transaction.sent.as_mut().unwrap();
        let group_recv = group_base_transaction.recv.as_mut().unwrap();
        let group_fee = &mut group_base_transaction.fee;

        let same_sent = &*group_sent.asset == sent.asset;
        let same_recv = &*group_recv.asset == recv.asset;
        let same_fee = match (&group_fee, &fee_asset) {
          (Some(group_fee), Some(fee_asset)) => &group_fee.asset == fee_asset,
          (None, None) => true,
          _ => false,
        };
        if same_sent && same_recv && same_fee {
          // combine transactions of identical assets
          group_sent.amount += sent.amount;
          group_recv.amount += recv.amount;
          if let Some(group_fee) = group_fee {
            group_fee.amount += fee.unwrap().amount;
          }
          was_combined = true;
          break;
        }
      }
      if !was_combined {
        group_base_transactions.push(base_transaction);
      }
    }

    for base_transaction in group_base_transactions {
      let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
      uncosted_transactions.push(uncosted_transaction);
    }
  }
  Ok(uncosted_transactions)
}

async fn parse_all_statements_row(
  row: AllStatementsRow,
) -> Result<Option<ParsedAllStatementsRow>, String> {
  let timestamp = match Utc.datetime_from_str(&row.utc_time, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => date.timestamp_millis(),
    Err(e) => throw!("Invalid date: {}", e),
  };
  let change = row.change;
  let coin = row.coin;

  match (row.account.as_str(), row.operation.as_str()) {
    ("Spot", "Buy" | "Transaction Related" | "Fee") => {
      let quantity = Quantity::new(change, coin)?;
      let mut trade = AllStatementsTradeRow {
        user_id: row.user_id,
        timestamp,
        account: row.account,
        side: match row.operation.as_str() {
          "Buy" => TradeSide::Buy,
          "Transaction Related" => match quantity.amount.is_sign_positive() {
            true => TradeSide::Buy,
            false => TradeSide::Sell,
          },
          "Fee" => TradeSide::Fee,
          _ => panic!("Error Buy/TxR/Fee"),
        },
        quantity,
        remark: row.remark,
      };
      match trade.side {
        TradeSide::Buy => {}
        TradeSide::Sell => trade.quantity.amount.set_sign_positive(true),
        TradeSide::Fee => trade.quantity.amount.set_sign_positive(true),
      }
      return Ok(Some(ParsedAllStatementsRow::Trade(trade)));
    }
    _ => {}
  }

  let mut base_transaction = BaseTransaction {
    tag: "".into(),
    date: timestamp,
    note: row.remark,
    hash: "".into(),
    sent: None,
    recv: None,
    fee: None,
    manual_worth: None,
  };
  match (row.account.as_str(), row.operation.as_str()) {
    ("Spot", "Deposit") => {
      base_transaction.tag = "Deposit".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", "Distribution") => {
      base_transaction.tag = "Gift".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", "Savings Interest" | "POS savings interest" | "Launchpool Interest") => {
      base_transaction.tag = "Interest".into();
      base_transaction.recv = Some(Value::new(change, coin, "Binance")?);
    }
    // skip savings balance "lock-ins"
    ("Spot", "Savings purchase" | "POS savings purchase") => {
      return Ok(None);
    }
    // skip savings balance "lock-in releases"
    ("Spot", "Savings Principal redemption" | "POS savings redemption") => {
      return Ok(None);
    }
    ("Spot", "Small assets exchange BNB") => {
      let value = Value::new(change, coin, "Binance")?;
      if value.amount.is_sign_positive() {
        base_transaction.tag = "Deposit".into();
        base_transaction.recv = Some(value);
      } else {
        base_transaction.tag = "Withdrawal".into();
        base_transaction.sent = Some(value);
      }
      base_transaction.note = "Small assets exchange BNB".into();
    }
    ("Spot", "Withdraw") => {
      base_transaction.tag = "Withdrawal".into();
      base_transaction.sent = Some(Value::new(change, coin, "Binance")?);
    }
    ("Spot", _) => throw!("Unsupported operation: {}", row.operation),
    (_, _) => throw!("Unsupported Account: {}", row.account),
  };
  if let Some(sent) = &mut base_transaction.sent {
    sent.amount.set_sign_positive(true);
  }
  if let Some(fee) = &mut base_transaction.fee {
    fee.amount.set_sign_positive(true);
  }

  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
  Ok(Some(ParsedAllStatementsRow::NonTrade(uncosted_transaction)))
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradeHistoryRow {
  #[serde(rename = "Date(UTC)")]
  date_utc: String,
  // pair: String,
  side: Side,
  // price: String,
  executed: String,
  amount: String,
  fee: String,
}
#[derive(Deserialize)]
enum Side {
  BUY,
  SELL,
}

async fn parse_trade_history(
  mut csv: Reader<File>,
) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
  let mut uncosted_transactions = Vec::new();

  for (i, row) in csv.deserialize().enumerate() {
    let uncosted_transaction = match parse_trade_history_row(row?).await {
      Ok(tx) => tx,
      Err(e) => return err!("Error in row {}: {}", i + 2, e),
    };
    uncosted_transactions.push(uncosted_transaction);
  }

  Ok(uncosted_transactions)
}

async fn parse_trade_history_row(row: TradeHistoryRow) -> Result<UncostedTransaction, String> {
  let timestamp = match Utc.datetime_from_str(&row.date_utc, "%Y-%m-%d %H:%M:%S") {
    Ok(date) => date.timestamp_millis(),
    Err(e) => throw!("Invalid date: {}", e),
  };

  let executed = Quantity::parse_with_commas(&row.executed)?;
  let amount = Quantity::parse_with_commas(&row.amount)?;

  let (from, to) = match row.side {
    Side::BUY => (amount, executed),
    Side::SELL => (executed, amount),
  };

  let base_transaction = BaseTransaction {
    tag: "Trade".into(),
    date: timestamp,
    note: "".into(),
    hash: "".into(),
    sent: Some(from.with_wallet("Binance")),
    recv: Some(to.with_wallet("Binance")),
    fee: Quantity::parse_optional(&row.fee)?,
    manual_worth: None,
  };

  let uncosted_transaction = base_transaction.into_uncosted_transaction()?;
  Ok(uncosted_transaction)
}
