use crate::api::VmApiImpl;
use multiversx_sc::{
    api::{PrintApi, PrintApiImpl},
    formatter::FormatBufferIgnore,
};

impl<'a> PrintApi<'a> for VmApiImpl {
    type PrintApiImpl = VmApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        VmApiImpl {}
    }
}

impl<'a> PrintApiImpl<'a> for VmApiImpl {
    type Buffer = FormatBufferIgnore;
}
