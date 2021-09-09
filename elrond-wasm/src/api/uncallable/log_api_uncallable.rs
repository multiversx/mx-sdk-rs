use crate::{
    api::{Handle, LogApi},
    types::ArgBuffer,
};

use super::UncallableApi;

impl LogApi for UncallableApi {
    fn write_event_log(&self, topics_buffer: &ArgBuffer, data: &[u8]) {
        unreachable!()
    }

    fn write_legacy_log(&self, topics: &[[u8; 32]], data: &[u8]) {
        unreachable!()
    }

    fn managed_write_log(&self, topics_handle: Handle, data_handle: Handle) {
        unreachable!()
    }
}
