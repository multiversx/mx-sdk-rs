use crate::api::{Handle, ManagedTypeApi, ManagedTypeApiImpl};

/// Commonalities between all managed types.
pub trait ManagedType<M: ManagedTypeApi> {
    #[doc(hidden)]
    fn from_raw_handle(handle: Handle) -> Self;

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle;

    #[inline]
    fn type_manager(&self) -> M::Impl {
        M::instance()
    }
}
