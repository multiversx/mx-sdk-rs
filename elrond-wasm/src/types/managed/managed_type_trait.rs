use crate::api::{Handle, ManagedTypeApi};

use super::{ManagedReadonly, ManagedRef};

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

    /// Implement carefully, since the underlying transmutation is an unsafe operation.
    /// For types that wrap a handle to some VM-managed data,
    /// make sure the type only contains the handle (plus ZSTs if necessary).
    /// For types that just wrap another managed type it is easier, call for the wrapped object.
    #[doc(hidden)]
    fn transmute_from_handle_ref(handle_ref: &Handle) -> &Self;

    fn into_readonly(self) -> ManagedReadonly<M, Self> {
        ManagedReadonly::from_raw_handle(self.get_raw_handle())
    }

    fn as_ref(&self) -> ManagedRef<'_, M, Self> {
        ManagedRef::new(self)
    }
}
