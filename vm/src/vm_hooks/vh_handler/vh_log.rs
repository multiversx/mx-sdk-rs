use crate::{tx_mock::TxLog, types::RawHandle, vm_hooks::VMHooksHandlerSource};

pub trait VMHooksLog: VMHooksHandlerSource {
    fn managed_write_log(&self, topics_handle: RawHandle, data_handle: RawHandle) {
        let topics = self.m_types_borrow().mb_get_vec_of_bytes(topics_handle);
        let data = self.m_types_borrow().mb_get(data_handle).to_vec();
        self.push_tx_log(TxLog {
            address: self.input_ref().to.clone(),
            endpoint: self.input_ref().func_name.clone(),
            topics,
            data,
        });
    }
}
