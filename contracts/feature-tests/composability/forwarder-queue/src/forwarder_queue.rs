#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub enum QueuedCallType {
    Sync,
    LegacyAsync,
    TransferExecute,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct QueuedCall<M: ManagedTypeApi> {
    call_type: QueuedCallType,
    to: ManagedAddress<M>,
    endpoint_name: ManagedBuffer<M>,
    payment_token: EgldOrEsdtTokenIdentifier<M>,
    payment_nonce: u64,
    payment_amount: BigUint<M>,
}

/// Testing multiple calls per transaction, cascading on.
///
/// TODO: write actual tests with these.
#[multiversx_sc::contract]
pub trait ForwarderQueue {
    #[init]
    fn init(&self) {}

    #[view]
    #[storage_mapper("queued_calls")]
    fn queued_calls(&self) -> LinkedListMapper<QueuedCall<Self::Api>>;

    #[endpoint]
    fn add_queued_call(
        &self,
        call_type: QueuedCallType,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        payment_token: EgldOrEsdtTokenIdentifier,
        payment_nonce: u64,
        payment_amount: BigUint,
    ) {
        self.queued_calls().push_back(QueuedCall {
            call_type,
            to,
            endpoint_name,
            payment_token,
            payment_nonce,
            payment_amount,
        });
    }

    #[endpoint]
    #[payable("*")]
    fn forward_queued_calls(&self) {
        let esdt_transfers_multi = self.call_value().all_esdt_transfers();
        self.forward_queued_calls_event(
            &self.call_value().egld_value(),
            &esdt_transfers_multi.into_multi_value(),
        );

        while let Some(node) = self.queued_calls().pop_front() {
            let call = node.clone().into_value();

            let contract_call = ContractCallWithEgldOrSingleEsdt::<Self::Api, ()>::new(
                call.to.clone(),
                call.endpoint_name.clone(),
                call.payment_token.clone(),
                call.payment_nonce,
                call.payment_amount.clone(),
            );
            match call.call_type {
                QueuedCallType::Sync => {
                    contract_call.execute_on_dest_context::<()>();
                },
                QueuedCallType::LegacyAsync => {
                    contract_call.async_call().call_and_exit();
                },
                QueuedCallType::TransferExecute => {
                    contract_call.transfer_execute();
                },
            }
        }
    }

    #[event("forward_queued_calls")]
    fn forward_queued_calls_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );
}
