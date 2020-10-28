use alloc::vec::Vec;
use crate::io::{ArgId, DynArg, DynArgInput};
use elrond_codec::TopDecodeInput;

pub struct AsyncCallError {
    pub err_code: i32,
    pub err_msg: Vec<u8>,
}

pub enum AsyncCallResult<T> {
    Ok(T),
    Err(AsyncCallError)
}

impl<I, D, T> DynArg<I, D> for AsyncCallResult<T>
where
    I: TopDecodeInput,
    D: DynArgInput<I>,
    T: DynArg<I, D>,
{
    fn dyn_load(loader: &mut D, arg_id: ArgId) -> Self {
        let err_code = i32::dyn_load(loader, arg_id);
        if err_code == 0 {
            let arg = T::dyn_load(loader, arg_id);
            AsyncCallResult::Ok(arg)
        } else {
            let err_msg = Vec::<u8>::dyn_load(loader, arg_id);
            AsyncCallResult::Err(AsyncCallError {
                err_code,
                err_msg,
            })
        }
    }
}
