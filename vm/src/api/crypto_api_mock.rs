use crate::{crypto_functions, tx_mock::TxPanic, DebugApi};
use multiversx_sc::{
    api::{CryptoApi, CryptoApiImpl, ManagedBufferApiImpl},
    types::MessageHashType,
};

impl CryptoApi for DebugApi {
    type CryptoApiImpl = DebugApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        DebugApi::new_from_static()
    }
}

impl CryptoApiImpl for DebugApi {
    fn sha256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes = crypto_functions::sha256(self.mb_to_boxed_bytes(data_handle).as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
    }

    fn keccak256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes =
            crypto_functions::keccak256(self.mb_to_boxed_bytes(data_handle).as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
    }

    fn ripemd160_managed(
        &self,
        _dest: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ripemd160 not implemented yet!")
    }

    fn verify_bls_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_bls not implemented yet!")
    }

    fn verify_ed25519_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) {
        let sig_valid = crypto_functions::verify_ed25519(
            self.mb_to_boxed_bytes(key).as_slice(),
            self.mb_to_boxed_bytes(message).as_slice(),
            self.mb_to_boxed_bytes(signature).as_slice(),
        );
        if !sig_valid {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: "invalid signature".to_string(),
            });
        }
    }

    fn verify_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_secp256k1 not implemented yet!")
    }

    fn verify_custom_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
        _hash_type: MessageHashType,
    ) -> bool {
        panic!("verify_custom_secp256k1 not implemented yet!")
    }

    fn encode_secp256k1_der_signature_managed(
        &self,
        _r: Self::ManagedBufferHandle,
        _s: Self::ManagedBufferHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        panic!("encode_secp256k1_signature not implemented yet!")
    }
}
