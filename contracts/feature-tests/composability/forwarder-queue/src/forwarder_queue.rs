#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod forwarder_queue_proxy;

pub type NodeName<M> = ManagedBuffer<M>;

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub enum ProgrammedCallType {
    Sync,
    LegacyAsync,
    TransferExecute,
    Promise,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct ProgrammedCall<M: ManagedTypeApi> {
    pub call_type: ProgrammedCallType,
    pub to: ManagedAddress<M>,
    pub gas_limit: u64,
    pub endpoint_name: ManagedBuffer<M>,
    pub args: ManagedArgBuffer<M>,
    pub payments: PaymentVec<M>,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct TraceItem<M: ManagedTypeApi> {
    pub caller_id: NodeName<M>,
    pub call_index: usize,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Trace<M: ManagedTypeApi> {
    pub block_nonce: u64,
    pub initial_gas: u64,
    pub final_gas: u64,
    pub items: ManagedVec<M, TraceItem<M>>,
}

/// Testing multiple calls per transaction.
#[multiversx_sc::contract]
pub trait ForwarderQueue {
    #[view]
    #[storage_mapper("id")]
    fn id(&self) -> SingleValueMapper<NodeName<Self::Api>>;

    #[view]
    #[storage_mapper("queued_calls")]
    fn queued_calls(&self) -> SingleValueMapper<ManagedVec<ProgrammedCall<Self::Api>>>;

    #[view]
    #[storage_mapper("trace")]
    fn trace(&self) -> VecMapper<Trace<Self::Api>>;

    #[init]
    fn init(&self, id: NodeName<Self::Api>) {
        self.id().set(id);
    }

    #[endpoint]
    fn set_queued_calls(&self, calls: MultiValueManagedVec<ProgrammedCall<Self::Api>>) {
        self.queued_calls().set(calls.into_vec());
    }

    /// Records the call, then calls all programmed calls.
    #[endpoint]
    #[payable("*")]
    fn bump(&self, call_trace: MultiValueManagedVec<TraceItem<Self::Api>>) {
        let initial_gas = self.blockchain().get_gas_left();
        let trace_index = self.trace().push(&Trace {
            block_nonce: self.blockchain().get_block_nonce(),
            initial_gas,
            final_gas: 0,
            items: call_trace.as_vec().clone(),
        });
        let calls = self.queued_calls().get();
        for (call_index, call) in calls.into_iter().enumerate() {
            self.forward_queued_call(call, call_index, &call_trace);
        }
        self.trace().update(trace_index, |trace| {
            trace.final_gas = self.blockchain().get_gas_left();
        });
    }

    fn forward_queued_call(
        &self,
        call: ProgrammedCall<Self::Api>,
        call_index: usize,
        call_trace: &MultiValueManagedVec<TraceItem<Self::Api>>,
    ) {
        let mut child_call_trace: MultiValueManagedVec<TraceItem<Self::Api>> = call_trace.clone();
        child_call_trace.push(TraceItem {
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
            ProgrammedCallType::Sync => {
                contract_call.gas(call.gas_limit).sync_call();
            }
            ProgrammedCallType::LegacyAsync => {
                contract_call.async_call_and_exit();
            }
            ProgrammedCallType::TransferExecute => {
                contract_call.gas(call.gas_limit).transfer_execute();
            }
            ProgrammedCallType::Promise => {
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
        #[indexed] call_type: &ProgrammedCallType,
        #[indexed] to: &ManagedAddress,
        #[indexed] endpoint_name: &ManagedBuffer,
        #[indexed] multi_esdt: &MultiValueEncoded<PaymentMultiValue>,
    );
}
