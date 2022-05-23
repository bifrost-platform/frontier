use std::collections::HashMap;
use ethereum_types::{H160, H256, U256};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TxPoolResult<T: Serialize> {
	pub pending: T,
	pub queued: T,
}

pub trait GetT {
	fn get(hash: H256, from_address: H160, txn: &ethereum::TransactionV2) -> Self;
}

pub type PoolTransactionMap<T> = HashMap<H160, HashMap<U256, T>>;
