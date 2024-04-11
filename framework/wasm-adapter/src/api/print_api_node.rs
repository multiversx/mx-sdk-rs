use crate::api::VmApiImpl;
use multiversx_sc::{
    api::{PrintApi, PrintApiImpl},
    formatter::FormatBufferIgnore,
};

impl PrintApi for VmApiImpl {
    type PrintApiImpl = VmApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        VmApiImpl {}
    }
}

impl PrintApiImpl for VmApiImpl {
    type Buffer = FormatBufferIgnore;
}
