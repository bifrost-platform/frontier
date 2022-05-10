use crate::types::GetT;
use ethereum::{TransactionAction, TransactionV2 as EthereumTransaction};
use ethereum_types::{H160, H256, U256};
use crate::types::Bytes;
use serde::{Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	pub hash: H256,
	pub nonce: U256,

	#[serde(serialize_with = "block_hash_serialize")]
	pub block_hash: Option<H256>,

	pub block_number: Option<U256>,
	pub from: H160,

	#[serde(serialize_with = "to_serialize")]
	pub to: Option<H160>,

	pub value: U256,
	pub gas_price: U256,
	pub gas: U256,
	pub input: Bytes,
	pub transaction_index: Option<U256>,
}

fn block_hash_serialize<S>(hash: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
{
	serializer.serialize_str(&format!("0x{:x}", hash.unwrap_or_default()))
}

fn to_serialize<S>(hash: &Option<H160>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
{
	serializer.serialize_str(&format!("0x{:x}", hash.unwrap_or_default()))
}

impl GetT for Transaction {
	fn get(hash: H256, from_address: H160, txn: &EthereumTransaction) -> Self {
		let (nonce, action, value, gas_price, gas_limit, input) = match txn {
			EthereumTransaction::Legacy(t) => (
				t.nonce,
				t.action,
				t.value,
				t.gas_price,
				t.gas_limit,
				t.input.clone(),
			),
			EthereumTransaction::EIP2930(t) => (
				t.nonce,
				t.action,
				t.value,
				t.gas_price,
				t.gas_limit,
				t.input.clone(),
			),
			EthereumTransaction::EIP1559(t) => (
				t.nonce,
				t.action,
				t.value,
				t.max_fee_per_gas,
				t.gas_limit,
				t.input.clone(),
			),
		};
		Self {
			hash,
			nonce,
			block_hash: None,
			block_number: None,
			from: from_address,
			to: match action {
				TransactionAction::Call(to) => Some(to),
				_ => None,
			},
			value,
			gas_price,
			gas: gas_limit,
			input: Bytes(input),
			transaction_index: None,
		}
	}
}
