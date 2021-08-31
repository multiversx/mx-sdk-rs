use crate::{TxContext, TxPanic};
use elrond_wasm::api::CallValueApi;
use elrond_wasm::err_msg;
use elrond_wasm::types::{BigUint, EsdtTokenType, ManagedBuffer, TokenIdentifier};

impl CallValueApi for TxContext {
    type TypeManager = Self;

    fn type_manager(&self) -> Self::TypeManager {
        self.clone()
    }

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
    fn egld_value(&self) -> BigUint<Self::TypeManager> {
        self.insert_new_big_uint(self.tx_input_box.call_value.clone())
    }

    #[inline]
    fn esdt_value(&self) -> BigUint<Self::TypeManager> {
        self.insert_new_big_uint(self.tx_input_box.esdt_value.clone())
    }

    #[inline]
    fn token(&self) -> TokenIdentifier<Self::TypeManager> {
        ManagedBuffer::new_from_bytes(
            self.type_manager(),
            self.tx_input_box.esdt_token_identifier.as_slice(),
        )
        .into()
    }

    #[inline]
    fn esdt_token_nonce(&self) -> u64 {
        // TODO: Add ESDT nonce in mock
        0u64
    }

    #[inline]
    fn esdt_token_type(&self) -> EsdtTokenType {
        // TODO: Add ESDT token type in mock
        EsdtTokenType::Fungible
    }

    // TODO: Mock multi-transfers

    #[inline]
    fn esdt_num_transfers(&self) -> usize {
        0
    }

    #[inline]
    fn esdt_value_by_index(&self, _index: usize) -> BigUint<Self::TypeManager> {
        self.insert_new_big_uint_zero()
    }

    #[inline]
    fn token_by_index(&self, _index: usize) -> TokenIdentifier<Self::TypeManager> {
        TokenIdentifier::egld(self.type_manager())
    }

    #[inline]
    fn esdt_token_nonce_by_index(&self, _index: usize) -> u64 {
        0
    }

    #[inline]
    fn esdt_token_type_by_index(&self, _index: usize) -> EsdtTokenType {
        EsdtTokenType::Fungible
    }
}
