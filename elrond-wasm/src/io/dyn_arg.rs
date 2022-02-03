use core::marker::PhantomData;

use crate::{
    api::{ErrorApi, ManagedTypeApi},
    signal_arg_de_error, ArgId, DynArgInput,
};
use elrond_codec::*;

/// Any type that is used as an endpoint argument must implement this trait.
pub trait DynArg: Sized {
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self;
}

/// All top-deserializable types can be endpoint arguments.
impl<T> DynArg for T
where
    T: TopEncode + TopDecode,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        if let TypeInfo::Unit = <T as TopDecode>::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return cast_unit;
        }

        let arg_input = loader.next_arg_input();

        let h = ArgErrorHandler::<I::ManagedTypeErrorApi>::from(arg_id);
        let result = T::top_decode_or_handle_err(arg_input, h);
        let Ok(value) = result;
        value
    }
}

#[derive(Clone)]
pub struct ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<M>,
    pub arg_id: ArgId,
}

impl<M> From<ArgId> for ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    fn from(arg_id: ArgId) -> Self {
        ArgErrorHandler {
            _phantom: PhantomData,
            arg_id,
        }
    }
}

impl<M> DecodeErrorHandler for ArgErrorHandler<M>
where
    M: ManagedTypeApi + ErrorApi,
{
    type HandledErr = !;

    #[inline(always)]
    fn handle_error(&self, err: DecodeError) -> Self::HandledErr {
        signal_arg_de_error::<M>(self.arg_id, err)
    }
}
