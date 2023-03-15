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
    pub call_type: QueuedCallType,
    pub to: ManagedAddress<M>,
    pub endpoint_name: ManagedBuffer<M>,
    pub payments: EgldOrMultiEsdtPayment<M>,
}

/// Testing multiple calls per transaction.
#[multiversx_sc::contract]
pub trait ForwarderQueue {
    #[init]
    fn init(&self) {}

    #[view]
    #[storage_mapper("queued_calls")]
    fn queued_calls(&self) -> LinkedListMapper<QueuedCall<Self::Api>>;

    #[endpoint]
    #[payable("*")]
    fn add_queued_call(
        &self,
        call_type: QueuedCallType,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
    ) {
        let payments = self.call_value().any_payment();

        match &payments {
            EgldOrMultiEsdtPayment::Egld(egld_value) => {
                self.add_queued_call_egld_event(&call_type, &to, &endpoint_name, egld_value);
            },
            EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => {
                self.add_queued_call_esdt_event(
                    &call_type,
                    &to,
                    &endpoint_name,
                    &esdt_payments.clone().into_multi_value(),
                );
            },
        }

        self.queued_calls().push_back(QueuedCall {
            call_type,
            to,
            endpoint_name,
            payments,
        });
    }

    #[endpoint]
    fn forward_queued_calls(&self) {
        while let Some(node) = self.queued_calls().pop_front() {
            let call = node.clone().into_value();

            let contract_call = match call.payments {
                EgldOrMultiEsdtPayment::Egld(egld_value) => {
                    self.forward_queued_call_egld_event(
                        &call.call_type,
                        &call.to,
                        &call.endpoint_name,
                        &egld_value,
                    );

                    ContractCallWithEgld::<Self::Api, ()>::new(
                        call.to.clone(),
                        call.endpoint_name.clone(),
                        egld_value,
                    )
                },
                EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => {
                    self.forward_queued_call_esdt_event(
                        &call.call_type,
                        &call.to,
                        &call.endpoint_name,
                        &esdt_payments.clone().into_multi_value(),
                    );

                    ContractCallWithMultiEsdt::<Self::Api, ()>::new(
                        call.to.clone(),
                        call.endpoint_name.clone(),
                        esdt_payments,
                    )
                    .into_normalized()
                },
            };

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

    #[event("forward_queued_call_egld")]
    fn forward_queued_call_egld_event(
        &self,
        #[indexed] call_type: &QueuedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] egld_value: &BigUint,
    );

    #[event("forward_queued_call_esdt")]
    fn forward_queued_call_esdt_event(
        &self,
        #[indexed] call_type: &QueuedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );

    #[event("add_queued_call_egld")]
    fn add_queued_call_egld_event(
        &self,
        #[indexed] call_type: &QueuedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] egld_value: &BigUint,
    );

    #[event("add_queued_call_esdt")]
    fn add_queued_call_esdt_event(
        &self,
        #[indexed] call_type: &QueuedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );
}
