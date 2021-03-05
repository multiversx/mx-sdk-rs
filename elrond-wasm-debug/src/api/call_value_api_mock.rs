use super::big_uint_api_mock::*;
use crate::{TxContext, TxPanic};
use elrond_wasm::api::CallValueApi;
use elrond_wasm::err_msg;
use elrond_wasm::types::TokenIdentifier;

impl CallValueApi<RustBigUint> for TxContext {
	fn check_not_payable(&self) {
		if self.egld_value() > 0 {
			panic!(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_EGLD.to_vec(),
			});
		}
		if self.esdt_value() > 0 {
			panic!(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_ESDT.to_vec(),
			});
		}
	}

	#[inline]
	fn egld_value(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn esdt_value(&self) -> RustBigUint {
		self.tx_input_box.esdt_value.clone().into()
	}

	#[inline]
	fn token(&self) -> TokenIdentifier {
		TokenIdentifier::from(self.tx_input_box.esdt_token_identifier.as_slice())
	}
}
