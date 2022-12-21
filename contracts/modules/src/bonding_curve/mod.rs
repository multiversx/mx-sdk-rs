mx_sc::imports!();
mx_sc::derive_imports!();

pub mod curves;
pub mod utils;
use utils::{events, owner_endpoints, storage, user_endpoints};

#[mx_sc::module]
pub trait BondingCurveModule:
    storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
}
