mod vh_blockchain;
mod vh_call_value;
mod vh_crypto;
mod vh_endpoint_arg;
mod vh_endpoint_finish;
mod vh_error;
mod vh_log;
mod vh_managed_types;
mod vh_send;
mod vh_storage;

pub use vh_blockchain::VMHooksBlockchain;
pub use vh_call_value::VMHooksCallValue;
pub use vh_crypto::VMHooksCrypto;
pub use vh_endpoint_arg::VMHooksEndpointArgument;
pub use vh_endpoint_finish::VMHooksEndpointFinish;
pub use vh_error::{VMHooksError, VMHooksErrorManaged};
pub use vh_log::VMHooksLog;
pub use vh_managed_types::{
    VMHooksBigFloat, VMHooksBigInt, VMHooksManagedBuffer, VMHooksManagedMap, VMHooksManagedTypes,
};
pub use vh_send::VMHooksSend;
pub use vh_storage::{VMHooksStorageRead, VMHooksStorageWrite};

/// Defines all methods that can handle VM hooks. They are spread out over several traits.
pub trait VMHooksHandler:
    VMHooksManagedTypes
    + VMHooksCallValue
    + VMHooksEndpointArgument
    + VMHooksEndpointFinish
    + VMHooksError
    + VMHooksErrorManaged
    + VMHooksStorageRead
    + VMHooksStorageWrite
    + VMHooksCrypto
    + VMHooksBlockchain
    + VMHooksLog
    + VMHooksSend
{
}
