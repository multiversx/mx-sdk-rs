use crate::{
    host::{
        context::TxTokenTransfer,
        vm_hooks::{VMHooksContext, vh_early_exit::early_exit_vm_error},
    },
    types::RawHandle,
    vm_err_msg,
};
use multiversx_chain_core::EGLD_000000_TOKEN_IDENTIFIER;
use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_traits::Zero;

use super::VMHooksHandler;

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn check_not_payable(&mut self) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_call_value)?;

        if self.context.input_ref().egld_value > num_bigint::BigUint::zero() {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_EGLD));
        }
        if self.esdt_num_transfers() > 0 {
            return Err(early_exit_vm_error(vm_err_msg::NON_PAYABLE_FUNC_ESDT));
        }
        Ok(())
    }

    pub fn load_egld_value(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let value = self.context.input_ref().received_egld().clone();
        self.context.m_types_lock().bi_overwrite(dest, value.into());

        Ok(())
    }

    pub fn load_all_esdt_transfers(
        &mut self,
        dest_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let num_bytes_copied = self
            .context
            .m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, self.context.input_ref().received_esdt());
        self.use_gas_for_data_copy(num_bytes_copied)?;

        Ok(())
    }

    pub fn load_all_transfers(&self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        let direct_egld_value = self.context.input_ref().received_egld().clone();
        let transfers = if !direct_egld_value.is_zero() {
            vec![TxTokenTransfer {
                token_identifier: EGLD_000000_TOKEN_IDENTIFIER.as_bytes().to_vec(),
                nonce: 0,
                value: direct_egld_value,
            }]
        } else {
            self.context.input_ref().received_esdt().to_owned()
        };
        self.context
            .m_types_lock()
            .mb_set_vec_of_esdt_payments(dest_handle, &transfers);
        Ok(())
    }

    pub fn esdt_num_transfers(&mut self) -> usize {
        self.context.input_ref().received_esdt().len()
    }
}
