use crate::{num_bigint, tx_mock::TxPanic, DebugApi};
use elrond_wasm::{
    api::{CallValueApi, CallValueApiImpl, Handle},
    err_msg,
    types::EsdtTokenType,
};
use num_traits::Zero;

impl DebugApi {
    fn fail_if_more_than_one_esdt_transfer(&self) {
        if self.esdt_num_transfers() > 1 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::TOO_MANY_ESDT_TRANSFERS.to_string(),
            });
        }
    }
}

impl CallValueApi for DebugApi {
    type CallValueApiImpl = DebugApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallValueApiImpl for DebugApi {
    fn check_not_payable(&self) {
        if self.input_ref().egld_value > num_bigint::BigUint::zero() {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_EGLD.to_string(),
            });
        }
        if self.esdt_num_transfers() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_ESDT.to_string(),
            });
        }
    }

    #[inline]
    fn load_egld_value(&self, dest: Handle) {
        self.set_big_uint(dest, self.input_ref().egld_value.clone())
    }

    #[inline]
    fn load_single_esdt_value(&self, dest: Handle) {
        self.fail_if_more_than_one_esdt_transfer();
        if let Some(esdt_value) = self.input_ref().esdt_values.get(0) {
            self.set_big_uint(dest, esdt_value.value.clone());
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ESDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token(&self) -> Option<Handle> {
        self.fail_if_more_than_one_esdt_transfer();

        if self.esdt_num_transfers() > 0 {
            Some(self.token_by_index(0))
        } else {
            None
        }
    }

    #[inline]
    fn esdt_token_nonce(&self) -> u64 {
        self.fail_if_more_than_one_esdt_transfer();
        self.esdt_token_nonce_by_index(0)
    }

    #[inline]
    fn esdt_token_type(&self) -> EsdtTokenType {
        self.fail_if_more_than_one_esdt_transfer();
        self.esdt_token_type_by_index(0)
    }

    #[inline]
    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().esdt_values.len()
    }

    #[inline]
    fn esdt_value_by_index(&self, index: usize) -> Handle {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            self.insert_new_big_uint(esdt_value.value.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ESDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn token_by_index(&self, index: usize) -> Handle {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            self.insert_new_managed_buffer(esdt_value.token_identifier.clone())
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ESDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            esdt_value.nonce
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::ESDT_INVALID_TOKEN_INDEX.to_string(),
            });
        }
    }

    #[inline]
    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        if self.esdt_token_nonce_by_index(index) == 0 {
            EsdtTokenType::Fungible
        } else {
            EsdtTokenType::NonFungible
        }
    }
}
