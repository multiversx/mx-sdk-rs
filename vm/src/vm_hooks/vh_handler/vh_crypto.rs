use crate::{crypto_functions, types::RawHandle, vm_hooks::VMHooksHandlerSource};

pub trait VMHooksCrypto: VMHooksHandlerSource {
    fn sha256_managed(&self, dest: RawHandle, data_handle: RawHandle) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::sha256(data);
        types.mb_set(dest, result_bytes[..].to_vec());
    }

    fn keccak256_managed(&self, dest: RawHandle, data_handle: RawHandle) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::keccak256(data);
        types.mb_set(dest, result_bytes[..].to_vec());
    }

    /// Should crash if the signature is invalid.
    fn verify_ed25519_managed(&self, key: RawHandle, message: RawHandle, signature: RawHandle) {
        let types = self.m_types_lock();
        let key = types.mb_get(key);
        let message = types.mb_get(message);
        let signature = types.mb_get(signature);
        let sig_valid = crypto_functions::verify_ed25519(key, message, signature);
        if !sig_valid {
            self.vm_error("invalid signature");
        }
    }
}
