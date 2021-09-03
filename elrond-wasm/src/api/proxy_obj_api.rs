use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::types::{Address, BigUint, ManagedAddress, TokenIdentifier};

pub trait ProxyObjApi {
    type TypeManager: ManagedTypeApi + 'static;

    /// The code generator produces the same types in the proxy, as for the main contract.
    /// Sometimes endpoints return types that contain a `Self::Storage` type argument,
    /// as for example in `SingleValueMapper<Self::Storage, i32>`.
    /// In order for the proxy code to compile, it is necessary to specify this type here too
    /// (even though it is not required by the trait's methods per se).
    type Storage: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static;

    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    #[doc(hidden)]
    fn new_proxy_obj(api: Self::SendApi) -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    fn contract(self, address: ManagedAddress<Self::TypeManager>) -> Self;

    #[doc(hidden)]
    fn into_fields(self) -> (Self::SendApi, ManagedAddress<Self::TypeManager>);
}

pub trait CallbackProxyObjApi {
    type TypeManager: ManagedTypeApi + ErrorApi + Clone + 'static;

    /// The code generator produces the same types in the proxy, as for the main contract.
    /// Sometimes endpoints return types that contain a `Self::Storage` type argument,
    /// as for example in `SingleValueMapper<Self::Storage, i32>`.
    /// In order for the proxy code to compile, it is necessary to specify this type here too
    /// (even though it is not required by the trait's methods per se).
    type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

    type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    fn new_cb_proxy_obj(api: Self::SendApi) -> Self;

    fn cb_call_api(self) -> Self::TypeManager;
}
