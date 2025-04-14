use crate::prices::{symbol_kind, AssetKind, PriceData};
use crate::tax::Api;
use crate::{round_8, throw};
use chrono::{Local, TimeZone};
use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::str::{Chars, FromStr};

pub fn format_date(ts: i64) -> String {
	let dt = Local.timestamp_millis_opt(ts).unwrap();
	dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
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
	pub manual_worth: Option<String>,
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
			manual_worth: None,
			cost: dec!(0),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
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
	pub manual_worth: Option<String>,
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
			manual_worth: None,
			cost: dec!(0),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
#[serde(deny_unknown_fields)]
pub struct Deposit {
	pub tag: String,
	pub date: i64,
	pub note: String,
	pub hash: String,
	pub amount: Decimal,
	pub asset: String,
	pub wallet: String,
	pub manual_worth: Option<String>,
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
			manual_worth: None,
			cost: dec!(0),
		}
	}
	pub fn cost(&self) -> Decimal {
		self.cost
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
#[serde(deny_unknown_fields)]
pub struct Withdrawal {
	pub tag: String,
	pub date: i64,
	pub note: String,
	pub hash: String,
	pub amount: Decimal,
	pub asset: String,
	pub wallet: String,
	pub manual_worth: Option<String>,
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
			manual_worth: None,
			cost: dec!(0),
		}
	}
}

/// A transaction without a final cost set
#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
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
		if let Some(mw) = self.manual_worth_qty()? {
			if mw.asset == base {
				Ok(mw.amount.clone())
			} else {
				let cost = price_data
					.get_value(mw.amount.clone(), &mw.asset, self.date(), apis, base)
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
	pub fn manual_worth(&self) -> &Option<String> {
		match self {
			UncostedTransaction::Trade(tx) => &tx.manual_worth,
			UncostedTransaction::Transfer(tx) => &tx.manual_worth,
			UncostedTransaction::Deposit(tx) => &tx.manual_worth,
			UncostedTransaction::Withdrawal(tx) => &tx.manual_worth,
		}
	}
	/// Gets the manual worth of a transaction as a Quantity
	pub fn manual_worth_qty(&self) -> Result<Option<Quantity>, String> {
		match self.manual_worth() {
			Some(manual_worth) => Quantity::parse_optional(&manual_worth),
			None => Ok(None),
		}
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
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
#[serde(tag = "type")]
pub enum Transaction {
	Trade(Trade),
	Transfer(Transfer),
	Deposit(Deposit),
	Withdrawal(Withdrawal),
}

impl Transaction {
	pub fn tag(&self) -> &String {
		match self {
			Transaction::Trade(tx) => &tx.tag,
			Transaction::Transfer(tx) => &tx.tag,
			Transaction::Deposit(tx) => &tx.tag,
			Transaction::Withdrawal(tx) => &tx.tag,
		}
	}
	pub fn date(&self) -> i64 {
		match self {
			Transaction::Trade(tx) => tx.date,
			Transaction::Transfer(tx) => tx.date,
			Transaction::Deposit(tx) => tx.date,
			Transaction::Withdrawal(tx) => tx.date,
		}
	}
	pub fn recv_asset(&self) -> Option<&String> {
		match self {
			Transaction::Trade(tx) => Some(&tx.recv_asset),
			Transaction::Transfer(tx) => Some(&tx.recv_asset),
			Transaction::Deposit(tx) => Some(&tx.asset),
			Transaction::Withdrawal(_) => None,
		}
	}
	pub fn sent_asset(&self) -> Option<&String> {
		match self {
			Transaction::Trade(tx) => Some(&tx.sent_asset),
			Transaction::Transfer(tx) => Some(&tx.sent_asset),
			Transaction::Deposit(_) => None,
			Transaction::Withdrawal(tx) => Some(&tx.asset),
		}
	}
	pub fn fee_asset(&self) -> Option<&String> {
		match self {
			Transaction::Trade(tx) => Some(&tx.fee_asset),
			Transaction::Transfer(_) => None,
			Transaction::Deposit(_) => None,
			Transaction::Withdrawal(_) => None,
		}
	}
	pub fn manual_worth<'a>(&'a self) -> &'a Option<String> {
		match self {
			Transaction::Trade(tx) => &tx.manual_worth,
			Transaction::Transfer(tx) => &tx.manual_worth,
			Transaction::Deposit(tx) => &tx.manual_worth,
			Transaction::Withdrawal(tx) => &tx.manual_worth,
		}
	}
	pub fn to_csv_record<'a>(&'a self) -> Vec<String> {
		let empty = "".to_string();
		let manual_worth = self.manual_worth().as_ref().unwrap_or(&empty);
		match self {
			Transaction::Trade(trade) => {
				let feeless = trade.fee_amount == dec!(0) && trade.fee_asset == "";
				vec![
					trade.tag.clone(),
					trade.sent_amount.to_string(),
					trade.sent_asset.clone(),
					trade.sent_wallet.clone(),
					trade.recv_amount.to_string(),
					trade.recv_asset.clone(),
					trade.recv_wallet.clone(),
					if feeless {
						"".to_string()
					} else {
						trade.fee_amount.to_string()
					},
					trade.fee_asset.clone(),
					trade.note.clone(),
					trade.hash.clone(),
					chrono::Utc
						.timestamp_millis_opt(trade.date)
						.unwrap()
						.to_string(),
					manual_worth.to_string(),
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
					chrono::Utc
						.timestamp_millis_opt(transfer.date)
						.unwrap()
						.to_string(),
					manual_worth.to_string(),
				]
			}
			Transaction::Deposit(deposit) => {
				vec![
					deposit.tag.clone(),
					"".to_string(),
					"".to_string(),
					"".to_string(),
					deposit.amount.to_string(),
					deposit.asset.clone(),
					deposit.wallet.clone(),
					"".to_string(),
					"".to_string(),
					deposit.note.clone(),
					deposit.hash.clone(),
					chrono::Utc
						.timestamp_millis_opt(deposit.date)
						.unwrap()
						.to_string(),
					manual_worth.to_string(),
				]
			}
			Transaction::Withdrawal(withdrawal) => {
				vec![
					withdrawal.tag.clone(),
					withdrawal.amount.to_string(),
					withdrawal.asset.clone(),
					withdrawal.wallet.clone(),
					"".to_string(),
					"".to_string(),
					"".to_string(),
					"".to_string(),
					"".to_string(),
					withdrawal.note.clone(),
					withdrawal.hash.clone(),
					chrono::Utc
						.timestamp_millis_opt(withdrawal.date)
						.unwrap()
						.to_string(),
					manual_worth.to_string(),
				]
			}
		}
	}
}

fn take_decimal_from_chars(chars: &mut Peekable<Chars>) -> Option<Decimal> {
	let mut num_str = "".to_string();
	let mut found_period = false;
	loop {
		let c = chars.peek()?;
		if c.is_ascii_digit() {
			num_str.push(chars.next().unwrap());
		} else if c == &'.' && !found_period {
			num_str.push(chars.next().unwrap());
			found_period = true;
		} else {
			break;
		}
	}
	let amount = Decimal::from_str(&num_str).ok()?;
	Some(amount)
}

lazy_static! {
	static ref QUANTITY_COMMA_PATTERN: Regex =
		Regex::new(r"^((\d+[\d,])*\d+(.\d+)*) *(.+)$").unwrap();
}

#[derive(Debug, Clone)]
pub struct Quantity {
	pub amount: Decimal,
	pub asset: String,
}
impl Quantity {
	pub fn new(amount: String, asset: String) -> Result<Quantity, String> {
		if amount == "" {
			throw!("Invalid amount \"{}\"", amount);
		} else if asset == "" {
			throw!("Invalid asset \"{}\"", asset);
		} else {
			let num = match Decimal::from_str(&amount) {
				Ok(d) => d,
				Err(_) => match Decimal::from_scientific(&amount) {
					Ok(d) => d,
					Err(_) => throw!("Invalid number \"{}\"", amount),
				},
			};
			Ok(Quantity { amount: num, asset })
		}
	}
	/// Returns `None` if both the amount and asset are empty
	pub fn new_optional(amount: String, asset: String) -> Result<Option<Quantity>, String> {
		if amount == "" && asset == "" {
			Ok(None)
		} else {
			Ok(Some(Quantity::new(amount, asset)?))
		}
	}
	pub fn with_wallet(self, wallet: impl Into<String>) -> Value {
		Value {
			amount: self.amount,
			asset: self.asset,
			wallet: wallet.into(),
		}
	}
	pub fn parse(value: &str) -> Result<Self, String> {
		if value.trim() == "" {
			throw!("Empty");
		}
		let mut chars = value.chars().peekable();
		let amount = match take_decimal_from_chars(&mut chars) {
			Some(amount) => amount,
			None => throw!("Invalid quantity \"{}\"", value),
		};

		let asset_str: String = chars.collect();
		let asset = asset_str.trim().to_string();

		Ok(Self { amount, asset })
	}
	pub fn parse_with_commas(value: &str) -> Result<Self, String> {
		if value.trim() == "" {
			throw!("Empty");
		}

		let caps = match QUANTITY_COMMA_PATTERN.captures(value) {
			Some(caps) => caps,
			None => throw!("Invalid quantity \"{}\"", value),
		};
		let amount = match Decimal::from_str(&caps[1].replace(",", "")) {
			Ok(d) => d,
			Err(_) => throw!("Invalid quantity \"{}\"", value),
		};

		Ok(Self {
			amount,
			asset: caps[4].to_string(),
		})
	}
	pub fn parse_optional(value: &str) -> Result<Option<Self>, String> {
		if value.trim() == "" {
			return Ok(None);
		}
		Ok(Some(Self::parse(value)?))
	}
	pub fn to_string(&self) -> String {
		self.amount.to_string() + " " + &self.asset
	}
}
pub struct Value {
	pub amount: Decimal,
	pub asset: String,
	pub wallet: String,
}
impl Value {
	pub fn new(
		amount: impl Into<String>,
		asset: impl Into<String>,
		wallet: impl Into<String>,
	) -> Result<Value, String> {
		let amount = amount.into();
		let asset = asset.into();
		let wallet = wallet.into();
		if asset != "" && amount != "" && wallet == "" {
			throw!("The amount {} {} has no wallet", amount, asset);
		} else if asset == "" || amount == "" || wallet == "" {
			throw!("Invalid amount {} {} with wallet {}", amount, asset, wallet,);
		} else {
			let quantity = Quantity::new(amount, asset)?;
			Ok(Value {
				amount: quantity.amount,
				asset: quantity.asset,
				wallet: wallet,
			})
		}
	}
	/// Returns `None` if all arguments are empty
	pub fn new_optional(
		amount: String,
		asset: String,
		wallet: String,
	) -> Result<Option<Value>, String> {
		if amount == "" && asset == "" && wallet == "" {
			Ok(None)
		} else {
			Ok(Some(Value::new(amount, asset, wallet)?))
		}
	}
}

/// A general unspecific transaction
pub struct BaseTransaction {
	pub tag: String,
	pub date: i64,
	pub note: String,
	pub hash: String,
	pub sent: Option<Value>,
	pub recv: Option<Value>,
	pub fee: Option<Quantity>,
	pub manual_worth: Option<Quantity>,
}
impl BaseTransaction {
	pub fn into_uncosted_transaction(self) -> Result<UncostedTransaction, String> {
		if let Some(sent) = &self.sent {
			if sent.amount < dec!(0) {
				throw!(
					"Tried to create a transaction with a negative \"sent\" amount of {} {}",
					sent.amount,
					sent.asset
				);
			}
		}
		if let Some(fee) = &self.fee {
			if fee.amount < dec!(0) {
				throw!(
					"Tried to create a transaction with a negative \"fee\" of {} {}",
					fee.amount,
					fee.asset
				);
			}
		}
		if let Some(recv) = &self.recv {
			if recv.amount < dec!(0) {
				throw!(
					"Tried to create a transaction with a negative \"received\" amount of {} {}",
					recv.amount,
					recv.asset
				);
			}
		}

		let manual_worth_str = self.manual_worth.map(|q| q.to_string());
		let uncosted_transaction = match self.tag.as_str() {
			"Trade" => {
				let sent = self
					.sent
					.ok_or(format!("Sent amount is missing from {}", self.tag))?;
				let recv = self
					.recv
					.ok_or(format!("Received amount is missing from {}", self.tag))?;
				let fee = self.fee.unwrap_or(Quantity {
					amount: dec!(0),
					asset: "".into(),
				});
				UncostedTransaction::Trade(Trade {
					tag: self.tag,
					date: self.date,
					note: self.note,
					hash: self.hash,

					sent_amount: sent.amount,
					sent_asset: sent.asset,
					sent_wallet: sent.wallet,

					recv_amount: recv.amount,
					recv_asset: recv.asset,
					recv_wallet: recv.wallet,

					fee_amount: fee.amount,
					fee_asset: fee.asset,

					manual_worth: manual_worth_str,
					cost: dec!(0),
				})
			}
			"Transfer" => {
				let sent = self
					.sent
					.ok_or(format!("Sent amount is missing from {}", self.tag))?;
				let recv = self
					.recv
					.ok_or(format!("Received amount is missing from {}", self.tag))?;
				if self.fee.is_some() {
					throw!("Fee is not allowed for {}", self.tag);
				}
				UncostedTransaction::Transfer(Transfer {
					tag: self.tag,
					date: self.date,
					note: self.note,
					hash: self.hash,

					sent_amount: sent.amount,
					sent_asset: sent.asset,
					sent_wallet: sent.wallet,

					recv_amount: recv.amount,
					recv_asset: recv.asset,
					recv_wallet: recv.wallet,

					manual_worth: manual_worth_str,
					cost: dec!(0),
				})
			}
			"Deposit" | "Buy" | "Income" | "Gift" | "Interest" => {
				if self.sent.is_some() {
					throw!("Sent amount is not allowed for {}", self.tag);
				}
				let recv = self
					.recv
					.ok_or(format!("Received amount is missing from {}", self.tag))?;
				if self.fee.is_some() {
					throw!("Fee is not allowed for {}", self.tag);
				}
				UncostedTransaction::Deposit(Deposit {
					tag: self.tag,
					date: self.date,
					note: self.note,
					hash: self.hash,

					amount: recv.amount,
					asset: recv.asset,
					wallet: recv.wallet,

					manual_worth: manual_worth_str,
					cost: dec!(0),
				})
			}
			"Withdrawal" | "Sell" | "Spend" | "Lost" => {
				let sent = self
					.sent
					.ok_or(format!("Sent amount is missing from {}", self.tag))?;
				if self.recv.is_some() {
					throw!("Received amount is not allowed for {}", self.tag);
				}
				if self.fee.is_some() {
					throw!("Fee is not allowed for {}", self.tag);
				}
				UncostedTransaction::Withdrawal(Withdrawal {
					tag: self.tag,
					date: self.date,
					note: self.note,
					hash: self.hash,

					amount: sent.amount,
					asset: sent.asset,
					wallet: sent.wallet,

					manual_worth: manual_worth_str,
					cost: dec!(0),
				})
			}
			_ => throw!("Invalid type \"{}\"", self.tag),
		};
		Ok(uncosted_transaction)
	}
}
