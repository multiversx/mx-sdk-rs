use core::marker::PhantomData;

use multiversx_sc::{
    api::{HandleConstraints, RawHandle},
    codec::TryStaticCast,
};

#[derive(Clone)]
pub struct StaticApiHandle {
    raw_handle: RawHandle,

    /// This field causes StaticApiHandle not to be `Send` or `Sync`,
    /// which is desirable since the handle is only valid on the thread of the original context.
    _phantom: PhantomData<*const ()>,
}

impl StaticApiHandle {
    /// Should almost never call directly, only used directly in a test.
    pub fn new(raw_handle: RawHandle) -> Self {
        Self {
            raw_handle,
            _phantom: PhantomData,
        }
    }
}

impl core::fmt::Debug for StaticApiHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RawHandle::fmt(&self.raw_handle, f)
    }
}

impl HandleConstraints for StaticApiHandle {
    fn new(handle: multiversx_sc::api::RawHandle) -> Self {
        StaticApiHandle::new(handle)
    }

    fn to_be_bytes(&self) -> [u8; 4] {
        self.raw_handle.to_be_bytes()
    }

    fn get_raw_handle(&self) -> RawHandle {
        self.raw_handle
    }

    fn get_raw_handle_unchecked(&self) -> RawHandle {
        self.raw_handle
    }
}

impl PartialEq<RawHandle> for StaticApiHandle {
    fn eq(&self, other: &RawHandle) -> bool {
        &self.raw_handle == other
    }
}

impl PartialEq<StaticApiHandle> for StaticApiHandle {
    fn eq(&self, other: &StaticApiHandle) -> bool {
        self.raw_handle == other.raw_handle
    }
}

impl From<i32> for StaticApiHandle {
    fn from(handle: i32) -> Self {
        StaticApiHandle::new(handle)
    }
}

impl TryStaticCast for StaticApiHandle {}

#[cfg(test)]
mod tests {
    use super::StaticApiHandle;

    // StaticApiHandle intentionally does not implement Send or Sync
    // (enforced via PhantomData<*const ()>), since a handle is only valid
    // on the thread that created the underlying context.
    static_assertions::assert_not_impl_any!(StaticApiHandle: Send, Sync);
}
