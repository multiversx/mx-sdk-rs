use std::{cell::RefCell, rc::Rc};

use forwarder_queue::QueuedCallType;
use multiversx_sc_snippets::{
    multiversx_sc::types::{EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, MultiValueEncoded},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        num_bigint::BigUint,
        scenario_model::{ScCallStep, TxExpect},
    },
    StepBuffer,
};

use crate::{
    call_tree::{CallNode, CallState, ForwarderQueueTarget},
    comp_interact_controller::ComposabilityInteract,
};
use forwarder_queue::ProxyTrait;

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
        let mut steps = Vec::new();

        for fwd_rc in forwarders {
            let (fwd_name, fwd_children) = {
                let fwd = fwd_rc.borrow();
                (fwd.name.clone(), fwd.children.clone())
            };
            let fwd_addr = {
                let fwd = fwd_rc.borrow();
                fwd.address.clone().unwrap()
            };
            let fwd_addr_bech32 = bech32::encode(&fwd_addr);
            let fwd_addr_expr = format!("bech32:{fwd_addr_bech32}");

            for child in &fwd_children {
                match child {
                    CallNode::ForwarderQueue(child_fwd_rc) => {
                        // forward_queued_calls to ForwarderQueue's children
                        let child_fwd_addr = {
                            let child_fwd = (*child_fwd_rc).borrow();
                            println!("child_name: {}, parent_name: {}", child_fwd.name, &fwd_name);
                            child_fwd.address.clone().unwrap()
                        };

                        let typed_sc_call = ScCallStep::new()
                            .call(
                                self.state
                                    .forwarder_queue_from_addr(&fwd_addr_expr)
                                    .add_queued_call(
                                        call_type.clone(),
                                        child_fwd_addr,
                                        DEFAULT_GAS_LIMIT,
                                        FORWARD_QUEUED_CALLS_ENDPOINT,
                                        MultiValueEncoded::<StaticApi, _>::new(),
                                    )
                                    .with_egld_or_single_esdt_transfer(
                                        EgldOrEsdtTokenPayment::new(
                                            payment_token.clone(),
                                            payment_nonce,
                                            payment_amount.clone().into(),
                                        ),
                                    ),
                            )
                            .from(&self.wallet_address)
                            .gas_limit("70,000,000");

                        steps.push(typed_sc_call);
                    },
                    CallNode::Vault(vault_rc) => {
                        // Call Vault
                        let vault_addr = {
                            let vault = (*vault_rc).borrow();
                            println!("child_name: {}, parent_name: {}", vault.name, &fwd_name);
                            vault.address.clone().unwrap()
                        };

                        let typed_sc_call = ScCallStep::new()
                            .call(
                                self.state
                                    .forwarder_queue_from_addr(&fwd_addr_expr)
                                    .add_queued_call(
                                        call_type.clone(),
                                        vault_addr,
                                        DEFAULT_GAS_LIMIT,
                                        endpoint_name,
                                        MultiValueEncoded::<StaticApi, _>::new(),
                                    )
                                    .with_egld_or_single_esdt_transfer(
                                        EgldOrEsdtTokenPayment::new(
                                            payment_token.clone(),
                                            payment_nonce,
                                            payment_amount.clone().into(),
                                        ),
                                    ),
                            )
                            .from(&self.wallet_address)
                            .gas_limit("70,000,000");

                        steps.push(typed_sc_call);
                    },
                }
            }
        }
        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_call_vec(&mut steps))
            .await;

        for step in steps.iter() {
            if !step.response().is_success() {
                println!(
                    "perform 'add_queued_call' failed with: {}",
                    step.response().tx_error
                );
                continue;
            }
            println!("successfully performed action 'add_queued_call'");
        }
    }

    pub async fn call_root(&mut self, call_state: &CallState) {
        let root_addr = {
            let root_addr_ref = call_state.root.borrow();
            root_addr_ref.address.clone().unwrap()
        };
        let root_addr_bech32 = bech32::encode(&root_addr);
        let root_addr_expr = format!("bech32:{root_addr_bech32}");

        self.interactor
            .sc_call(
                ScCallStep::new()
                    .call(
                        self.state
                            .forwarder_queue_from_addr(&root_addr_expr)
                            .forward_queued_calls(),
                    )
                    .from(&self.wallet_address)
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok().additional_error_message("calling root failed with: ")),
            )
            .await;

        println!("successfully called root");
    }
}
