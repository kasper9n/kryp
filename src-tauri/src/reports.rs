use crate::calc::Calculation;
use crate::data::Data;
use crate::transaction::Transaction;
use crate::{save_csv_tsv, throw};
use chrono::{Local, TimeZone};
use rust_decimal::{Decimal, RoundingStrategy::AwayFromZero as AwayFrom0};
use rust_decimal_macros::dec;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use tauri::{command, State, Window};

#[derive(Serialize, Debug)]
pub struct Row {
	name: String,
	income: Decimal,
	deductible: Decimal,
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
			income: dec!(0),
			deductible: dec!(0),
		}
	}
}

#[derive(Serialize, Debug)]
pub struct Report {
	records: Vec<Row>,
	total_deductible: Decimal,
	total_income: Decimal,
	total_realized_gain: Decimal,
	total_realized_loss: Decimal,
	total_realized: Decimal,
}

#[command]
pub async fn get_report(
	year: i32,
	kryp: State<'_, Data>,
	deductible_tags: Vec<String>,
	income_tags: Vec<String>,
	hide_values_less_than: Decimal,
) -> Result<Report, String> {
	let kryp = kryp.0.lock().await;
	let transactions: Vec<&Transaction> = kryp.tax.transactions.iter().collect();

	let at_least = Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
	let at_least_ts = at_least.timestamp_millis();
	let less_than = Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap();
	let less_than_ts = less_than.timestamp_millis();
	let range = at_least_ts..less_than_ts;

	let selected_transactions: Vec<_> = transactions
		.into_iter()
		.filter(|tx| {
			return tx.date() < less_than_ts;
		})
		.collect();

	let calculation = Calculation::calculate(selected_transactions)?;
	let mut report = generate_report(calculation, range, deductible_tags, income_tags)?;
	report.records = report
		.records
		.into_iter()
		.map(|mut row| {
			row.income = row.income.round_dp_with_strategy(2, AwayFrom0);
			row.deductible = row.deductible.round_dp_with_strategy(2, AwayFrom0);
			row.realized = row.realized.round_dp_with_strategy(2, AwayFrom0);
			row.realized_gain = row.realized_gain.round_dp_with_strategy(2, AwayFrom0);
			row.realized_loss = row.realized_loss.round_dp_with_strategy(2, AwayFrom0);
			row
		})
		.filter(|row| {
			row.deductible >= hide_values_less_than
				|| row.income >= hide_values_less_than
				|| row.realized_gain >= hide_values_less_than
				|| row.realized_loss >= hide_values_less_than
		})
		.collect();
	Ok(report)
}

#[command]
pub async fn download_report(
	year: i32,
	win: Window,
	kryp: State<'_, Data>,
	deductible_tags: Vec<String>,
	income_tags: Vec<String>,
	hide_values_less_than: Decimal,
) -> Result<(), String> {
	let file_name = format!("Kryp Report {}", year);
	let file_path = match save_csv_tsv(&win, &file_name) {
		Some(p) => p,
		None => return Ok(()),
	};

	let report = get_report(
		year,
		kryp,
		deductible_tags,
		income_tags,
		hide_values_less_than,
	)
	.await?;

	let file_path = file_path.into_path().unwrap();
	let mut writer = match csv::Writer::from_path(file_path) {
		Ok(writer) => writer,
		Err(e) => throw!("Unable to write to file: {}", e),
	};
	let header_record = vec![
		"Name",
		"Income",
		"Deductible",
		"Realized",
		"Realized Gain",
		"Realized Loss",
	];
	match writer.write_record(&header_record) {
		Ok(()) => {}
		Err(e) => throw!("Unable to write row: {}", e),
	};

	for record in report.records {
		let record = vec![
			record.name,
			record.income.to_string(),
			record.deductible.to_string(),
			record.realized.to_string(),
			record.realized_gain.to_string(),
			record.realized_loss.to_string(),
		];
		match writer.write_record(&record) {
			Ok(()) => {}
			Err(e) => throw!("Unable to write row: {}", e),
		};
	}

	Ok(())
}

#[derive(Serialize, Debug)]
pub struct DWTags {
	withdrawal_tags: HashSet<String>,
	deposit_tags: HashSet<String>,
}
#[command]
pub async fn get_deposit_withdrawal_tags(kryp: State<'_, Data>) -> Result<DWTags, String> {
	let kryp = kryp.0.lock().await;

	let mut withdrawal_tags: HashSet<String> = HashSet::new();
	withdrawal_tags.insert("Withdrawal".into());
	withdrawal_tags.insert("Sell".into());
	withdrawal_tags.insert("Spend".into());
	withdrawal_tags.insert("Lost".into());

	let mut deposit_tags: HashSet<String> = HashSet::new();
	deposit_tags.insert("Deposit".into());
	deposit_tags.insert("Buy".into());
	deposit_tags.insert("Income".into());
	deposit_tags.insert("Gift".into());
	deposit_tags.insert("Interest".into());

	for transaction in &kryp.tax.transactions {
		match transaction {
			Transaction::Deposit(deposit) => {
				deposit_tags.insert(deposit.tag.clone());
			}
			Transaction::Withdrawal(withdrawal) => {
				withdrawal_tags.insert(withdrawal.tag.clone());
			}
			_ => {}
		}
	}
	Ok(DWTags {
		withdrawal_tags,
		deposit_tags,
	})
}

fn generate_report(
	calculation: Calculation,
	range: Range<i64>,
	deductible_tags: Vec<String>,
	income_tags: Vec<String>,
) -> Result<Report, String> {
	let mut report_map: HashMap<String, Row> = HashMap::new();

	let mut total_realized_gain = dec!(0);
	let mut total_realized_loss = dec!(0);
	let mut total_deductible = dec!(0);
	let mut total_income = dec!(0);

	for realized in calculation.realized_gains {
		let tag_excluded = deductible_tags.contains(&realized.tag);
		if range.contains(&realized.date) && !tag_excluded {
			let row = report_map
				.entry(realized.asset.clone())
				.or_insert(Row::new(realized.asset.clone()));
			if realized.asset == "BNB" {
				println!("{:?}", realized);
			}
			if realized.output > realized.input {
				let gain = realized.output - realized.input;
				row.realized_gain += gain;
				row.realized += gain;
				total_realized_gain += gain;
			} else {
				let loss = realized.input - realized.output;
				row.realized_loss += loss;
				row.realized -= loss;
				total_realized_loss += loss;
			}
		}
	}
	for withdrawal in calculation.withdrawals {
		let is_deductible_tag = deductible_tags.contains(&withdrawal.tag);
		if range.contains(&withdrawal.date) && is_deductible_tag {
			let row = report_map
				.entry(withdrawal.quantity.asset.clone())
				.or_insert(Row::new(withdrawal.quantity.asset.clone()));
			row.deductible += withdrawal.cost;
			total_deductible += withdrawal.cost;
		}
	}
	for deposit in calculation.deposits {
		let is_income_tag = income_tags.contains(&deposit.tag);
		if range.contains(&deposit.date) && is_income_tag {
			let row = report_map
				.entry(deposit.quantity.asset.clone())
				.or_insert(Row::new(deposit.quantity.asset.clone()));
			row.income += deposit.cost;
			total_income += deposit.cost;
		}
	}

	let mut records: Vec<Row> = report_map.into_values().collect();
	records.sort_by(|a, b| a.name.cmp(&b.name));

	Ok(Report {
		records,
		total_realized_gain,
		total_realized_loss,
		total_deductible,
		total_income,
		total_realized: total_realized_gain - total_realized_loss,
	})
}
