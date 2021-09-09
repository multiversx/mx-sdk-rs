// use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::{api::VMApi, types::ManagedAddress};

pub trait ProxyObjApi {
    type Api: VMApi;

    // type TypeManager: ManagedTypeApi + 'static;

    // /// The code generator produces the same types in the proxy, as for the main contract.
    // /// Sometimes endpoints return types that contain a `Self::Api` type argument,
    // /// as for example in `SingleValueMapper<Self::Api, i32>`.
    // /// In order for the proxy code to compile, it is necessary to specify this type here too
    // /// (even though it is not required by the trait's methods per se).
    // type Storage: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static;

    // type SendApi: SendApi<ProxyTypeManager = Self::TypeManager> + Clone + 'static;

    #[doc(hidden)]
    fn new_proxy_obj(api: Self::Api) -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    fn contract(self, address: ManagedAddress<Self::Api>) -> Self;

    #[doc(hidden)]
    fn into_fields(self) -> (Self::Api, ManagedAddress<Self::Api>);
}
