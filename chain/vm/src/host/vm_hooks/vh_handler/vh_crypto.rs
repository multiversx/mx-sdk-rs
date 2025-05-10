use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{
    crypto_functions,
    host::vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksHandlerSource},
    types::RawHandle,
    vm_err_msg,
};

pub trait VMHooksCrypto: VMHooksHandlerSource {
    fn sha256_managed(
        &mut self,
        dest: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().crypto_api_cost.sha_256)?;

        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::sha256(data);
        types.mb_set(dest, result_bytes[..].to_vec());

        Ok(())
    }

    fn keccak256_managed(
        &mut self,
        dest: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().crypto_api_cost.keccak_256)?;

        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::keccak256(data);
        types.mb_set(dest, result_bytes[..].to_vec());

        Ok(())
    }

    /// Should crash if the signature is invalid.
    fn verify_ed25519_managed(
        &mut self,
        key: RawHandle,
        message: RawHandle,
        signature: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let sig_valid = {
            self.use_gas(self.gas_schedule().crypto_api_cost.verify_ed_25519)?;

            let types = self.m_types_lock();
            let key = types.mb_get(key);
            let message = types.mb_get(message);
            let signature = types.mb_get(signature);
            crypto_functions::verify_ed25519(key, message, signature)
        };
        if !sig_valid {
            return Err(early_exit_vm_error(vm_err_msg::CRYPTO_INVALID_SIGNATURE));
        }

        Ok(())
    }
}
