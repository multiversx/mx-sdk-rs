use super::{Handle, ManagedTypeApi};

pub trait PrintApi: ManagedTypeApi {
    type PrintApiImpl: PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl {
    #[inline]
    fn print_biguint(&self, _bu_handle: Handle) {}

    #[inline]
    fn print_managed_buffer(&self, _mb_handle: Handle) {}
}
