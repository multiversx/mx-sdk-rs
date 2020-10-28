use crate::*;
use elrond_codec::*;

/// Any type that is used as an endpoint argument must implement this trait.
pub trait DynArg<I, D>: Sized
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
{
    fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self;
}

/// Used for loading arguments annotated with `#[multi(...)]`.
pub trait DynArgMulti<I, D>: DynArg<I, D>
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
{
    fn dyn_load_multi(loader: &mut D, arg_id: ArgId, num: usize) -> Self;
}

/// All top-deserializable types can be endpoint arguments.
impl<I, D, T> DynArg<I, D> for T
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
    T: TopDecode,
{
    #[inline]
    fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self {
        if let TypeInfo::Unit = T::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return cast_unit;
        }

        if let Some(arg_input) = loader.next_arg_input() {
            T::top_decode(arg_input, |res| match res {
                Ok(v) => v,
                Err(de_err) => loader.signal_arg_de_error(arg_id, de_err),
            })
        } else {
            loader.signal_arg_wrong_number()
        }
    }
}
