use ethereum_types::U256;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

pub use crate::types::{PoolTransactionMap, PoolTransaction, PoolSummary, TxPoolResult};

pub use rpc_impl_TxPoolApi::gen_server::TxPoolApi as TxPoolApiServer;


#[rpc(server)]
pub trait TxPoolApi {
	#[rpc(name = "txpool_content")]
	fn content(&self) -> Result<TxPoolResult<PoolTransactionMap<PoolTransaction>>>;

	#[rpc(name = "txpool_inspect")]
	fn inspect(&self) -> Result<TxPoolResult<PoolTransactionMap<PoolSummary>>>;

	#[rpc(name = "txpool_status")]
	fn status(&self) -> Result<TxPoolResult<U256>>;
}
