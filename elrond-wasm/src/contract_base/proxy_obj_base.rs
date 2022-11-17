use crate::{
    api::VMApi,
    types::{ManagedAddress, ManagedOption},
};

pub trait ProxyObjBase {
    type Api: VMApi;

    #[doc(hidden)]
    fn new_proxy_obj() -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    #[must_use]
    fn contract(self, address: ManagedAddress<Self::Api>) -> Self;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will just return `ManagedOption::none()` if no address was specified.
    #[doc(hidden)]
    fn extract_opt_address(&mut self) -> ManagedOption<Self::Api, ManagedAddress<Self::Api>>;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will crash if no address was specified.
    #[doc(hidden)]
    fn extract_address(&mut self) -> ManagedAddress<Self::Api>;
}
