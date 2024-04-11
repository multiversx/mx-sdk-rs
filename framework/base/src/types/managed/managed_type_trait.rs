use core::marker::PhantomData;
use crate::api::{HandleConstraints, ManagedTypeApi, RawHandle};

use super::ManagedRef;

/// Commonalities between all managed types.
pub trait ManagedType<'a, M: ManagedTypeApi<'a>>: Sized {
    type OwnHandle: HandleConstraints;

    #[doc(hidden)]
    fn from_handle(handle: Self::OwnHandle) -> Self;

    unsafe fn get_handle(&self) -> Self::OwnHandle;

    fn take_handle(self) -> Self::OwnHandle;

    #[doc(hidden)]
    fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_handle(Self::OwnHandle::new(handle))
    }

    unsafe fn get_raw_handle(&self) -> RawHandle {
        self.get_handle().cast_or_signal_error::<'a, M, _>()
    }

    /// Implement carefully, since the underlying transmutation is an unsafe operation.
    /// For types that wrap a handle to some VM-managed data,
    /// make sure the type only contains the handle (plus ZSTs if necessary).
    /// For types that just wrap another managed type it is easier, call for the wrapped object.
    fn transmute_from_handle_ref(handle_ref: &Self::OwnHandle) -> &Self;

    fn as_ref(&self) -> ManagedRef<'_, M, Self> {
        ManagedRef::new(self)
    }
}

pub struct ManagedTypeHandleWrapper<'a, T, M, HC>
where
    T: ManagedType<'a, M, OwnHandle = HC>,
    M: ManagedTypeApi<'a>,
    HC: HandleConstraints
{
    value: &'a T,
    _phantom: PhantomData<M>
}

impl<'a, T, M, HC> ManagedTypeHandleWrapper<'a, T, M, HC>
    where
        T: ManagedType<'a, M, OwnHandle = HC>,
        M: ManagedTypeApi<'a>,
        HC: HandleConstraints
{
    pub fn new(value: &'a T) -> Self {
        Self {
            value,
            _phantom: PhantomData
        }
    }

    pub fn get_handle(self) -> &'a T::OwnHandle {
        unsafe { self.get_handle() }
    }
}
