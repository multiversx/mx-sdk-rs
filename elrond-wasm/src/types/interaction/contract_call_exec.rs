use elrond_codec::TopDecodeMulti;

use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::SendRawWrapper,
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
    types::{ManagedBuffer, ManagedVec},
};

use super::{
    contract_call_common::{TRANSFER_EXECUTE_DEFAULT_LEFTOVER, UNSPECIFIED_GAS_LIMIT},
    AsyncCall, AsyncCallPromises, ContractCallFull,
};

impl<SA, OriginalResult> ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn resolve_gas_limit(&self) -> u64 {
        if self.basic.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            SA::blockchain_api_impl().get_gas_left()
        } else {
            self.basic.explicit_gas_limit
        }
    }

    pub(super) fn async_call(self) -> AsyncCall<SA> {
        AsyncCall {
            to: self.basic.to,
            egld_payment: self.egld_payment,
            endpoint_name: self.basic.endpoint_name,
            arg_buffer: self.basic.arg_buffer,
            callback_call: None,
        }
    }

    pub(super) fn async_call_promise(self) -> AsyncCallPromises<SA> {
        AsyncCallPromises {
            to: self.basic.to,
            egld_payment: self.egld_payment,
            endpoint_name: self.basic.endpoint_name,
            arg_buffer: self.basic.arg_buffer,
            explicit_gas_limit: self.basic.explicit_gas_limit,
            extra_gas_for_callback: 0,
            callback_call: None,
        }
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    pub(super) fn execute_on_dest_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.egld_payment,
            &self.basic.endpoint_name,
            &self.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_dest_context_readonly<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_dest_context_readonly_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.basic.endpoint_name,
            &self.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn execute_on_same_context<RequestedResult>(self) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let raw_result = SendRawWrapper::<SA>::new().execute_on_same_context_raw(
            self.resolve_gas_limit(),
            &self.basic.to,
            &self.egld_payment,
            &self.basic.endpoint_name,
            &self.basic.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        decode_result(raw_result)
    }

    pub(super) fn resolve_gas_limit_with_leftover(&self) -> u64 {
        if self.basic.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            let mut gas_left = SA::blockchain_api_impl().get_gas_left();
            if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
                gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
            }
            gas_left
        } else {
            self.basic.explicit_gas_limit
        }
    }

    pub(super) fn no_payment_transfer_execute(&self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();

        let _ = SendRawWrapper::<SA>::new().direct_egld_execute(
            &self.basic.to,
            &self.egld_payment,
            gas_limit,
            &self.basic.endpoint_name,
            &self.basic.arg_buffer,
        );
    }

    pub(super) fn single_transfer_execute(self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let payment = &self.payments.try_get(0).unwrap();

        if self.egld_payment > 0 {
            let _ = SendRawWrapper::<SA>::new().direct_egld_execute(
                &self.basic.to,
                &self.egld_payment,
                gas_limit,
                &self.basic.endpoint_name,
                &self.basic.arg_buffer,
            );
        } else if payment.token_nonce == 0 {
            // fungible ESDT
            let _ = SendRawWrapper::<SA>::new().transfer_esdt_execute(
                &self.basic.to,
                &payment.token_identifier,
                &payment.amount,
                gas_limit,
                &self.basic.endpoint_name,
                &self.basic.arg_buffer,
            );
        } else {
            // non-fungible/semi-fungible ESDT
            let _ = SendRawWrapper::<SA>::new().transfer_esdt_nft_execute(
                &self.basic.to,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
                gas_limit,
                &self.basic.endpoint_name,
                &self.basic.arg_buffer,
            );
        }
    }

    pub(super) fn multi_transfer_execute(self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let _ = SendRawWrapper::<SA>::new().multi_esdt_transfer_execute(
            &self.basic.to,
            &self.payments,
            gas_limit,
            &self.basic.endpoint_name,
            &self.basic.arg_buffer,
        );
    }
}

fn decode_result<SA, RequestedResult>(
    raw_result: ManagedVec<SA, ManagedBuffer<SA>>,
) -> RequestedResult
where
    SA: CallTypeApi + 'static,
    RequestedResult: TopDecodeMulti,
{
    let mut loader = ManagedResultArgLoader::new(raw_result);
    let arg_id = ArgId::from(&b"sync result"[..]);
    let h = ArgErrorHandler::<SA>::from(arg_id);
    let Ok(result) = RequestedResult::multi_decode_or_handle_err(&mut loader, h);
    result
}
