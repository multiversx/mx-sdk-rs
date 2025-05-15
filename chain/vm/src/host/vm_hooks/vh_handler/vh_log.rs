use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{host::context::TxLog, host::vm_hooks::VMHooksHandlerSource, types::RawHandle};

pub trait VMHooksLog: VMHooksHandlerSource {
    fn managed_write_log(
        &mut self,
        topics_handle: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.log)?;

        let (topics, topic_bytes_copied) = self.m_types_lock().mb_get_vec_of_bytes(topics_handle);
        let single_data_field = self.m_types_lock().mb_get(data_handle).to_vec();

        self.use_gas_for_data_copy(topic_bytes_copied + single_data_field.len())?;

        self.push_tx_log(TxLog {
            address: self.current_address().clone(),
            endpoint: self.input_ref().func_name.clone(),
            topics,
            data: vec![single_data_field],
        });
        Ok(())
    }
}
