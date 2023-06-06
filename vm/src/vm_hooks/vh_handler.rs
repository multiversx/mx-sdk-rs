mod vh_big_int;
mod vh_error;
mod vh_managed_buffer;
mod vh_managed_types;

pub use vh_big_int::VMHooksBigInt;
pub use vh_error::VMHooksError;
pub use vh_managed_buffer::VMHooksManagedBuffer;
pub use vh_managed_types::VMHooksManagedTypes;

/// Defines all methods that can handle VM hooks. They are spread out over several traits.
pub trait VMHooksHandler: VMHooksManagedTypes {}
