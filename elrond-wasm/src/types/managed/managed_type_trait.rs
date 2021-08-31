use crate::api::{Handle, ManagedTypeApi};

/// Commonalities between all managed types.
pub trait ManagedType<M: ManagedTypeApi> {
    #[doc(hidden)]
    fn from_raw_handle(api: M, raw_handle: Handle) -> Self;

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle;

    fn type_manager(&self) -> M;
}
