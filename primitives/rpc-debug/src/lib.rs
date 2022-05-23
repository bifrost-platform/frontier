#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use ethereum::{TransactionV0 as LegacyTransaction, TransactionV2 as Transaction};
use ethereum_types::H256;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	// Api version is virtually 4.

	#[api_version(4)]
	pub trait DebugRuntimeApi {
		#[changed_in(4)]
		fn trace_transaction(
			extrinsics: Vec<Block::Extrinsic>,
			transaction: &LegacyTransaction,
		) -> Result<(), sp_runtime::DispatchError>;

		fn trace_transaction(
			extrinsics: Vec<Block::Extrinsic>,
			transaction: &Transaction,
		) -> Result<(), sp_runtime::DispatchError>;

		fn trace_block(
			extrinsics: Vec<Block::Extrinsic>,
			known_transactions: Vec<H256>,
		) -> Result<(), sp_runtime::DispatchError>;
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode)]
pub enum TracerInput {
	None,
	Blockscout,
	CallTracer,
}

#[derive(Debug)]
pub enum Response {
	Single,
	Block,
}
