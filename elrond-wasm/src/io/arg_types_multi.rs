use crate::*;

#[inline]
pub fn load_dyn_multi_arg<T, D, E>(loader: &mut D, err_handler: &E, arg_id: ArgId, num: usize) -> T
where
    T: ArgTypeMulti<D>,
    E: DynArgErrHandler,
{
    match T::load_multi_exact(loader, arg_id, num) {
        Ok(arg) => arg,
        Err(sc_err) => err_handler.handle_sc_error(sc_err),
    }
}

pub trait ArgTypeMulti<D>: ArgType<D> {
    fn load_multi_exact(_loader: &mut D, _arg_name: ArgId, _num: usize) -> Result<Self, SCError>;
}

impl<T, D> ArgTypeMulti<D> for VarArgs<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()>,
{
    fn load_multi_exact(loader: &mut D, arg_id: ArgId, num: usize) -> Result<Self, SCError> {
        let mut result_vec: Vec<T> = Vec::new();
        let mut i = 0usize;
        while DynArgLoader::<()>::has_next(&*loader) && i < num {
            result_vec.push(T::load(loader, arg_id)?);
            i += 1
        }
        if i < num {
            return Err(SCError::Static(err_msg::ARG_WRONG_NUMBER));
        }
        Ok(VarArgs(result_vec))
    }
}
