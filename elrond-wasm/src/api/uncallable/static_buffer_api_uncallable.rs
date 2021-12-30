use crate::{
    api::{StaticBufferApi, StaticBufferApiImpl},
    types::LockableStaticBuffer,
};

use super::UncallableApi;

impl StaticBufferApi for UncallableApi {
    type StaticBufferApiImpl = UncallableApi;

    fn static_buffer_api_impl() -> Self::StaticBufferApiImpl {
        unreachable!()
    }
}

impl StaticBufferApiImpl for UncallableApi {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(
        &self,
        _f: F,
    ) -> R {
        unreachable!()
    }
}
