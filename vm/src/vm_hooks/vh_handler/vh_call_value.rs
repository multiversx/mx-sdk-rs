use crate::{types::RawHandle, vm_err_msg, vm_hooks::VMHooksHandlerSource};
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

    fn esdt_num_transfers(&self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
