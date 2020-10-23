use alloc::vec::Vec;
use crate::io::{ArgId, ArgType, DynArgLoader};
use super::SCError;

pub struct AsyncCallError {
    pub err_code: i32,
    pub err_msg: Vec<u8>,
}

pub enum AsyncCallResult<T> {
    Ok(T),
    Err(AsyncCallError)
}

impl<T, D> ArgType<D> for AsyncCallResult<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()> + DynArgLoader<i32> + DynArgLoader<Vec<u8>>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        let err_code = i32::load(loader, arg_id)?;
        if err_code == 0 {
            let arg = T::load(loader, arg_id)?;
            Ok(AsyncCallResult::Ok(arg))
        } else {
            let err_msg = Vec::<u8>::load(loader, arg_id)?;
            Ok(AsyncCallResult::Err(AsyncCallError {
                err_code,
                err_msg,
            }))
        }
    }
}
