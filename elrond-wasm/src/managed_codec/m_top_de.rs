use elrond_codec::{DecodeError, TopDecode, TypeInfo};

use crate::api::ManagedTypeApi;

use super::{TopDecodeInputAdapter, m_top_de_input::ManagedTopDecodeInput};

pub trait ManagedTopDecode<M: ManagedTypeApi>: Sized {
    fn top_decode_or_exit<I: ManagedTopDecodeInput<M>, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self;

    fn is_unit() -> bool {
        false
    }
}

impl<M, T> ManagedTopDecode<M> for T
where
    M: ManagedTypeApi,
    T: TopDecode,
{
    fn top_decode_or_exit<I: ManagedTopDecodeInput<M>, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        <T as TopDecode>::top_decode_or_exit(TopDecodeInputAdapter::new(input), c, exit)
    }

    fn is_unit() -> bool {
        matches!(<T as TopDecode>::TYPE_INFO, TypeInfo::Unit)
    }
}
