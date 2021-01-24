use super::big_uint_api_mock::*;
use crate::{TxContext, TxPanic};
use elrond_wasm::api::CallValueApi;
use elrond_wasm::err_msg;
use elrond_wasm::types::BoxedBytes;

impl CallValueApi<RustBigUint> for TxContext {
	fn check_not_payable(&self) {
		if self.get_call_value_big_uint() > 0 {
			panic!(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_EGLD.to_vec(),
			});
		}
		if self.get_esdt_value_big_uint() > 0 {
			panic!(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_ESDT.to_vec(),
			});
		}
	}

	#[inline]
	fn get_call_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn get_esdt_value_big_uint(&self) -> RustBigUint {
		self.tx_input_box.esdt_value.clone().into()
	}

	#[inline]
	fn get_esdt_token_name(&self) -> BoxedBytes {
		self.tx_input_box.esdt_token_name.clone().into()
	}
}
