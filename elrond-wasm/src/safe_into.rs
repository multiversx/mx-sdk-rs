use core::convert::TryInto;

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    err_msg,
};

pub trait SafeInto<R> {
    fn safe_into<M: ManagedTypeApi>(self) -> R;
}

impl<T, R> SafeInto<R> for T
where
    T: 'static + TryInto<R>,
{
    fn safe_into<M: ManagedTypeApi>(self) -> R {
        self.try_into()
            .unwrap_or_else(|_| M::error_api_impl().signal_error(err_msg::SAFE_INTO_CAST_ERROR))
    }
}
