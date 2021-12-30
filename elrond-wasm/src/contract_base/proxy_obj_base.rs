use crate::{api::VMApi, types::ManagedAddress};

pub trait ProxyObjBase {
    type Api: VMApi;

    #[doc(hidden)]
    fn new_proxy_obj() -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    #[must_use]
    fn contract(self, address: ManagedAddress<Self::Api>) -> Self;

    #[doc(hidden)]
    fn into_fields(self) -> ManagedAddress<Self::Api>;
}
