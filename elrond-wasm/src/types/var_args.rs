use alloc::vec::Vec;
use crate::io::{ArgId, ArgType, DynArgLoader};
use super::SCError;

/// Structure that allows taking a variable number of arguments in a smart contract function.
pub struct VarArgs<T>(pub Vec<T>);

impl<T> From<Vec<T>> for VarArgs<T> {
    fn from(v: Vec<T>) -> Self {
        VarArgs(v)
    }
}

impl<T> VarArgs<T> {
    #[inline]
    pub fn new() -> Self {
        VarArgs(Vec::new())
    }
}

impl<T> Default for VarArgs<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VarArgs<T> {
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }

}

impl<T, D> ArgType<D> for VarArgs<T>
where
    T: ArgType<D>,
    D: DynArgLoader<()>,
{
    #[inline(never)]
    fn load(loader: &mut D, arg_id: ArgId) -> Result<Self, SCError> {
        let mut result_vec: Vec<T> = Vec::new();
        while DynArgLoader::<()>::has_next(&*loader) {
            result_vec.push(T::load(loader, arg_id)?);
        }
        Ok(VarArgs(result_vec))
    }
}
