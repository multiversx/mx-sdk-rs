// use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::{
    api::VMApi,
    types::{Address, BigUint, ManagedAddress, TokenIdentifier},
};

pub trait CallbackProxyObjApi {
    type Api: VMApi;

    // type TypeManager: ManagedTypeApi + ErrorApi + Clone + 'static;

    // /// The code generator produces the same types in the proxy, as for the main contract.
    // /// Sometimes endpoints return types that contain a `Self::Api` type argument,
    // /// as for example in `SingleValueMapper<Self::Api, i32>`.
    // /// In order for the proxy code to compile, it is necessary to specify this type here too
    // /// (even though it is not required by the trait's methods per se).
    // type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

    // type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    fn new_cb_proxy_obj(api: Self::Api) -> Self;

    fn cb_call_api(self) -> Self::Api;
}
