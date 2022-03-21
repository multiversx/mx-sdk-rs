use crate::formatter::FormatBuffer;

use super::ManagedTypeApi;

pub trait PrintApi: ManagedTypeApi {
    type PrintApiImpl: PrintApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl {
    /// Buffer used for printing only.
    type Buffer: FormatBuffer;

    #[inline]
    fn print_buffer(&self, _buffer: Self::Buffer) {}
}
