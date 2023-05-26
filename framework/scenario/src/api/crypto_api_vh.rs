use multiversx_sc::api::{uncallable::UncallableApi, CryptoApi};

use super::StaticApi;

impl CryptoApi for StaticApi {
    type CryptoApiImpl = UncallableApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        unreachable!()
    }
}
