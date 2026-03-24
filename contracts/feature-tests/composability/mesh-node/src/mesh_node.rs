#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod mesh_node_proxy;

pub type NodeName<M> = ManagedBuffer<M>;

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub enum ProgrammedCallType {
    AsyncV1,
    AsyncV2,
    Sync,
    TransferExecute,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct ProgrammedCall<M: ManagedTypeApi> {
    pub call_type: ProgrammedCallType,
    pub to: ManagedAddress<M>,
    pub gas_limit: u64,
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
pub enum TraceName {
    Bump,
    AsyncV1CallbackOk,
    AsyncV1CallbackErr,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Trace<M: ManagedTypeApi> {
    pub location: TraceName,
    pub block_nonce: u64,
    pub initial_gas: u64,
    pub final_gas: u64,
    pub input: ManagedVec<M, TraceItem<M>>,
    pub call_value: PaymentVec<M>,
    pub back_transfers: PaymentVec<M>,
    pub results: ManagedVec<M, ManagedBuffer<M>>,
}

/// Testing multiple calls per transaction.
#[multiversx_sc::contract]
pub trait MeshNode {
    #[view]
    #[storage_mapper("id")]
    fn id(&self) -> SingleValueMapper<NodeName<Self::Api>>;

    #[view]
    #[storage_mapper("programmed_calls")]
    fn programmed_calls(&self) -> SingleValueMapper<ManagedVec<ProgrammedCall<Self::Api>>>;

    #[view]
    #[storage_mapper("programmed_returns")]
    fn programmed_returns(&self) -> SingleValueMapper<PaymentVec>;

    #[view]
    #[storage_mapper("trace")]
    fn trace(&self) -> VecMapper<Trace<Self::Api>>;

    #[init]
    fn init(&self, id: NodeName<Self::Api>) {
        self.id().set(id);
    }

    #[endpoint(programCalls)]
    fn program_calls(&self, calls: MultiValueManagedVec<ProgrammedCall<Self::Api>>) {
        self.programmed_calls().set(calls.into_vec());
    }

    #[endpoint(programReturns)]
    fn program_returns(&self, returns: MultiValueManagedVec<Payment>) {
        self.programmed_returns().set(returns.into_vec());
    }

    /// Records the call, then calls all programmed calls.
    #[endpoint]
    #[payable]
    fn bump(&self, call_trace: MultiValueManagedVec<TraceItem<Self::Api>>) {
        let initial_gas = self.blockchain().get_gas_left();
        let call_value = self.call_value().all();
        let trace_index = self.trace().push(&Trace {
            location: TraceName::Bump,
            block_nonce: self.blockchain().get_block_nonce(),
            initial_gas,
            final_gas: 0,
            input: call_trace.as_vec().clone(),
            call_value: call_value.clone(),
            back_transfers: PaymentVec::new(),
            results: ManagedVec::new(),
        });

        self.forward_programmed_calls(&call_trace);

        self.send_programmed_returns();

        self.trace().update(trace_index, |trace| {
            trace.final_gas = self.blockchain().get_gas_left();
        });
    }

    fn forward_programmed_calls(&self, call_trace: &MultiValueManagedVec<TraceItem<Self::Api>>) {
        let calls = self.programmed_calls().get();
        for (call_index, call) in calls.into_iter().enumerate() {
            self.forward_programmed_call(call, call_index, call_trace);
        }
    }

    fn forward_programmed_call(
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

        let contract_call = self
            .tx()
            .to(&call.to)
            .typed(mesh_node_proxy::MeshNodeProxy)
            .bump(&child_call_trace)
            .payment(&call.payments);

        match call.call_type {
            ProgrammedCallType::AsyncV1 => {
                contract_call
                    .callback(self.callbacks().async_v1_callback(&child_call_trace))
                    .async_call_and_exit();
            }
            ProgrammedCallType::AsyncV2 => {
                contract_call
                    .gas(call.gas_limit)
                    .callback(self.callbacks().async_v2_callback(&child_call_trace))
                    .register_promise();
            }
            ProgrammedCallType::TransferExecute => {
                contract_call.gas(call.gas_limit).transfer_execute();
            }
            ProgrammedCallType::Sync => {
                contract_call.gas(call.gas_limit).sync_call();
            }
        }
    }

    fn send_programmed_returns(&self) {
        let returns = self.programmed_returns().get();
        if returns.is_empty() {
            return;
        }
        let all_sufficient = returns.iter().all(|payment| {
            let balance = self
                .blockchain()
                .get_sc_balance(&payment.token_identifier, payment.token_nonce);
            payment.amount <= balance
        });
        if all_sufficient {
            let caller = self.blockchain().get_caller();
            self.tx().to(&caller).payment(returns).transfer();
        }
    }

    fn callback_body(
        &self,
        call_trace: &MultiValueManagedVec<TraceItem<Self::Api>>,
        result: MultiValueEncoded<ManagedBuffer>,
    ) {
        let call_value = self.call_value().all();
        let back_payments = self.blockchain().get_back_transfers();
        self.trace().push(&Trace {
            location: TraceName::AsyncV1CallbackOk,
            block_nonce: self.blockchain().get_block_nonce(),
            initial_gas: 0,
            final_gas: 0,
            input: call_trace.as_vec().clone(),
            call_value: call_value.clone(),
            back_transfers: back_payments.into_payment_vec(),
            results: result.into_vec_of_buffers(),
        });
    }

    #[callback]
    fn async_v1_callback(
        &self,
        call_trace: &MultiValueManagedVec<TraceItem<Self::Api>>,
        #[call_result] result: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.callback_body(call_trace, result);
    }

    #[promises_callback]
    fn async_v2_callback(
        &self,
        call_trace: &MultiValueManagedVec<TraceItem<Self::Api>>,
        #[call_result] result: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.callback_body(call_trace, result);
    }
}
