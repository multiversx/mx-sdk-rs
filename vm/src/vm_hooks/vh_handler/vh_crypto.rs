use crate::{crypto_functions, types::RawHandle, vm_hooks::VMHooksHandlerSource};

pub trait VMHooksCrypto: VMHooksHandlerSource {
    fn sha256_managed(&self, dest: RawHandle, data_handle: RawHandle) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes = crypto_functions::sha256(self.m_types_borrow().mb_get(data_handle));
        self.m_types_borrow_mut()
            .mb_set(dest, result_bytes[..].to_vec());
    }

    fn keccak256_managed(&self, dest: RawHandle, data_handle: RawHandle) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes = crypto_functions::keccak256(self.m_types_borrow().mb_get(data_handle));
        self.m_types_borrow_mut()
            .mb_set(dest, result_bytes[..].to_vec());
    }

    /// Should crash if the signature is invalid.
    fn verify_ed25519_managed(&self, key: RawHandle, message: RawHandle, signature: RawHandle) {
        let sig_valid = crypto_functions::verify_ed25519(
            self.m_types_borrow().mb_get(key),
            self.m_types_borrow().mb_get(message),
            self.m_types_borrow().mb_get(signature),
        );
        if !sig_valid {
            self.vm_error("invalid signature");
        }
    }
}
