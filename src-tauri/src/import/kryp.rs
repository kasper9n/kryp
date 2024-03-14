use super::csv::{get_cell, get_cell_index, get_header_lowercase, read_csv};
use crate::throw;
use crate::transaction::{BaseTransaction, Quantity, UncostedTransaction, Value};
use chrono::{TimeZone, Utc};
use csv::StringRecord;
use std::error::Error;
use std::path::PathBuf;

pub async fn read(
	path: PathBuf,
	tz: chrono_tz::Tz,
) -> Result<Vec<UncostedTransaction>, Box<dyn Error>> {
	let mut csv = read_csv(path)?;
	let mut uncosted_transactions = Vec::new();

	let cols = CsvCols::from_header(&get_header_lowercase(&mut csv)?)?;

	for (i, row) in csv.records().enumerate() {
		let uncosted_transaction = from_row(row?, &cols, tz)
			.await
			.map_err(|e| format!("Error in row {}: {}", i + 2, e))?;
		uncosted_transactions.push(uncosted_transaction);
	}
	Ok(uncosted_transactions)
}

fn pos(row: &Vec<String>, values: &[&str]) -> Option<usize> {
	row.iter().position(|s| values.contains(&s.as_str()))
}

#[derive(Debug)]
pub struct CsvCols {
	kind: usize,
	date: usize,
	note: usize,
	hash: usize,
	sent_amount: Option<usize>,
	sent_asset: Option<usize>,
	sent_wallet: Option<usize>,
	recv_amount: Option<usize>,
	recv_asset: Option<usize>,
	recv_wallet: Option<usize>,
	fee_amount: Option<usize>,
	fee_asset: Option<usize>,
	cost: Option<usize>,
}
impl CsvCols {
	pub fn from_header(header: &StringRecord) -> Result<Self, String> {
		let row: Vec<String> = header.iter().map(|s| s.to_lowercase()).collect();
		println!("{:?}", row);

		let sent_amount = row.iter().position(|s| s == "sent");
		let mut sent_asset = pos(&row, &["sent asset", "s asset"]);
		let mut sent_wallet = pos(&row, &["sent wallet", "s wallet"]);
		if let Some(i) = sent_amount {
			if sent_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
				sent_asset = Some(i + 1);
			}
			if sent_wallet.is_none() && row.get(i + 2) == Some(&"wallet".to_string()) {
				sent_wallet = Some(i + 2);
			}
		}

		let recv_amount = row.iter().position(|s| s == "received");
		let mut recv_asset = pos(&row, &["received asset", "recv asset", "r asset"]);
		let mut recv_wallet = pos(&row, &["received wallet", "recv wallet", "r wallet"]);
		if let Some(i) = recv_amount {
			if recv_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
				recv_asset = Some(i + 1);
			}
			if recv_wallet.is_none() && row.get(i + 2) == Some(&"wallet".to_string()) {
				recv_wallet = Some(i + 2);
			}
		}

		let fee_amount = row.iter().position(|s| s == "fee");
		let mut fee_asset = pos(&row, &["fee asset", "f asset"]);
		if let Some(i) = fee_amount {
			if fee_asset.is_none() && row.get(i + 1) == Some(&"asset".to_string()) {
				fee_asset = Some(i + 1);
			}
		}

		let cost = row.iter().position(|s| s == "cost");

		Ok(CsvCols {
			kind: get_cell_index(&row, &["type"])?,
			date: get_cell_index(&row, &["date"])?,
			note: get_cell_index(&row, &["note"])?,
			hash: get_cell_index(&row, &["hash", "tx hash"])?,
			sent_amount,
			sent_asset,
			sent_wallet,
			recv_amount,
			recv_asset,
			recv_wallet,
			fee_amount,
			fee_asset,
			cost,
		})
	}
}

fn parse_kind(kind: &str) -> &str {
	match kind {
		"Withdraw" => "Withdrawal",
		other => other,
	}
}

async fn from_row(
	row: StringRecord,
	cols: &CsvCols,
	tz: chrono_tz::Tz,
) -> Result<UncostedTransaction, String> {
	let kind = get_cell(&row, Some(cols.kind), "Kind")?;
	let date = get_cell(&row, Some(cols.date), "Date")?;
	let note = get_cell(&row, Some(cols.note), "Note")?;
	let hash = get_cell(&row, Some(cols.hash), "Hash")?;
	let cost = match cols.cost {
		Some(i) => get_cell(&row, Some(i), "Cost")?,
		None => "",
	};

	let sent_amount = get_cell(&row, cols.sent_amount, "Sent Amount")?.into();
	let sent_asset = get_cell(&row, cols.sent_asset, "Sent Asset")?.into();
	let sent_wallet = get_cell(&row, cols.sent_wallet, "Sent Wallet")?.into();

	let recv_amount = get_cell(&row, cols.recv_amount, "Received Amount")?.into();
	let recv_asset = get_cell(&row, cols.recv_asset, "Received Asset")?.into();
	let recv_wallet = get_cell(&row, cols.recv_wallet, "Received Wallet")?.into();

	let fee_amount = get_cell(&row, cols.fee_amount, "Fee Amount")?.into();
	let fee_asset = get_cell(&row, cols.fee_asset, "Fee Asset")?.into();

	let base_transaction = BaseTransaction {
		tag: parse_kind(kind).into(),
		date: match Utc.datetime_from_str(date, "%Y-%m-%d %H:%M:%S%.f UTC") {
			Ok(date) => date.timestamp_millis(),
			Err(_) => match tz.datetime_from_str(date, "%Y-%m-%d %H:%M:%S") {
				Ok(date) => date.timestamp_millis(),
				Err(e) => throw!("Invalid date \"{}\": {}", date, e),
			},
		},
		note: note.into(),
		hash: hash.into(),
		sent: Value::new_optional(sent_amount, sent_asset, sent_wallet)?,
		recv: Value::new_optional(recv_amount, recv_asset, recv_wallet)?,
		fee: Quantity::new_optional(fee_amount, fee_asset)?,
		manual_worth: Quantity::parse_optional(cost)?,
	};
	let uncosted_transaction = base_transaction.into_uncosted_transaction()?;

	println!("{:#?}", uncosted_transaction);
	Ok(uncosted_transaction)
}
