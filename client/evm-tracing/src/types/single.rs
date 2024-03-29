use super::serialization::*;
use serde::Serialize;

use codec::{Decode, Encode};
use ethereum_types::{H256, U256};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Call {
	Blockscout(crate::formatters::blockscout::BlockscoutCall),
	CallTracer(crate::formatters::call_tracer::CallTracerCall),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode)]
pub enum TraceType {
	/// Classic geth with no javascript based tracing.
	Raw {
		disable_storage: bool,
		disable_memory: bool,
		disable_stack: bool,
	},
	/// List of calls and subcalls formatted with an input tracer (i.e. callTracer or Blockscout).
	CallList,
	/// A single block trace. Use in `debug_traceTransactionByNumber` / `traceTransactionByHash`.
	Block,
}

/// Single transaction trace.
#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TransactionTrace {
	/// Classical output of `debug_trace`.
	#[serde(rename_all = "camelCase")]
	Raw {
		failed: bool,
		gas: U256,
		#[serde(with = "hex")]
		return_value: Vec<u8>,
		struct_logs: Vec<RawStructLog>,
	},
	/// Matches the formatter used by Blockscout.
	/// Is also used to built output of OpenEthereum's `trace_filter`.
	CallList(Vec<Call>),
	/// Used by Geth's callTracer.
	CallListNested(Call),
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawStructLog {
	#[serde(serialize_with = "u256_serialize")]
	pub depth: U256,

	#[serde(serialize_with = "u256_serialize")]
	pub gas: U256,

	#[serde(serialize_with = "u256_serialize")]
	pub gas_cost: U256,

	#[serde(
	serialize_with = "seq_h256_serialize",
	skip_serializing_if = "Option::is_none"
	)]
	pub memory: Option<Vec<H256>>,

	#[serde(serialize_with = "opcode_serialize")]
	pub op: Vec<u8>,

	#[serde(serialize_with = "u256_serialize")]
	pub pc: U256,

	#[serde(
	serialize_with = "seq_h256_serialize",
	skip_serializing_if = "Option::is_none"
	)]
	pub stack: Option<Vec<H256>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub storage: Option<BTreeMap<H256, H256>>,
}
