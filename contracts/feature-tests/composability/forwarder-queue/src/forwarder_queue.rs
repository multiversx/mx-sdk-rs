#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub enum QueuedCallType {
    Sync,
    LegacyAsync,
    TransferExecute,
    Promise,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct QueuedCall<M: ManagedTypeApi> {
    pub call_type: QueuedCallType,
    pub to: ManagedAddress<M>,
    pub gas_limit: u64,
    pub endpoint_name: ManagedBuffer<M>,
    pub args: ManagedArgBuffer<M>,
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
    fn add_queued_call_sync(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.add_queued_call(QueuedCallType::Sync, to, 0, endpoint_name, args);
    }

    #[endpoint]
    #[payable("*")]
    fn add_queued_call_legacy_async(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.add_queued_call(QueuedCallType::LegacyAsync, to, 0, endpoint_name, args);
    }

    #[endpoint]
    #[payable("*")]
    fn add_queued_call_transfer_execute(
        &self,
        to: ManagedAddress,
        gas_limit: u64,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.add_queued_call(
            QueuedCallType::TransferExecute,
            to,
            gas_limit,
            endpoint_name,
            args,
        );
    }

    #[endpoint]
    #[payable("*")]
    fn add_queued_call_transfer_esdt(
        &self,
        to: ManagedAddress,
        gas_limit: u64,
        endpoint_name: ManagedBuffer,
        token: TokenIdentifier,
        amount: BigUint,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let mut payment = ManagedVec::new();
        payment.push(EsdtTokenPayment::new(token, 0, amount));

        let payments = EgldOrMultiEsdtPayment::MultiEsdt(payment);

        let call_type = QueuedCallType::Promise;
        self.queued_calls().push_back(QueuedCall {
            call_type,
            to,
            gas_limit,
            endpoint_name,
            args: args.to_arg_buffer(),
            payments,
        });
    }

    #[endpoint]
    #[payable("*")]
    fn add_queued_call_promise(
        &self,
        to: ManagedAddress,
        gas_limit: u64,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.add_queued_call(QueuedCallType::Promise, to, gas_limit, endpoint_name, args);
    }

    #[endpoint]
    #[payable("*")]
    fn add_queued_call(
        &self,
        call_type: QueuedCallType,
        to: ManagedAddress,
        gas_limit: u64,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
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
            gas_limit,
            endpoint_name,
            args: args.to_arg_buffer(),
            payments,
        });
    }

    #[endpoint]
    #[payable("*")]
    fn forward_queued_calls(&self) {
        while let Some(node) = self.queued_calls().pop_front() {
            let call = node.clone().into_value();

            match &call.payments {
                EgldOrMultiEsdtPayment::Egld(egld_value) => {
                    self.forward_queued_call_egld_event(
                        &call.call_type,
                        &call.to,
                        &call.endpoint_name,
                        egld_value,
                    );
                },
                EgldOrMultiEsdtPayment::MultiEsdt(esdt_payments) => {
                    self.forward_queued_call_esdt_event(
                        &call.call_type,
                        &call.to,
                        &call.endpoint_name,
                        &esdt_payments.clone().into_multi_value(),
                    );
                },
            };

            let contract_call = ContractCallWithAnyPayment::<_, ()>::new(
                call.to,
                call.endpoint_name,
                call.payments,
            );

            match call.call_type {
                QueuedCallType::Sync => {
                    contract_call.execute_on_dest_context::<()>();
                },
                QueuedCallType::LegacyAsync => {
                    contract_call.async_call().call_and_exit();
                },
                QueuedCallType::TransferExecute => {
                    contract_call
                        .with_gas_limit(call.gas_limit)
                        .transfer_execute();
                },
                QueuedCallType::Promise => {
                    #[cfg(feature = "promises")]
                    contract_call
                        .with_gas_limit(call.gas_limit)
                        .with_raw_arguments(call.args)
                        .async_call_promise()
                        .with_callback(self.callbacks().promises_callback_method())
                        .register_promise();
                },
            }
        }
    }

    #[promises_callback]
    #[label("promises-callback")]
    fn promises_callback_method(&self) {
        self.callback_count().update(|c| *c += 1);
        let payments = self.call_value().any_payment();

        let payments_data_string =
            ContractCallNoPayment::<_, ()>::new(ManagedAddress::default(), ManagedBuffer::new())
                .with_any_payment(payments)
                .into_call_data_string();
        self.callback_payments().set(payments_data_string);
    }

    #[view]
    #[storage_mapper("callback_count")]
    fn callback_count(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("callback_payments")]
    fn callback_payments(&self) -> SingleValueMapper<ManagedBuffer>;

    #[event("forward_queued_callback")]
    fn forward_queued_callback_event(&self);

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
