use crate::api::{Handle, PrintApi, PrintApiImpl};

use super::UncallableApi;

impl PrintApi for UncallableApi {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}

impl PrintApiImpl for UncallableApi {
    fn print_biguint(&self, _bu_handle: Handle) {
        unreachable!();
    }
}
