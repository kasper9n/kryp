use crate::calc::Calculation;
use crate::data::Data;
use crate::transaction::Transaction;
use chrono::{Local, TimeZone};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::Range;
use tauri::{command, State};

#[derive(Serialize, Debug)]
pub struct Row {
  name: String,
  realized_gain: Decimal,
  realized_loss: Decimal,
  realized: Decimal,
}
impl Row {
  fn new(name: String) -> Self {
    Self {
      name,
      realized_gain: dec!(0),
      realized_loss: dec!(0),
      realized: dec!(0),
    }
  }
}

#[derive(Serialize, Debug)]
pub struct Report {
  records: Vec<Row>,
  realized_gain: Decimal,
  realized_loss: Decimal,
  realized: Decimal,
}

#[command]
pub async fn get_report(year: i32, kryp: State<'_, Data>) -> Result<Report, String> {
  let kryp = kryp.0.lock().await;
  let transactions: Vec<&Transaction> = kryp.tax.transactions.iter().collect();

  let at_least = Local.ymd(year, 1, 1).and_hms(0, 0, 0);
  let at_least_ts = at_least.timestamp_millis();
  let less_than = Local.ymd(year + 1, 1, 1).and_hms(0, 0, 0);
  let less_than_ts = less_than.timestamp_millis();
  let range = at_least_ts..less_than_ts;

  let selected_transactions: Vec<_> = transactions
    .into_iter()
    .filter(|tx| {
      return tx.date() < less_than_ts;
    })
    .collect();

  let calculation = Calculation::calculate(selected_transactions)?;
  let report = generate_report(calculation, range)?;
  Ok(report)
}

fn generate_report(calculation: Calculation, range: Range<i64>) -> Result<Report, String> {
  let mut report_map: HashMap<String, Row> = HashMap::new();

  let mut total_realized_gain = dec!(0);
  let mut total_realized_loss = dec!(0);

  for realized in calculation.realized_gains {
    if range.contains(&realized.date) {
      let row = report_map
        .entry(realized.asset.clone())
        .or_insert(Row::new(realized.asset.clone()));
      if realized.output > realized.input {
        let gain = realized.output - realized.input;
        row.realized_gain += gain;
        total_realized_gain += gain;
      } else {
        let loss = realized.input - realized.output;
        row.realized_loss += loss;
        total_realized_loss += loss;
      }
    }
  }

  let mut records: Vec<Row> = report_map.into_values().collect();
  records.sort_by(|a, b| a.name.cmp(&b.name));

  Ok(Report {
    records,
    realized_gain: total_realized_gain,
    realized_loss: total_realized_loss,
    realized: total_realized_gain - total_realized_loss,
  })
}
