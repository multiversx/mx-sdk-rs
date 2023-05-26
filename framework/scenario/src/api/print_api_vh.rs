use multiversx_sc::api::{uncallable::UncallableApi, PrintApi};

use super::StaticApi;

impl PrintApi for StaticApi {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}
