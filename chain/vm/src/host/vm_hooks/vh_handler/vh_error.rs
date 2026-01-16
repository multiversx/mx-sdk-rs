use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{host::vm_hooks::VMHooksContext, types::RawHandle};

use super::VMHooksHandler;

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn signal_error(&mut self, message: &[u8]) -> Result<(), VMHooksEarlyExit> {
        let message_string = String::from_utf8_lossy(message);
        self.context.log_error_trace(&message_string);
        Err(VMHooksEarlyExit::new(ReturnCode::UserError.as_u64())
            .with_message(message_string.to_string()))
    }

    pub fn signal_error_from_buffer(
        &mut self,
        message_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let bytes = self.context.m_types_lock().mb_get_owned(message_handle);
        self.signal_error(&bytes)
    }
}
