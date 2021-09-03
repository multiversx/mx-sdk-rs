use crate::{
    api::{
        BlockchainApi, ErrorApi, SendApi, ESDT_MULTI_TRANSFER_STRING, ESDT_NFT_TRANSFER_STRING,
        ESDT_TRANSFER_STRING,
    },
    hex_call_data::HexCallDataSerializer,
    types::{
        Address, ArgBuffer, AsyncCall, BigUint, BoxedBytes, EsdtTokenPayment, ManagedAddress,
        ManagedArgBuffer, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
    ArgId, BytesArgLoader, ContractCallArg, DynArg, ManagedResultArgLoader,
};
use alloc::vec;
use alloc::vec::Vec;
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
    to: ManagedAddress<SA::ProxyTypeManager>,
    egld_payment: BigUint<SA::ProxyTypeManager>,
    payments: ManagedVec<SA::ProxyTypeManager, EsdtTokenPayment<SA::ProxyTypeManager>>,
    endpoint_name: ManagedBuffer<SA::ProxyTypeManager>,
    explicit_gas_limit: u64,
    pub arg_buffer: ManagedArgBuffer<SA::ProxyTypeManager>, // TODO: make private?
    _return_type: PhantomData<R>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractCall::<SA, R>::new`, here types can be inferred from the context.
pub fn new_contract_call<SA, R>(
    api: SA,
    to: ManagedAddress<SA::ProxyTypeManager>,
    endpoint_name_slice: &'static [u8],
) -> ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    let endpoint_name = ManagedBuffer::new_from_bytes(api.type_manager(), endpoint_name_slice);
    ContractCall::<SA, R>::new(api, to, endpoint_name)
}

impl<SA, R> ContractCall<SA, R>
where
    SA: SendApi + 'static,
{
    pub fn new(
        api: SA,
        to: ManagedAddress<SA::ProxyTypeManager>,
        endpoint_name: ManagedBuffer<SA::ProxyTypeManager>,
    ) -> Self {
        let payments = vec![EsdtTokenPayment::no_payment(api.type_manager())];
        let arg_buffer = ManagedArgBuffer::new_empty(api.type_manager());
        let egld_payment = BigUint::zero(api.type_manager());
        let payments = ManagedVec::new_empty(api.type_manager());
        ContractCall {
            api,
            to,
            egld_payment,
            payments,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            endpoint_name,
            arg_buffer,
            _return_type: PhantomData,
        }
    }

    pub fn add_token_transfer(
        mut self,
        payment_token: TokenIdentifier<SA::ProxyTypeManager>,
        payment_nonce: u64,
        payment_amount: BigUint<SA::ProxyTypeManager>,
    ) -> Self {
        self.payments.push(EsdtTokenPayment::from(
            payment_token,
            payment_nonce,
            payment_amount,
        ));
        self
    }

    pub fn with_egld_transfer(mut self, egld_amount: BigUint<SA::ProxyTypeManager>) -> Self {
        self.payments
            .overwrite_with_single_item(EsdtTokenPayment::from(
                TokenIdentifier::egld(self.api.type_manager()),
                0,
                egld_amount,
            ));
        self
    }

    pub fn with_multi_token_transfer(
        mut self,
        payments: ManagedVec<SA::ProxyTypeManager, EsdtTokenPayment<SA::ProxyTypeManager>>,
    ) -> Self {
        self.payments = payments;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.explicit_gas_limit = gas_limit;
        self
    }

    // pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
    //     &mut self.arg_buffer
    // }

    /// Provided for cases where we build the contract call by hand.
    pub fn push_arg_managed_buffer(&mut self, m_buffer: ManagedBuffer<SA::ProxyTypeManager>) {
        self.arg_buffer.push_arg_raw(m_buffer)
    }

    /// Provided for cases where we build the contract call by hand.
    /// Convenience method, also creates the new managed buffer from bytes.
    pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
        self.arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(
            self.api.type_manager(),
            bytes,
        ));
    }

    pub fn push_endpoint_arg<D: ContractCallArg>(&mut self, endpoint_arg: D) {
        endpoint_arg.push_dyn_arg(&mut self.arg_buffer);
    }

    fn no_payments(
        &self,
    ) -> ManagedVec<SA::ProxyTypeManager, EsdtTokenPayment<SA::ProxyTypeManager>> {
        ManagedVec::new_empty(self.api.type_manager())
    }

    /// If this is an ESDT call, it converts it to a regular call to ESDTTransfer.
    /// Async calls require this step, but not `transfer_esdt_execute`.
    fn convert_to_esdt_transfer_call(self) -> Self {
        match self.payments.len() {
            0 => self,
            1 => self.convert_to_single_transfer_esdt_call(),
            _ => self.convert_to_multi_transfer_esdt_call(),
        }
    }

    fn convert_to_single_transfer_esdt_call(mut self) -> Self {
        if let Some(payment) = self.payments.get(0) {
            if payment.token_name.is_egld() {
                self.egld_payment = payment.amount;
                self.payments.clear();
                self
            } else if payment.token_nonce == 0 {
                let no_payments = self.no_payments();

                // fungible ESDT
                let mut new_arg_buffer = ManagedArgBuffer::new_empty(self.api.type_manager());
                new_arg_buffer.push_arg(&payment.token_name);
                new_arg_buffer.push_arg(&payment.amount);
                new_arg_buffer.push_arg(&self.endpoint_name);

                let zero = BigUint::zero(self.api.type_manager());
                let endpoint_name =
                    ManagedBuffer::new_from_bytes(self.api.type_manager(), ESDT_TRANSFER_STRING);

                ContractCall {
                    api: self.api.clone(),
                    to: self.to,
                    egld_payment: zero,
                    payments: no_payments,
                    explicit_gas_limit: self.explicit_gas_limit,
                    endpoint_name,
                    arg_buffer: new_arg_buffer.concat(self.arg_buffer),
                    _return_type: PhantomData,
                }
            } else {
                let payments = self.no_payments();

                // NFT
                // `ESDTNFTTransfer` takes 4 arguments:
                // arg0 - token identifier
                // arg1 - nonce
                // arg2 - quantity to transfer
                // arg3 - destination address
                let mut new_arg_buffer = ManagedArgBuffer::new_empty(self.api.type_manager());
                new_arg_buffer.push_arg(&payment.token_name);
                new_arg_buffer.push_arg(&payment.token_nonce);
                new_arg_buffer.push_arg(&payment.amount);
                new_arg_buffer.push_arg(&self.to);
                new_arg_buffer.push_arg(&self.endpoint_name);

                // nft transfer is sent to self, sender = receiver
                let recipient_addr = self.api.blockchain().get_sc_address();
                let zero = BigUint::zero(self.api.type_manager());
                let endpoint_name = ManagedBuffer::new_from_bytes(
                    self.api.type_manager(),
                    ESDT_NFT_TRANSFER_STRING,
                );

                ContractCall {
                    api: self.api,
                    to: recipient_addr,
                    egld_payment: zero,
                    payments,
                    explicit_gas_limit: self.explicit_gas_limit,
                    endpoint_name,
                    arg_buffer: new_arg_buffer.concat(self.arg_buffer),
                    _return_type: PhantomData,
                }
            }
        } else {
            self
        }
    }

    fn convert_to_multi_transfer_esdt_call(self) -> Self {
        let payments = self.no_payments();

        let mut new_arg_buffer = ManagedArgBuffer::new_empty(self.api.type_manager());
        new_arg_buffer.push_arg(self.to);
        new_arg_buffer.push_arg(self.payments.len());

        for payment in self.payments.into_iter() {
            // TODO: check that `!token_name.is_egld()` or let Arwen throw the error?
            new_arg_buffer.push_arg(payment.token_name);
            new_arg_buffer.push_arg(payment.token_nonce);
            new_arg_buffer.push_arg(payment.amount);
        }
        new_arg_buffer.push_arg(self.endpoint_name);

        // multi transfer is sent to self, sender = receiver
        let recipient_addr = self.api.blockchain().get_sc_address();
        let zero = BigUint::zero(self.api.type_manager());
        let endpoint_name =
            ManagedBuffer::new_from_bytes(self.api.type_manager(), ESDT_MULTI_TRANSFER_STRING);

        ContractCall {
            api: self.api,
            to: recipient_addr,
            egld_payment: zero,
            payments,
            explicit_gas_limit: self.explicit_gas_limit,
            endpoint_name,
            arg_buffer: new_arg_buffer.concat(self.arg_buffer),
            _return_type: PhantomData,
        }
    }

    fn resolve_gas_limit(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            self.api.blockchain().get_gas_left()
        } else {
            self.explicit_gas_limit
        }
    }

    pub fn async_call(mut self) -> AsyncCall<SA> {
        self = self.convert_to_esdt_transfer_call();
        AsyncCall {
            api: self.api,
            to: self.to,
            egld_payment: self.egld_payment,
            endpoint_name: self.endpoint_name,
            arg_buffer: self.arg_buffer,
            callback_call: None,
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
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        );

        let mut loader = ManagedResultArgLoader::new(self.api.type_manager(), raw_result);
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
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
            range_closure,
        );

        let mut loader = ManagedResultArgLoader::new(self.api.type_manager(), raw_result);
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
            &self.egld_payment,
            &self.endpoint_name,
            &self.arg_buffer,
        );
    }

    fn resolve_gas_limit_with_leftover(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            let mut gas_left = self.api.blockchain().get_gas_left();
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
        if self.payments.len() == 1 {
            self.single_transfer_execute()
        } else {
            self.multi_transfer_execute()
        }
    }

    fn single_transfer_execute(self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        if let Some(payment) = &self.payments.get(0) {
            if payment.token_name.is_egld() {
                self.api.direct_egld_execute(
                    &self.to,
                    &payment.amount,
                    gas_limit,
                    &self.endpoint_name,
                    &self.arg_buffer,
                );
            } else if payment.token_nonce == 0 {
                // fungible ESDT
                self.api.direct_esdt_execute(
                    &self.to,
                    &payment.token_name,
                    &payment.amount,
                    gas_limit,
                    &self.endpoint_name,
                    &self.arg_buffer,
                );
            } else {
                // non-fungible/semi-fungible ESDT
                self.api.direct_esdt_nft_execute(
                    &self.to,
                    &payment.token_name,
                    payment.token_nonce,
                    &payment.amount,
                    gas_limit,
                    &self.endpoint_name,
                    &self.arg_buffer,
                );
            }
        } else {
            // no payment
            self.api.direct_egld_execute(
                &self.to,
                &BigUint::zero(self.api.type_manager()),
                gas_limit,
                &self.endpoint_name,
                &self.arg_buffer,
            );
        }
    }

    fn multi_transfer_execute(self) {
        let gas_limit = self.resolve_gas_limit_with_leftover();
        let result = self.api.direct_multi_esdt_transfer_execute(
            &self.to,
            &self.payments,
            gas_limit,
            &self.endpoint_name,
            &self.arg_buffer,
        );

        if let Err(e) = result {
            self.api.error_api().signal_error(e);
        }
    }
}
