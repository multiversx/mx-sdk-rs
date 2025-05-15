use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{
    host::vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksHandlerSource},
    types::RawHandle,
};

use super::VMHooksManagedTypes;

pub trait VMHooksSignalError: VMHooksHandlerSource {
    fn signal_error(&mut self, message: &[u8]) -> Result<(), VMHooksEarlyExit> {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        match String::from_utf8(message.to_owned()) {
            Ok(message_string) => {
                Err(VMHooksEarlyExit::new(ReturnCode::UserError.as_u64())
                    .with_message(message_string))
            },
            Err(_) => Err(early_exit_vm_error("error message utf-8 error")),
        }
    }
}

pub trait VMHooksErrorManaged: VMHooksManagedTypes + VMHooksSignalError {
    fn signal_error_from_buffer(
        &mut self,
        message_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let bytes = self.m_types_lock().mb_get_owned(message_handle);
        self.signal_error(&bytes)
    }
}
