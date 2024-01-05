use crate::transaction::{format_date, Quantity, Transaction};
use crate::{round_8, throw};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::slice::IterMut;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Balance {
	pub acquire_date: i64,
	pub amount: Decimal,
	pub currency: String,
	pub wallet: String,
	pub cost: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Balances(Vec<Balance>);
impl Balances {
	fn add_if_positive(&mut self, balance: Balance) {
		if balance.amount > dec!(0) || balance.cost > dec!(0) {
			let pos = self
				.0
				.binary_search_by(|current_b| current_b.acquire_date.cmp(&balance.acquire_date))
				.unwrap_or_else(|pos| pos);
			self.0.insert(pos, balance);
		}
	}
	pub fn to_inner(self) -> Vec<Balance> {
		self.0
	}
	fn iter_fifo(&mut self) -> IterMut<Balance> {
		self.0.iter_mut()
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Realized {
	pub tag: String,
	pub date: i64,
	pub input: Decimal,
	pub asset: String,
	pub is_fee: bool,
	pub output: Decimal,
	pub wallet: String,
}

#[derive(Debug)]
pub struct TaggedValue {
	pub tag: String,
	pub date: i64,
	pub quantity: Quantity,
	pub wallet: String,
	pub cost: Decimal,
}

pub struct Calculation {
	pub balances: Balances,
	pub realized_gains: Vec<Realized>,
	pub deposits: Vec<TaggedValue>,
	pub withdrawals: Vec<TaggedValue>,
}

enum DeductError {
	LessThanZero {
		amount_to_deduct: Decimal,
		asset_to_deduct: String,
		wallet_to_deduct: String,
	},
	InsufficientBalance {
		actual_balance: Decimal,
		amount_to_deduct: Decimal,
		asset_to_deduct: String,
		wallet_to_deduct: String,
	},
}

impl Calculation {
	/// Adds transactions to the calculation
	pub fn calculate(mut transactions: Vec<&Transaction>) -> Result<Self, String> {
		let mut calc = Calculation {
			balances: Balances::default(),
			realized_gains: Vec::new(),
			deposits: Vec::new(),
			withdrawals: Vec::new(),
		};

		// sort by date
		transactions.sort_by_key(|tx| tx.date());

		for transaction in transactions {
			match calc.apply_transaction(transaction) {
				Ok(()) => {}
				Err(DeductError::InsufficientBalance {
					actual_balance,
					amount_to_deduct,
					asset_to_deduct,
					wallet_to_deduct,
				}) => {
					throw!(
						"Tried to deduct {} {} from \"{}\" wallet, but the balance is only {} {}. The deduction is from a {} transaction at {}",
						amount_to_deduct,
						asset_to_deduct,
						wallet_to_deduct,
						actual_balance,
						asset_to_deduct,
						transaction.tag(),
						format_date(transaction.date()),
					);
				}
				Err(DeductError::LessThanZero {
					amount_to_deduct,
					asset_to_deduct,
					wallet_to_deduct,
				}) => {
					throw!(
						"Tried to deduct a negative amount of {} {} from \"{}\" wallet. The deduction is from a {} transaction at {}",
						amount_to_deduct,
						asset_to_deduct,
						wallet_to_deduct,
						transaction.tag(),
						format_date(transaction.date()),
					);
				}
			};
		}
		calc.clean_empty_balances();
		Ok(calc)
	}

	fn clean_empty_balances(&mut self) {
		self.balances.0.retain(|b| b.amount != dec!(0));
	}

	fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), DeductError> {
		// println!("tx {:?}", transaction);
		match transaction {
			Transaction::Trade(trade) => {
				self.balances.add_if_positive(Balance {
					acquire_date: trade.date,
					amount: trade.recv_amount,
					currency: trade.recv_asset.clone(),
					wallet: trade.recv_wallet.clone(),
					cost: trade.cost(),
				});

				let deducted = self.deduct(&trade.sent_wallet, &trade.sent_asset, trade.sent_amount)?;
				let r = Realized {
					tag: trade.tag.clone(),
					date: trade.date,
					input: sum_balance_costs(&deducted),
					asset: trade.sent_asset.clone(),
					is_fee: false,
					output: trade.cost(),
					wallet: trade.sent_wallet.clone(),
				};
				if trade.recv_asset == "BNB" || trade.sent_asset == "BNB" {
					println!("\n\n{:?}\n{:?}\n{:?}", trade, deducted, r);
				}
				self.realized_gains.push(r);

				if trade.fee_asset != "" {
					let fee_deducted = self.deduct(&trade.sent_wallet, &trade.fee_asset, trade.fee_amount)?;
					let rf = Realized {
						tag: trade.tag.clone(),
						date: trade.date,
						input: sum_balance_costs(&fee_deducted),
						asset: trade.fee_asset.clone(),
						is_fee: true,
						// TODO calculate fee output cost
						output: sum_balance_costs(&fee_deducted),
						wallet: trade.sent_wallet.clone(),
					};
					if trade.fee_asset == "BNB" {
						println!("\n\n{:?}\n{:?}\n{:?}", trade, fee_deducted, rf);
					}
					self.realized_gains.push(rf);
				};
			}
			Transaction::Transfer(transfer) => {
				if transfer.sent_amount > transfer.recv_amount {
					let fee_amount = transfer.sent_amount - transfer.recv_amount;
					let fee_deducted =
						self.deduct(&transfer.sent_wallet, &transfer.sent_asset, fee_amount)?;
					self.realized_gains.push(Realized {
						tag: transfer.tag.clone(),
						date: transfer.date,
						input: sum_balance_costs(&fee_deducted),
						asset: transfer.sent_asset.clone(),
						is_fee: true,
						// TODO calculate fee output cost
						output: sum_balance_costs(&fee_deducted),
						wallet: transfer.sent_wallet.clone(),
					});
				}

				let deducted = self.deduct(
					&transfer.sent_wallet,
					&transfer.sent_asset,
					transfer.recv_amount,
				)?;
				for mut balance in deducted {
					balance.wallet = transfer.recv_wallet.clone();
					self.balances.add_if_positive(balance);
				}
			}
			Transaction::Deposit(deposit) => {
				self.balances.add_if_positive(Balance {
					acquire_date: deposit.date,
					amount: deposit.amount,
					currency: deposit.asset.clone(),
					wallet: deposit.wallet.clone(),
					cost: deposit.cost(),
				});
				self.deposits.push(TaggedValue {
					tag: deposit.tag.clone(),
					date: deposit.date,
					quantity: Quantity {
						amount: deposit.amount,
						asset: deposit.asset.clone(),
					},
					wallet: deposit.wallet.clone(),
					cost: deposit.cost(),
				});
			}
			Transaction::Withdrawal(withdrawal) => {
				let deducted = self.deduct(&withdrawal.wallet, &withdrawal.asset, withdrawal.amount)?;
				self.realized_gains.push(Realized {
					tag: withdrawal.tag.clone(),
					date: withdrawal.date,
					input: sum_balance_costs(&deducted),
					asset: withdrawal.asset.clone(),
					is_fee: false,
					output: withdrawal.cost(),
					wallet: withdrawal.wallet.clone(),
				});
				self.withdrawals.push(TaggedValue {
					tag: withdrawal.tag.clone(),
					date: withdrawal.date,
					quantity: Quantity {
						amount: withdrawal.amount,
						asset: withdrawal.asset.clone(),
					},
					wallet: withdrawal.wallet.clone(),
					cost: withdrawal.cost(),
				});
			}
		}
		Ok(())
	}

	// Deduct from a balance. Returns the deducted amounts
	fn deduct(
		&mut self,
		wallet: &str,
		asset: &str,
		amount: Decimal,
	) -> Result<Vec<Balance>, DeductError> {
		let mut amount_left = amount;
		let mut deducted_balances = Vec::new();

		for balance in self.balances.iter_fifo() {
			if balance.wallet != wallet || balance.currency != asset {
				continue;
			}

			if amount_left < dec!(0) {
				return Err(DeductError::LessThanZero {
					amount_to_deduct: amount,
					asset_to_deduct: asset.to_string(),
					wallet_to_deduct: wallet.to_string(),
				});
			} else if amount_left >= balance.amount {
				// more stuff to deduct
				amount_left = amount_left - balance.amount;
				deducted_balances.push(balance.clone());
				balance.cost = dec!(0);
				balance.amount = dec!(0);
				if amount_left <= dec!(0) {
					break;
				}
			} else {
				// last thing to deduct
				let deduct_percent = amount_left / balance.amount;
				let cost_to_deduct = round_8(balance.cost * deduct_percent);
				deducted_balances.push(Balance {
					acquire_date: balance.acquire_date,
					amount: amount_left,
					currency: balance.currency.clone(),
					wallet: balance.wallet.clone(),
					cost: cost_to_deduct,
				});
				balance.amount -= amount_left;
				balance.cost -= cost_to_deduct;
				amount_left = dec!(0);
				break;
			}
		}

		if amount_left <= dec!(0) {
			Ok(deducted_balances)
		} else {
			Err(DeductError::InsufficientBalance {
				actual_balance: amount - amount_left,
				amount_to_deduct: amount,
				asset_to_deduct: asset.to_string(),
				wallet_to_deduct: wallet.to_string(),
			})
		}
	}
}

fn sum_balance_costs(balances: &Vec<Balance>) -> Decimal {
	balances.iter().map(|b| b.cost).sum()
}
