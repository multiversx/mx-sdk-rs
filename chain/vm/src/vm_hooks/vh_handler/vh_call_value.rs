use crate::{
    tx_mock::TxTokenTransfer, types::RawHandle, vm_err_msg, vm_hooks::VMHooksHandlerSource,
};
use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;
use num_traits::Zero;

use super::VMHooksManagedTypes;

pub trait VMHooksCallValue: VMHooksHandlerSource + VMHooksManagedTypes {
    fn check_not_payable(&self) {
        if self.input_ref().egld_value > num_bigint::BigUint::zero() {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_EGLD);
        }
        if self.esdt_num_transfers() > 0 {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_ESDT);
        }
    }

    fn load_egld_value(&self, dest: RawHandle) {
        let value = self.input_ref().received_egld().clone();
        self.m_types_lock().bi_overwrite(dest, value.into());
    }

    fn load_all_esdt_transfers(&self, dest_handle: RawHandle) {
        let transfers = self.input_ref().received_esdt();
        self.m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, transfers);
    }

    fn load_all_transfers(&self, dest_handle: RawHandle) {
        let direct_egld_value = self.input_ref().received_egld().clone();
        let transfers = if !direct_egld_value.is_zero() {
            vec![TxTokenTransfer {
                token_identifier: EGLD_000000_TOKEN_IDENTIFIER.as_bytes().to_vec(),
                nonce: 0,
                value: direct_egld_value,
            }]
        } else {
            self.input_ref().received_esdt().to_owned()
        };
        self.m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, &transfers);
    }

    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
