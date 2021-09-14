// use super::{ErrorApi, ManagedTypeApi, SendApi, StorageReadApi, StorageWriteApi};
use crate::{api::VMApi, types::ManagedAddress};

pub trait ProxyObjBase {
    type Api: VMApi;

    #[doc(hidden)]
    fn new_proxy_obj(api: Self::Api) -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    fn contract(self, address: ManagedAddress<Self::Api>) -> Self;

    #[doc(hidden)]
    fn into_fields(self) -> (Self::Api, ManagedAddress<Self::Api>);
}
