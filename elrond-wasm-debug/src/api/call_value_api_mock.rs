use crate::{tx_mock::TxPanic, DebugApi};
use elrond_wasm::{
    api::CallValueApi,
    err_msg,
    types::{BigUint, EsdtTokenType, TokenIdentifier},
};

impl DebugApi {
    fn fail_if_more_than_one_esdt_transfer(&self) {
        if self.esdt_num_transfers() > 1 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::TOO_MANY_ESDT_TRANSFERS.to_vec(),
            });
        }
    }
}

impl CallValueApi for DebugApi {
    fn check_not_payable(&self) {
        if self.egld_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_EGLD.to_vec(),
            });
        }
        if self.esdt_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_ESDT.to_vec(),
            });
        }
    }

    #[inline]
    fn egld_value(&self) -> BigUint<Self> {
        self.insert_new_big_uint(self.input_ref().egld_value.clone())
    }

    #[inline]
    fn esdt_value(&self) -> BigUint<Self> {
        self.fail_if_more_than_one_esdt_transfer();
        self.esdt_value_by_index(0)
    }

    #[inline]
    fn token(&self) -> TokenIdentifier<Self> {
        self.fail_if_more_than_one_esdt_transfer();
        self.token_by_index(0)
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
    fn esdt_value_by_index(&self, index: usize) -> BigUint<Self> {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            self.insert_new_big_uint(esdt_value.value.clone())
        } else {
            self.insert_new_big_uint_zero()
        }
    }

    #[inline]
    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self> {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            TokenIdentifier::from(
                self.insert_new_managed_buffer(esdt_value.token_identifier.clone()),
            )
        } else {
            TokenIdentifier::egld()
        }
    }

    #[inline]
    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        if let Some(esdt_value) = self.input_ref().esdt_values.get(index) {
            esdt_value.nonce
        } else {
            0
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
