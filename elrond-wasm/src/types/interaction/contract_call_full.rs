use core::marker::PhantomData;

use elrond_codec::TopEncodeMulti;

use crate::{
    api::{
        BlockchainApiImpl, CallTypeApi, ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME,
        ESDT_TRANSFER_FUNC_NAME,
    },
    contract_base::{BlockchainWrapper, SendRawWrapper},
    types::{BigUint, EsdtTokenPayment, ManagedVec},
};

use super::{
    contract_call_common::{TRANSFER_EXECUTE_DEFAULT_LEFTOVER, UNSPECIFIED_GAS_LIMIT},
    contract_call_no_payment::ContractCallNoPayment,
    ContractCallTrait, ManagedArgBuffer,
};

#[must_use]
pub struct ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub(super) egld_payment: BigUint<SA>,
    pub(super) payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCallTrait<SA, OriginalResult>
    for ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_contract_call_full(self) -> ContractCallFull<SA, OriginalResult> {
        self
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }
}

impl<SA, OriginalResult> ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    fn no_payments(&self) -> ManagedVec<SA, EsdtTokenPayment<SA>> {
        ManagedVec::new()
    }

    /// If this is an ESDT call, it converts it to a regular call to ESDTTransfer.
    /// Async calls require this step, but not `transfer_esdt_execute`.
    pub(super) fn convert_to_esdt_transfer_call(self) -> Self {
        match self.payments.len() {
            0 => self,
            1 => self.convert_to_single_transfer_esdt_call(),
            _ => self.convert_to_multi_transfer_esdt_call(),
        }
    }

    fn convert_to_single_transfer_esdt_call(self) -> Self {
        if let Some(payment) = self.payments.try_get(0) {
            if payment.token_nonce == 0 {
                let no_payments = self.no_payments();

                // fungible ESDT
                let mut new_arg_buffer = ManagedArgBuffer::new();
                new_arg_buffer.push_arg(&payment.token_identifier);
                new_arg_buffer.push_arg(&payment.amount);
                if !self.basic.endpoint_name.is_empty() {
                    new_arg_buffer.push_arg(&self.basic.endpoint_name);
                }

                ContractCallFull {
                    basic: ContractCallNoPayment {
                        _phantom: PhantomData,
                        to: self.basic.to,
                        endpoint_name: ESDT_TRANSFER_FUNC_NAME.into(),
                        arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                        explicit_gas_limit: self.basic.explicit_gas_limit,
                        _return_type: PhantomData,
                    },
                    egld_payment: BigUint::zero(),
                    payments: no_payments,
                }
            } else {
                let payments = self.no_payments();

                // NFT
                // `ESDTNFTTransfer` takes 4 arguments:
                // arg0 - token identifier
                // arg1 - nonce
                // arg2 - quantity to transfer
                // arg3 - destination address
                let mut new_arg_buffer = ManagedArgBuffer::new();
                new_arg_buffer.push_arg(&payment.token_identifier);
                new_arg_buffer.push_arg(payment.token_nonce);
                new_arg_buffer.push_arg(&payment.amount);
                new_arg_buffer.push_arg(&self.basic.to);
                if !self.basic.endpoint_name.is_empty() {
                    new_arg_buffer.push_arg(&self.basic.endpoint_name);
                }

                // nft transfer is sent to self, sender = receiver
                let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

                ContractCallFull {
                    basic: ContractCallNoPayment {
                        _phantom: PhantomData,
                        to: recipient_addr,
                        endpoint_name: ESDT_NFT_TRANSFER_FUNC_NAME.into(),
                        arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                        explicit_gas_limit: self.basic.explicit_gas_limit,
                        _return_type: PhantomData,
                    },
                    egld_payment: BigUint::zero(),
                    payments,
                }
            }
        } else {
            self
        }
    }

    fn convert_to_multi_transfer_esdt_call(self) -> Self {
        let payments = self.no_payments();

        let mut new_arg_buffer = ManagedArgBuffer::new();
        new_arg_buffer.push_arg(self.basic.to);
        new_arg_buffer.push_arg(self.payments.len());

        for payment in self.payments.into_iter() {
            new_arg_buffer.push_arg(payment.token_identifier);
            new_arg_buffer.push_arg(payment.token_nonce);
            new_arg_buffer.push_arg(payment.amount);
        }
        if !self.basic.endpoint_name.is_empty() {
            new_arg_buffer.push_arg(self.basic.endpoint_name);
        }

        // multi transfer is sent to self, sender = receiver
        let recipient_addr = BlockchainWrapper::<SA>::new().get_sc_address();

        ContractCallFull {
            basic: ContractCallNoPayment {
                _phantom: PhantomData,
                to: recipient_addr,
                endpoint_name: ESDT_MULTI_TRANSFER_FUNC_NAME.into(),
                arg_buffer: new_arg_buffer.concat(self.basic.arg_buffer),
                explicit_gas_limit: self.basic.explicit_gas_limit,
                _return_type: PhantomData,
            },
            egld_payment: BigUint::zero(),
            payments,
        }
    }

    pub fn resolve_gas_limit(&self) -> u64 {
        if self.basic.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            SA::blockchain_api_impl().get_gas_left()
        } else {
            self.basic.explicit_gas_limit
        }
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
