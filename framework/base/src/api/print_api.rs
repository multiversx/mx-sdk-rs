use crate::formatter::FormatBuffer;

use super::ManagedTypeApi;

pub trait PrintApi<'a>: ManagedTypeApi<'a> {
    type PrintApiImpl: PrintApiImpl<'a>;

    fn print_api_impl() -> Self::PrintApiImpl;
}

pub trait PrintApiImpl<'a> {
    /// Buffer used for printing only.
    type Buffer: FormatBuffer<'a>;

    #[inline]
    fn print_buffer(&self, _buffer: Self::Buffer) {}
}
