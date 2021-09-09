use crate::{
    api::{Handle, LogApi},
    types::ArgBuffer,
};

use super::UncallableApi;

impl LogApi for UncallableApi {
    fn write_event_log(&self, _topics_buffer: &ArgBuffer, _data: &[u8]) {
        unreachable!()
    }

    fn write_legacy_log(&self, _topics: &[[u8; 32]], _data: &[u8]) {
        unreachable!()
    }

    fn managed_write_log(&self, _topics_handle: Handle, _data_handle: Handle) {
        unreachable!()
    }
}
