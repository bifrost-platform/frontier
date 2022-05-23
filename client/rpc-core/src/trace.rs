use ethereum_types::H160;
use futures::future::BoxFuture;
use jsonrpc_derive::rpc;
use fc_evm_tracing::types::block::TransactionTrace;
use crate::types::RequestBlockId;
use serde::Deserialize;


pub use rpc_impl_Trace::gen_server::Trace as TraceServer;

#[rpc(server)]
pub trait Trace {
	#[rpc(name = "trace_filter")]
	fn filter(
		&self,
		filter: FilterRequest,
	) -> BoxFuture<'static, jsonrpc_core::Result<Vec<TransactionTrace>>>;
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRequest {
	/// (optional?) From this block.
	pub from_block: Option<RequestBlockId>,

	/// (optional?) To this block.
	pub to_block: Option<RequestBlockId>,

	/// (optional) Sent from these addresses.
	pub from_address: Option<Vec<H160>>,

	/// (optional) Sent to these addresses.
	pub to_address: Option<Vec<H160>>,

	/// (optional) The offset trace number
	pub after: Option<u32>,

	/// (optional) Integer number of traces to display in a batch.
	pub count: Option<u32>,
}
