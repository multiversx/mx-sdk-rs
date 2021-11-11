use crate::{api::StaticBufferApi, types::LockableStaticBuffer};

impl StaticBufferApi for super::UncallableApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(
        &self,
        _f: F,
    ) -> R {
        unreachable!()
    }
}
