extern crate alloc;

pub mod evm;
pub mod gasometer;
pub mod runtime;

pub use self::evm::EvmEvent;
pub use gasometer::GasometerEvent;
pub use runtime::RuntimeEvent;

use codec::{Decode, Encode};
use ethereum_types::{H160, U256};
use sp_runtime_interface::pass_by::PassByCodec;

environmental::environmental!(listener: dyn Listener + 'static);

pub fn using<R, F: FnOnce() -> R>(l: &mut (dyn Listener + 'static), f: F) -> R {
	listener::using(l, f)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode, Default, PassByCodec)]
pub struct StepEventFilter {
	pub enable_stack: bool,
	pub enable_memory: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub enum Event {
	Evm(EvmEvent),
	Gasometer(GasometerEvent),
	Runtime(RuntimeEvent),
	CallListNew(),
}

impl Event {
	pub fn emit(self) {
		listener::with(|listener| listener.event(self));
	}
}

pub trait Listener {
	fn event(&mut self, event: Event);

	fn step_event_filter(&self) -> StepEventFilter;
}

pub fn step_event_filter() -> Option<StepEventFilter> {
	let mut filter = None;
	listener::with(|listener| filter = Some(listener.step_event_filter()));
	filter
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct Context {
	/// Execution address.
	pub address: H160,
	/// Caller of the EVM.
	pub caller: H160,
	/// Apparent value of the EVM.
	pub apparent_value: U256,
}

impl From<evm_runtime::Context> for Context {
	fn from(i: evm_runtime::Context) -> Self {
		Self {
			address: i.address,
			caller: i.caller,
			apparent_value: i.apparent_value,
		}
	}
}
