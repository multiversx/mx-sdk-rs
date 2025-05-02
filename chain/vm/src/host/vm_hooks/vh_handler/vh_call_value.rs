use crate::{host::vm_hooks::VMHooksHandlerSource, types::RawHandle, vm_err_msg};
use multiversx_chain_vm_executor::VMHooksError;
use num_traits::Zero;

use super::VMHooksManagedTypes;

pub trait VMHooksCallValue: VMHooksHandlerSource + VMHooksManagedTypes {
    fn check_not_payable(&mut self) -> Result<(), VMHooksError> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_call_value)?;

        if self.input_ref().egld_value > num_bigint::BigUint::zero() {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_EGLD)?;
        }
        if self.esdt_num_transfers() > 0 {
            self.vm_error(vm_err_msg::NON_PAYABLE_FUNC_ESDT)?;
        }
        Ok(())
    }

    fn load_egld_value(&mut self, dest: RawHandle) {
        let value = self.input_ref().received_egld().clone();
        self.m_types_lock().bi_overwrite(dest, value.into());
    }

    fn load_all_esdt_transfers(&mut self, dest_handle: RawHandle) {
        let transfers = self.input_ref().received_esdt();
        self.m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, transfers);
    }

    fn esdt_num_transfers(&mut self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
