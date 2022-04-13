use crate::{
    api::{PrintApi, PrintApiImpl},
    formatter::FormatBufferIgnore,
};

use super::UncallableApi;

impl PrintApi for UncallableApi {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}

impl PrintApiImpl for UncallableApi {
    type Buffer = FormatBufferIgnore;
}
