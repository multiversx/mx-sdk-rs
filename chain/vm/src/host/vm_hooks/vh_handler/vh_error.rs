use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksError;

use crate::{host::vm_hooks::VMHooksHandlerSource, types::RawHandle};

use super::VMHooksManagedTypes;

pub trait VMHooksSignalError: VMHooksHandlerSource {
    fn signal_error(&mut self, message: &[u8]) -> Result<(), VMHooksError> {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        self.halt_with_error(ReturnCode::UserError, std::str::from_utf8(message).unwrap())
    }
}

pub trait VMHooksErrorManaged: VMHooksManagedTypes + VMHooksSignalError {
    fn signal_error_from_buffer(&mut self, message_handle: RawHandle) -> Result<(), VMHooksError> {
        let bytes = self.m_types_lock().mb_get_owned(message_handle);
        self.signal_error(&bytes)
    }
}
