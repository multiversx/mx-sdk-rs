use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{
    crypto_functions,
    host::vm_hooks::{vh_early_exit::early_exit_vm_error, VMHooksContext},
    types::RawHandle,
    vm_err_msg,
};

use super::VMHooksHandler;

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn sha256_managed(
        &mut self,
        dest: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().crypto_api_cost.sha_256)?;

        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.context.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::sha256(data);
        types.mb_set(dest, result_bytes[..].to_vec());

        Ok(())
    }

    pub fn keccak256_managed(
        &mut self,
        dest: RawHandle,
        data_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().crypto_api_cost.keccak_256)?;

        // default implementation used in debugger
        // the VM has a dedicated hook
        let mut types = self.context.m_types_lock();
        let data = types.mb_get(data_handle);
        let result_bytes = crypto_functions::keccak256(data);
        types.mb_set(dest, result_bytes[..].to_vec());

        Ok(())
    }

    /// Should crash if the signature is invalid.
    pub fn verify_ed25519_managed(
        &mut self,
        key: RawHandle,
        message: RawHandle,
        signature: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let sig_valid = {
            self.use_gas(self.gas_schedule().crypto_api_cost.verify_ed_25519)?;

            let types = self.context.m_types_lock();
            let key = types.mb_get(key);
            let message = types.mb_get(message);
            let signature = types.mb_get(signature);
            crypto_functions::verify_ed25519(key, message, signature)
        };
        if !sig_valid {
            return Err(early_exit_vm_error(vm_err_msg::CRYPTO_ED25519_ERROR));
        }

        Ok(())
    }

    pub fn verify_bls_managed(
        &self,
        key_handle: RawHandle,
        message_handle: RawHandle,
        signature_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let types = self.context.m_types_lock();
        let key = types.mb_get(key_handle);
        let message = types.mb_get(message_handle);
        let signature = types.mb_get(signature_handle);
        let sig_valid = crypto_functions::verify_bls(key, message, signature);
        if !sig_valid {
            // self.vm_error("invalid signature");

            return Err(early_exit_vm_error("invalid signature"));
        }

        Ok(())
    }

    pub fn verify_bls_aggregated_signature(
        &self,
        key_handle: RawHandle,
        message_handle: RawHandle,
        signature_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        let types = self.context.m_types_lock();
        let (keys, _) = types.mb_get_vec_of_bytes(key_handle);
        let message = types.mb_get(message_handle);
        let signature = types.mb_get(signature_handle);
        let sig_valid = crypto_functions::verify_bls_aggregated_signature(keys, message, signature);
        if !sig_valid {
            // self.vm_error("invalid signature");

            return Err(early_exit_vm_error("invalid signature"));
        }

        Ok(())
    }
}
