use multiversx_sc::api::{uncallable::UncallableApi, CryptoApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> CryptoApi for VMHooksApi<BACKEND_TYPE> {
    type CryptoApiImpl = UncallableApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        unreachable!()
    }
}
