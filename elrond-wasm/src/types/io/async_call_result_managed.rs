use crate::{
    abi::TypeAbi,
    api::ManagedTypeApi,
    io::{ArgId, DynArg, DynArgInput},
    types::ManagedBuffer,
    ContractCallArg, DynArgOutput,
};
use alloc::string::String;

pub struct ManagedAsyncCallError<M>
where
    M: ManagedTypeApi,
{
    pub err_code: u32,
    pub err_msg: ManagedBuffer<M>,
}

pub enum ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
{
    Ok(T),
    Err(ManagedAsyncCallError<M>),
}

impl<M, T> ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
{
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(self, ManagedAsyncCallResult::Ok(_))
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl<M, T> DynArg for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: DynArg,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let err_code = u32::dyn_load(loader, arg_id);
        if err_code == 0 {
            let arg = T::dyn_load(loader, arg_id);
            ManagedAsyncCallResult::Ok(arg)
        } else {
            let err_msg = if loader.has_next() {
                ManagedBuffer::dyn_load(loader, arg_id)
            } else {
                // error messages should not normally be missing
                // but there was a problem with Arwen in the past,
                // so we are keeping this a little longer, for safety
                ManagedBuffer::new(loader.vm_api_cast())
            };
            ManagedAsyncCallResult::Err(ManagedAsyncCallError { err_code, err_msg })
        }
    }
}

impl<M, T> ContractCallArg for &ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        match self {
            ManagedAsyncCallResult::Ok(result) => {
                0u32.push_dyn_arg(output);
                result.push_dyn_arg(output);
            },
            ManagedAsyncCallResult::Err(error_message) => {
                error_message.err_code.push_dyn_arg(output);
                error_message.err_msg.push_dyn_arg(output);
            },
        }
    }
}

impl<M, T> ContractCallArg for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T> TypeAbi for ManagedAsyncCallResult<M, T>
where
    M: ManagedTypeApi,
    T: TypeAbi,
{
    fn type_name() -> String {
        let mut repr = String::from("AsyncCallResult<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }
}
