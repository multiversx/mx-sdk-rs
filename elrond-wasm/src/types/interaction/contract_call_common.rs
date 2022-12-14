use elrond_codec::TopEncodeMulti;

use crate::{api::CallTypeApi, contract_base::ExitCodecErrorHandler, err_msg};

use super::ManagedArgBuffer;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
pub(super) const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

/// In case of `transfer_execute`, we leave by default a little gas for the calling transaction to finish.
pub(super) const TRANSFER_EXECUTE_DEFAULT_LEFTOVER: u64 = 100_000;

pub(super) fn proxy_arg<SA, T>(arg_buffer: &mut ManagedArgBuffer<SA>, endpoint_arg: &T)
where
    SA: CallTypeApi + 'static,
    T: TopEncodeMulti,
{
    let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
    let Ok(()) = endpoint_arg.multi_encode_or_handle_err(arg_buffer, h);
}
