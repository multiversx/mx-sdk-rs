use std::{cell::RefCell, rc::Rc};

use multiversx_sc_snippets::imports::*;
use num_bigint::BigUint;

use crate::{
    call_tree::{CallNode, CallState, ForwarderQueueTarget},
    comp_interact_controller::ComposabilityInteract,
    forwarder_queue_proxy::{self, QueuedCallType},
};

const FORWARD_QUEUED_CALLS_ENDPOINT: &str = "forward_queued_calls";
const DEFAULT_GAS_LIMIT: u64 = 10_000_000;

impl ComposabilityInteract {
    pub async fn add_queued_calls_to_children(
        &mut self,
        forwarders: &Vec<Rc<RefCell<ForwarderQueueTarget>>>,
        call_type: QueuedCallType,
        endpoint_name: &str,
        payment_token: EgldOrEsdtTokenIdentifier<StaticApi>,
        payment_nonce: u64,
        payment_amount: BigUint,
    ) {
        let mut buffer = self.interactor.homogenous_call_buffer();

        for fwd_rc in forwarders {
            let (fwd_name, fwd_children) = {
                let fwd = fwd_rc.borrow();
                (fwd.name.clone(), fwd.children.clone())
            };
            let fwd_addr = {
                let fwd = fwd_rc.borrow();
                fwd.address.clone().unwrap()
            };

            for child in &fwd_children {
                match child {
                    CallNode::ForwarderQueue(child_fwd_rc) => {
                        // forward_queued_calls to ForwarderQueue's children
                        let child_fwd_addr = {
                            let child_fwd = (*child_fwd_rc).borrow();
                            println!("child_name: {}, parent_name: {}", child_fwd.name, &fwd_name);
                            child_fwd.address.clone().unwrap()
                        };

                        buffer.push_tx(|tx| {
                            tx.from(&self.wallet_address)
                                .to(&fwd_addr)
                                .gas(70_000_000u64)
                                .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                                .add_queued_call(
                                    call_type.clone(),
                                    child_fwd_addr,
                                    DEFAULT_GAS_LIMIT,
                                    FORWARD_QUEUED_CALLS_ENDPOINT,
                                    MultiValueEncoded::<StaticApi, _>::new(),
                                )
                                .payment(EgldOrEsdtTokenPayment::new(
                                    payment_token.clone(),
                                    payment_nonce,
                                    payment_amount.clone().into(),
                                ))
                                .returns(ReturnsStatus)
                                .returns(ReturnsResult)
                        });
                    },
                    CallNode::Vault(vault_rc) => {
                        // Call Vault
                        let vault_addr = {
                            let vault = (*vault_rc).borrow();
                            println!("child_name: {}, parent_name: {}", vault.name, &fwd_name);
                            vault.address.clone().unwrap()
                        };

                        buffer.push_tx(|tx| {
                            tx.from(&self.wallet_address)
                                .to(&fwd_addr)
                                .gas(70_000_000u64)
                                .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                                .add_queued_call(
                                    call_type.clone(),
                                    vault_addr,
                                    DEFAULT_GAS_LIMIT,
                                    endpoint_name,
                                    MultiValueEncoded::<StaticApi, _>::new(),
                                )
                                .payment(EgldOrEsdtTokenPayment::new(
                                    payment_token.clone(),
                                    payment_nonce,
                                    payment_amount.clone().into(),
                                ))
                                .returns(ReturnsStatus)
                                .returns(ReturnsResult)
                        });
                    },
                }
            }
        }

        let results = buffer.run().await;

        for (index, (status, result)) in results.iter().enumerate() {
            if !status == 0u64 {
                println!("perform 'add_queued_call' failed with error code {status}");
                continue;
            }
            println!(
                "successfully performed action {index} 'add_queued_call' with result {result:?}"
            );
        }
    }

    pub async fn call_root(&mut self, call_state: &CallState) {
        let root_addr = {
            let root_addr_ref = call_state.root.borrow();
            root_addr_ref.address.clone().unwrap()
        };

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .gas(70_000_000u64)
            .to(&root_addr)
            .typed(forwarder_queue_proxy::ForwarderQueueProxy)
            .forward_queued_calls()
            .run()
            .await;

        println!("successfully called root");
    }
}
