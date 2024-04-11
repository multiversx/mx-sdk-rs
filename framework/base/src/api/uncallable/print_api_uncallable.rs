use crate::{
    api::{PrintApi, PrintApiImpl},
    formatter::FormatBufferIgnore,
};

use super::UncallableApi;

impl<'a> PrintApi<'a> for UncallableApi {
    type PrintApiImpl = UncallableApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        unreachable!()
    }
}

impl<'a> PrintApiImpl<'a> for UncallableApi {
    type Buffer = FormatBufferIgnore;
}
