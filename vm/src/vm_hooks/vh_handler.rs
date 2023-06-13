mod vh_call_value;
mod vh_endpoint_arg;
mod vh_endpoint_finish;
mod vh_error;
mod vh_managed_types;

pub use vh_call_value::VMHooksCallValue;
pub use vh_endpoint_arg::VMHooksEndpointArgument;
pub use vh_endpoint_finish::VMHooksEndpointFinish;
pub use vh_error::{VMHooksError, VMHooksErrorManaged};
pub use vh_managed_types::{VMHooksBigInt, VMHooksManagedBuffer, VMHooksManagedTypes};

/// Defines all methods that can handle VM hooks. They are spread out over several traits.
pub trait VMHooksHandler:
    VMHooksManagedTypes
    + VMHooksCallValue
    + VMHooksEndpointArgument
    + VMHooksEndpointFinish
    + VMHooksError
    + VMHooksErrorManaged
{
}
