use core::convert::TryInto;

use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    err_msg,
};

pub(crate) fn cast_to_i64<'a, M, T>(value: T) -> i64
where
    M: ManagedTypeApi<'a>,
    T: TryInto<i64>,
{
    value
        .try_into()
        .unwrap_or_else(|_| M::error_api_impl().signal_error(err_msg::CAST_TO_I64_ERROR))
}
