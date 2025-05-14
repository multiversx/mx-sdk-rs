use crate::{
    host::vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksHandlerSource},
    types::RawHandle,
    vm_err_msg,
};
use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_traits::Zero;

use super::VMHooksManagedTypes;

pub trait VMHooksCallValue: VMHooksHandlerSource + VMHooksManagedTypes {
    fn check_not_payable(&mut self) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_call_value)?;

        if self.input_ref().egld_value > num_bigint::BigUint::zero() {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_EGLD));
        }
        if self.esdt_num_transfers() > 0 {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_ESDT));
        }
        Ok(())
    }

    fn load_egld_value(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let value = self.input_ref().received_egld().clone();
        self.m_types_lock().bi_overwrite(dest, value.into());

        Ok(())
    }

    fn load_all_esdt_transfers(&mut self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        let num_bytes_copied = self
            .m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, self.input_ref().received_esdt());
        self.use_gas_for_data_copy(num_bytes_copied)?;

        Ok(())
    }

    fn esdt_num_transfers(&mut self) -> usize {
        self.input_ref().received_esdt().len()
    }
}
