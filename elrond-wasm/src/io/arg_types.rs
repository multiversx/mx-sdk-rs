use crate::*;
use elrond_codec::*;

/// Some info to display in endpoint argument deserialization error messages,
/// to help users identify the faulty argument.
/// Generated automatically.
/// Current version uses argument names,
/// but in principle it could be changed to argument index to save some bytes from the wasm output.
pub type ArgId = &'static [u8];

pub trait DynArgLoader<T>: Sized {
    fn has_next(&self) -> bool;

    fn next_arg(&mut self, arg_id: ArgId) -> Result<Option<T>, SCError>;
}

pub trait ArgType<D>: Sized {
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError>;
}

#[inline]
pub fn load_dyn_arg<T, D, E>(loader: &mut D, err_handler: &E, arg_id: ArgId) -> T
where
    T: ArgType<D>,
    E: DynArgErrHandler,
{
    match T::load(loader, arg_id) {
        Ok(arg) => arg,
        Err(sc_err) => err_handler.handle_sc_error(sc_err),
    }
}

#[inline]
pub fn check_no_more_args<D, E>(loader: &D, err_handler: &E)
where
    D: DynArgLoader<()>,
    E: DynArgErrHandler,
{
    if D::has_next(loader) {
        err_handler.handle_sc_error(SCError::from(err_msg::ARG_WRONG_NUMBER));
    }
}

impl<T, D> ArgType<D> for T
where
    T: NestedDecode,
    D: DynArgLoader<T>,
{
    #[inline(never)]
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        if let TypeInfo::Unit = T::TYPE_INFO {
            // unit type returns without loading anything
            let cast_unit: T = unsafe { core::mem::transmute_copy(&()) };
            return Ok(cast_unit);
        }

        match loader.next_arg(arg_id) {
            Ok(Some(arg)) => Ok(arg),
            Ok(None) => Err(SCError::from(err_msg::ARG_WRONG_NUMBER)),
            Err(sc_err) => Err(sc_err),
        }
    }
}




