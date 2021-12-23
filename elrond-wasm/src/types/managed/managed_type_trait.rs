use crate::api::{Handle, ManagedTypeApi};

use super::ManagedReadonly;

/// Commonalities between all managed types.
pub trait ManagedType<M: ManagedTypeApi>: Sized {
    #[doc(hidden)]
    fn from_raw_handle(handle: Handle) -> Self;

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle;

    #[inline]
    fn type_manager(&self) -> M {
        M::instance()
    }

    #[doc(hidden)]
    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self;

    fn into_readonly(self) -> ManagedReadonly<M, Self> {
        ManagedReadonly::from_raw_handle(self.get_raw_handle())
    }
}
