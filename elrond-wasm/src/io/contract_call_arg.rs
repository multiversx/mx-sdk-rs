use crate::DynArgOutput;
use elrond_codec::TopEncode;

/// Trait that specifies how arguments are serialized in contract calls.
pub trait ContractCallArg: Sized {
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O);
}

impl<T> ContractCallArg for T
where
    T: TopEncode,
{
    #[inline]
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        output.push_single_arg(self);
    }
}
