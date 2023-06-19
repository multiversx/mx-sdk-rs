use crate::{num_bigint, tx_mock::TxPanic, vm_hooks::VMHooksHandlerSource};
use multiversx_sc::{api::RawHandle, err_msg};
use num_traits::Zero;

use super::VMHooksManagedTypes;

pub trait VMHooksCallValue: VMHooksHandlerSource + VMHooksManagedTypes {
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

    fn load_egld_value(&self, dest: RawHandle) {
        let value = self.input_ref().received_egld().clone();
        self.m_types_borrow_mut().bi_overwrite(dest, value.into());
    }

    fn load_all_esdt_transfers(&self, dest_handle: RawHandle) {
        let transfers = self.input_ref().received_esdt();
        self.m_types_borrow_mut()
            .mb_set_vec_of_esdt_payments(dest_handle, transfers);
    }

    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
