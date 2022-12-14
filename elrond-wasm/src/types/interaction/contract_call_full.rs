use core::marker::PhantomData;

use elrond_codec::TopEncodeMulti;

use crate::{
    api::{
        CallTypeApi, ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME,
        ESDT_TRANSFER_FUNC_NAME,
    },
    contract_base::BlockchainWrapper,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ManagedArgBuffer};

#[must_use]
pub struct ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub egld_payment: BigUint<SA>,
    pub payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallFull<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

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
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        egld_payment: BigUint<SA>,
        esdt_payments: ManagedVec<SA, EsdtTokenPayment<SA>>,
    ) -> Self {
        ContractCallFull {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            egld_payment,
            payments: esdt_payments,
        }
    }

    pub(super) fn transfer_execute(self) {
        match self.payments.len() {
            0 => self.no_payment_transfer_execute(),
            1 => self.single_transfer_execute(),
            _ => self.multi_transfer_execute(),
        }
    }

    fn no_payments(&self) -> ManagedVec<SA, EsdtTokenPayment<SA>> {
        ManagedVec::new()
    }

    /// If this is an ESDT call, it converts it to a regular call to ESDTTransfer.
    /// Async calls require this step, but not `transfer_esdt_execute`.
    pub fn convert_to_esdt_transfer_call(self) -> Self {
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
}
