use crate::{
    types::{ManagedAddress, ManagedOption},
};
use crate::api::VMApi;

pub trait ProxyObjBase<A: VMApi> {
    #[doc(hidden)]
    fn new_proxy_obj() -> Self;

    /// Specify the target contract to call.
    /// Not taken into account for deploys.
    #[must_use]
    fn contract(self, address: ManagedAddress<A>) -> Self;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will just return `ManagedOption::none()` if no address was specified.
    #[doc(hidden)]
    fn extract_opt_address(&mut self) -> ManagedOption<A, ManagedAddress<A>>;

    /// Extracts the address contained in the proxy object and replaces it with None.
    ///
    /// Will crash if no address was specified.
    #[doc(hidden)]
    fn extract_address(&mut self) -> ManagedAddress<A>;
}
