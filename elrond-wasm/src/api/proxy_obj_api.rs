use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::types::{Address, BigUint, TokenIdentifier};

pub trait ProxyObjApi {
    type TypeManager: ManagedTypeApi + 'static;

    /// The code generator produces the same types in the proxy, as for the main contract.
    /// Sometimes endpoints return types that contain a `Self::Storage` type argument,
    /// as for example in `SingleValueMapper<Self::Storage, i32>`.
    /// In order for the proxy code to compile, it is necessary to specify this type here too
    /// (even though it is not required by the trait's methods per se).
    type Storage: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static;

    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    fn new_proxy_obj(api: Self::SendApi) -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    fn contract(self, address: Address) -> Self;

    fn with_token_transfer(
        self,
        token: TokenIdentifier<Self::TypeManager>,
        payment: BigUint<Self::TypeManager>,
    ) -> Self;

    fn with_nft_nonce(self, nonce: u64) -> Self;

    #[allow(clippy::type_complexity)]
    fn into_fields(
        self,
    ) -> (
        Self::SendApi,
        Address,
        TokenIdentifier<Self::TypeManager>,
        BigUint<Self::TypeManager>,
        u64,
    );
}

pub trait CallbackProxyObjApi {
    type TypeManager: ManagedTypeApi + 'static;

    /// The code generator produces the same types in the proxy, as for the main contract.
    /// Sometimes endpoints return types that contain a `Self::Storage` type argument,
    /// as for example in `SingleValueMapper<Self::Storage, i32>`.
    /// In order for the proxy code to compile, it is necessary to specify this type here too
    /// (even though it is not required by the trait's methods per se).
    type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    type ErrorApi: ManagedTypeApi + ErrorApi + Clone + 'static;

    fn new_cb_proxy_obj(api: Self::SendApi) -> Self;

    fn cb_error_api(self) -> Self::ErrorApi;
}
