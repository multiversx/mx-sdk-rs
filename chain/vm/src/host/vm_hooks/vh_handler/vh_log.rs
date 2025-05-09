use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{host::context::TxLog, host::vm_hooks::VMHooksHandlerSource, types::RawHandle};

pub trait VMHooksLog: VMHooksHandlerSource {
    fn managed_write_log(
        &mut self,
        topics_handle: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        //TODO: check exact cost
        self.use_gas(
            2 * self
                .gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;
        let topics = self.m_types_lock().mb_get_vec_of_bytes(topics_handle);
        let single_data_field = self.m_types_lock().mb_get(data_handle).to_vec();
        self.push_tx_log(TxLog {
            address: self.current_address().clone(),
            endpoint: self.input_ref().func_name.clone(),
            topics,
            data: vec![single_data_field],
        });
        Ok(())
    }
}
