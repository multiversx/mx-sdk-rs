use crate::formatter::FormatBuffer;

use super::{Handle, ManagedTypeApi};

pub trait PrintApi: ManagedTypeApi {
    type PrintApiImpl: PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl {
    /// Buffer used for printing only.
    type PrintFormatBuffer: FormatBuffer;

    #[inline]
    fn print_biguint(&self, _bu_handle: Handle) {}

    #[inline]
    fn print_buffer(&self, _buffer: Self::PrintFormatBuffer) {}
}
