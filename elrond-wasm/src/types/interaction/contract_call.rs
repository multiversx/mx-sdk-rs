use crate::types::{
    Address, ArgBuffer, AsyncCall, BigUint, BoxedBytes, EsdtTokenPayment, TokenIdentifier,
};
use crate::{
    api::{SendApi, ESDT_NFT_TRANSFER_STRING, ESDT_TRANSFER_STRING},
    BytesArgLoader, DynArg,
};
use crate::{hex_call_data::HexCallDataSerializer, ArgId};
use core::marker::PhantomData;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

/// In case of `transfer_execute`, we leave by default a little gas for the calling transaction to finish.
const TRANSFER_EXECUTE_DEFAULT_LEFTOVER: u64 = 100_000;

/// Represents metadata for calling another contract.
/// Can transform into either an async call, transfer call or other types of calls.
#[must_use]
pub struct ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    api: SA,
    to: Address,
    payment_token: TokenIdentifier,
    payment_amount: BigUint<SA::ProxyTypeManager>,
    payment_nonce: u64,
    endpoint_name: BoxedBytes,
    explicit_gas_limit: u64,
    pub arg_buffer: ArgBuffer, // TODO: make private and find a better way to serialize
    _return_type: PhantomData<R>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractCall::<SA, R>::new`, here types can be inferred from the context.
pub fn new_contract_call<SA, R>(
    api: SA,
    to: Address,
    payment_token: TokenIdentifier,
    payment_amount: BigUint<SA::ProxyTypeManager>,
    payment_nonce: u64,
    endpoint_name: BoxedBytes,
) -> ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    let mut contract_call = ContractCall::<SA, R>::new(api, to, endpoint_name);
    contract_call.payment_token = payment_token;
    contract_call.payment_amount = payment_amount;
    contract_call.payment_nonce = payment_nonce;
    contract_call
}

impl<SA, R> ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    pub fn new(api: SA, to: Address, endpoint_name: BoxedBytes) -> Self {
        let zero = BigUint::zero(api.type_manager());
        ContractCall {
            api,
            to,
            payment_token: TokenIdentifier::egld(),
            payment_amount: zero,
            payment_nonce: 0,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            endpoint_name,
            arg_buffer: ArgBuffer::new(),
            _return_type: PhantomData,
        }
    }

    pub fn with_token_transfer(
        mut self,
        payment_token: TokenIdentifier,
        payment_amount: BigUint<SA::ProxyTypeManager>,
    ) -> Self {
        self.payment_token = payment_token;
        self.payment_amount = payment_amount;
        self
    }

    pub fn with_nft_nonce(mut self, payment_nonce: u64) -> Self {
        self.payment_nonce = payment_nonce;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.explicit_gas_limit = gas_limit;
        self
    }

    pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
        &mut self.arg_buffer
    }

    /// Provided for cases where we build the contract call by hand.
    pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
        self.arg_buffer.push_argument_bytes(bytes);
    }

    /// If this is an ESDT call, it converts it to a regular call to ESDTTransfer.
    /// Async calls require this step, but not `transfer_esdt_execute`.
    fn convert_to_esdt_transfer_call(self) -> Self {
        if self.payment_token.is_egld() {
            self
        } else if self.payment_nonce == 0 {
            // fungible ESDT
            let mut new_arg_buffer = ArgBuffer::new();
            new_arg_buffer.push_argument_bytes(self.payment_token.as_esdt_identifier());
            new_arg_buffer.push_argument_bytes(self.payment_amount.to_bytes_be().as_slice());
            new_arg_buffer.push_argument_bytes(self.endpoint_name.as_slice());

            let zero = BigUint::zero(self.api.type_manager());
            ContractCall {
                api: self.api,
                to: self.to,
                payment_token: TokenIdentifier::egld(),
                payment_amount: zero,
                payment_nonce: 0,
                explicit_gas_limit: self.explicit_gas_limit,
                endpoint_name: BoxedBytes::from(ESDT_TRANSFER_STRING),
                arg_buffer: new_arg_buffer.concat(self.arg_buffer),
                _return_type: PhantomData,
            }
        } else {
            // NFT
            // `ESDTNFTTransfer` takes 4 arguments:
            // arg0 - token identifier
            // arg1 - nonce
            // arg2 - quantity to transfer
            // arg3 - destination address
            let mut new_arg_buffer = ArgBuffer::new();
            new_arg_buffer.push_argument_bytes(self.payment_token.as_esdt_identifier());
            new_arg_buffer.push_argument_bytes(
                elrond_codec::top_encode_no_err(&self.payment_nonce).as_slice(),
            );
            new_arg_buffer.push_argument_bytes(self.payment_amount.to_bytes_be().as_slice());
            new_arg_buffer.push_argument_bytes(self.to.as_bytes());
            new_arg_buffer.push_argument_bytes(self.endpoint_name.as_slice());

            let recipient_addr = Self::nft_transfer_recipient_address(&self.api, self.to);

            let zero = BigUint::zero(self.api.type_manager());
            ContractCall {
                api: self.api,
                to: recipient_addr,
                payment_token: TokenIdentifier::egld(),
                payment_amount: zero,
                payment_nonce: 0,
                explicit_gas_limit: self.explicit_gas_limit,
                endpoint_name: BoxedBytes::from(ESDT_NFT_TRANSFER_STRING),
                arg_buffer: new_arg_buffer.concat(self.arg_buffer),
                _return_type: PhantomData,
            }
        }
    }

    /// nft transfer is sent to self, sender = receiver
    #[cfg(not(feature = "legacy-nft-transfer"))]
    fn nft_transfer_recipient_address(api: &SA, _to: Address) -> Address {
        api.get_sc_address()
    }

    /// legacy nft transfer is sent to the actual intended destination
    #[cfg(feature = "legacy-nft-transfer")]
    fn nft_transfer_recipient_address(_api: &SA, to: Address) -> Address {
        to
    }

    fn resolve_gas_limit(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            self.api.get_gas_left()
        } else {
            self.explicit_gas_limit
        }
    }

    pub fn async_call(mut self) -> AsyncCall<SA> {
        self = self.convert_to_esdt_transfer_call();
        AsyncCall {
            api: self.api,
            to: self.to,
            egld_payment: self.payment_amount,
            hex_data: HexCallDataSerializer::from_arg_buffer(
                self.endpoint_name.as_slice(),
                &self.arg_buffer,
            ),
            callback_data: HexCallDataSerializer::new(&[]),
        }
    }
}

impl<SA, R> ContractCall<SA, R>
where
    SA: SendApi + 'static,
    R: DynArg,
{
    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    pub fn execute_on_dest_context(mut self) -> R {
        self = self.convert_to_esdt_transfer_call();
        let raw_result = self.api.execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.to,
            &self.payment_amount,
            self.endpoint_name.as_slice(),
            &self.arg_buffer,
        );

        let mut loader = BytesArgLoader::new(raw_result.as_slice(), self.api);
        R::dyn_load(&mut loader, ArgId::from(&b"sync result"[..]))
    }

    /// Executes immediately, synchronously, and returns contract call result.
    /// Only works if the target contract is in the same shard.
    /// This is a workaround to handle nested sync calls.
    /// Please do not use this method unless there is absolutely no other option.
    /// Will be eliminated after some future Arwen hook redesign.
    /// `range_closure` takes the number of results before, the number of results after,
    /// and is expected to return the start index (inclusive) and end index (exclusive).
    pub fn execute_on_dest_context_custom_range<F>(mut self, range_closure: F) -> R
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        self = self.convert_to_esdt_transfer_call();
        let raw_result = self.api.execute_on_dest_context_raw_custom_result_range(
            self.resolve_gas_limit(),
            &self.to,
            &self.payment_amount,
            self.endpoint_name.as_slice(),
            &self.arg_buffer,
            range_closure,
        );

        let mut loader = BytesArgLoader::new(raw_result.as_slice(), self.api);
        R::dyn_load(&mut loader, ArgId::from(&b"sync result"[..]))
    }
}

impl<SA, R> ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    /// Executes immediately, synchronously.
    /// The result (if any) is ignored.
    /// Only works if the target contract is in the same shard.
    pub fn execute_on_dest_context_ignore_result(mut self) {
        self = self.convert_to_esdt_transfer_call();
        let _ = self.api.execute_on_dest_context_raw(
            self.resolve_gas_limit(),
            &self.to,
            &self.payment_amount,
            self.endpoint_name.as_slice(),
            &self.arg_buffer,
        );
    }

    fn resolve_gas_limit_with_leftover(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            let mut gas_left = self.api.get_gas_left();
            if gas_left > TRANSFER_EXECUTE_DEFAULT_LEFTOVER {
                gas_left -= TRANSFER_EXECUTE_DEFAULT_LEFTOVER;
            }
            gas_left
        } else {
            self.explicit_gas_limit
        }
    }

    /// Immediately launches a transfer-execute call.
    /// This is similar to an async call, but there is no callback
    /// and there can be more than one such call per transaction.
    pub fn transfer_execute(self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let result = if self.payment_token.is_egld() {
            self.api.direct_egld_execute(
                &self.to,
                &self.payment_amount,
                gas_limit,
                self.endpoint_name.as_slice(),
                &self.arg_buffer,
            )
        } else if self.payment_nonce == 0 {
            // fungible ESDT
            self.api.direct_esdt_execute(
                &self.to,
                &self.payment_token,
                &self.payment_amount,
                gas_limit,
                self.endpoint_name.as_slice(),
                &self.arg_buffer,
            )
        } else {
            // non-fungible/semi-fungible ESDT
            self.api.direct_esdt_nft_execute(
                &self.to,
                &self.payment_token,
                self.payment_nonce,
                &self.payment_amount,
                gas_limit,
                self.endpoint_name.as_slice(),
                &self.arg_buffer,
            )
        };
        if let Err(e) = result {
            self.api.signal_error(e);
        }
    }

    /// Immediately launches a transfer-execute call with multiple transfers in a single call.
    /// This is similar to an async call, but there is no callback
    /// and there can be more than one such call per transaction.
    pub fn esdt_multi_transfer(&self, payments: &[EsdtTokenPayment<SA::ProxyTypeManager>]) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let result = self.api.direct_multi_esdt_transfer_execute(
            &self.to,
            payments,
            gas_limit,
            self.endpoint_name.as_slice(),
            &self.arg_buffer,
        );

        if let Err(e) = result {
            self.api.signal_error(e);
        }
    }
}
