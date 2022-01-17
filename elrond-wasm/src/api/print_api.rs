use super::{Handle, ManagedTypeApi};

pub trait PrintApi: ManagedTypeApi {
    type PrintApiImpl: PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl {
    fn print_biguint(&self, bu_handle: Handle);
}
