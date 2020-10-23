use crate::io::{ArgId, ArgType, DynArgLoader};
use super::SCError;

/// A smart contract argument that can be provided or not.
/// If arguments stop before this argument, None will be returned.
pub enum OptionalArg<T> {
    Some(T),
    None
}

impl<T> From<Option<T>> for OptionalArg<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(arg) => OptionalArg::Some(arg),
            None => OptionalArg::None
        }
    }
}

impl<T> OptionalArg<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            OptionalArg::Some(arg) => Some(arg),
            OptionalArg::None => None
        }
    }
}

impl<T, D> ArgType<D> for OptionalArg<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()>,
{
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        if DynArgLoader::<()>::has_next(&*loader) {
            Ok(OptionalArg::Some(T::load(loader, arg_id)?))
        } else {
            Ok(OptionalArg::None)
        }
    }
}
