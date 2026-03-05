#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod forwarder_queue_proxy;

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub enum QueuedCallType {
    Sync,
    LegacyAsync,
    TransferExecute,
    Promise,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct QueuedCall<M: ManagedTypeApi> {
    pub call_type: QueuedCallType,
    pub to: ManagedAddress<M>,
    pub gas_limit: u64,
    pub endpoint_name: ManagedBuffer<M>,
    pub args: ManagedArgBuffer<M>,
    pub payments: PaymentVec<M>,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct CallInfo {
    pub caller_id: u32,
    pub call_index: usize,
}

/// Testing multiple calls per transaction.
#[multiversx_sc::contract]
pub trait ForwarderQueue {
    #[view]
    #[storage_mapper("id")]
    fn id(&self) -> SingleValueMapper<u32>;

    #[view]
    #[storage_mapper("queued_calls")]
    fn queued_calls(&self) -> SingleValueMapper<ManagedVec<QueuedCall<Self::Api>>>;

    #[view]
    #[storage_mapper("trace")]
    fn trace(&self) -> VecMapper<ManagedVec<CallInfo>>;

    #[init]
    fn init(&self, id: u32) {
        self.id().set(id);
    }

    #[endpoint]
    fn set_queued_calls(&self, calls: MultiValueManagedVec<QueuedCall<Self::Api>>) {
        self.queued_calls().set(calls.into_vec());
    }

    /// Records the call, then calls all programmed calls.
    #[endpoint]
    #[payable("*")]
    fn bump(&self, call_trace: MultiValueManagedVec<CallInfo>) {
        self.trace().push(call_trace.as_vec());
        let calls = self.queued_calls().get();
        for (call_index, call) in calls.into_iter().enumerate() {
            self.forward_queued_call(call, call_index, &call_trace);
        }
    }

    fn forward_queued_call(
        &self,
        call: QueuedCall<Self::Api>,
        call_index: usize,
        call_trace: &MultiValueManagedVec<CallInfo>,
    ) {
        let mut child_call_trace: MultiValueManagedVec<CallInfo> = call_trace.clone();
        child_call_trace.push(CallInfo {
            caller_id: self.id().get(),
            call_index,
        });

        self.forward_queued_call_payment_event(
            &call.call_type,
            &call.to,
            &call.endpoint_name,
            &call.payments.clone().into_multi_value(),
        );

        let contract_call = self
            .tx()
            .to(&call.to)
            .typed(forwarder_queue_proxy::ForwarderQueueProxy)
            .bump(child_call_trace)
            .payment(&call.payments);

        match call.call_type {
            QueuedCallType::Sync => {
                contract_call.gas(call.gas_limit).sync_call();
            }
            QueuedCallType::LegacyAsync => {
                contract_call.async_call_and_exit();
            }
            QueuedCallType::TransferExecute => {
                contract_call.gas(call.gas_limit).transfer_execute();
            }
            QueuedCallType::Promise => {
                contract_call
                    .gas(call.gas_limit)
                    .arguments_raw(call.args)
                    .callback(self.callbacks().promises_callback_method())
                    .register_promise();
            }
        }
    }

    #[promises_callback]
    fn promises_callback_method(&self) {
        self.callback_count().update(|c| *c += 1);
        let payments = self.call_value().all();

        let payments_data_string = self
            .tx()
            .to(&ManagedAddress::default())
            .payment(payments)
            .raw_call("")
            .to_call_data_string();

        self.callback_payments().set(payments_data_string);
    }

    #[view]
    #[storage_mapper("callback_count")]
    fn callback_count(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("callback_payments")]
    fn callback_payments(&self) -> SingleValueMapper<ManagedBuffer>;

    #[event("forward_queued_call_payment")]
    fn forward_queued_call_payment_event(
        &self,
        #[indexed] call_type: &QueuedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] multi_esdt: &MultiValueEncoded<PaymentMultiValue>,
    );
}
