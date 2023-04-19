use std::{cell::RefCell, rc::Rc};

use forwarder_queue::QueuedCallType;
use multiversx_sc_snippets::{
    multiversx_sc::types::{EgldOrEsdtTokenIdentifier, MultiValueEncoded},
    multiversx_sc_scenario::{bech32, scenario_model::TxExpect, DebugApi},
    StepBuffer,
};

use crate::{
    call_tree::{CallNode, CallState, ForwarderQueueTarget},
    comp_interact_controller::ComposabilityInteract,
};
use forwarder_queue::ProxyTrait;
use multiversx_sc_snippets::multiversx_sc_scenario::scenario_model::IntoBlockchainCall;

const FORWARD_QUEUED_CALLS_ENDPOINT: &str = "forward_queued_calls";
const DEFAULT_GAS_LIMIT: u64 = 10_000_000;

impl ComposabilityInteract {
    // pub async fn add_queued_call(
    //     &mut self,
    //     fwd_rc: Rc<RefCell<ForwarderQueueTarget>>,
    //     call_type: QueuedCallType,
    //     to: Address,
    //     endpoint_name: &str,
    // ) {
    //     let fwd_addr = {
    //         let fwd = fwd_rc.borrow();
    //         fwd.address.clone().unwrap()
    //     };

    //     let fwd_addr_bech32 = bech32::encode(&fwd_addr);
    //     let fwd_addr_expr = format!("bech32:{fwd_addr_bech32}");

    //     let mut typed_sc_call = self
    //         .state
    //         .forwarder_queue_from_addr(&fwd_addr_expr)
    //         .add_queued_call(
    //             call_type,
    //             to,
    //             DEFAULT_GAS_LIMIT,
    //             endpoint_name,
    //             MultiValueEncoded::<DebugApi, _>::new(),
    //         )
    //         .into_blockchain_call()
    //         .from(&self.wallet_address)
    //         .gas_limit("70,000,000")
    //         .expect(TxExpect::ok());

    //     self.interactor.sc_call(&mut typed_sc_call).await;
    // }

    pub async fn add_queued_calls_to_children(
        &mut self,
        forwarders: &Vec<Rc<RefCell<ForwarderQueueTarget>>>,
        call_type: QueuedCallType,
        endpoint_name: &str,
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

                        // self.add_queued_call(
                        //     fwd_rc.clone(),
                        //     call_type.clone(),
                        //     child_fwd_addr,
                        //     FORWARD_QUEUED_CALLS_ENDPOINT,
                        // )
                        // .await;

                        let typed_sc_call = self
                            .state
                            .forwarder_queue_from_addr(&fwd_addr_expr)
                            .add_queued_call(
                                call_type.clone(),
                                child_fwd_addr,
                                DEFAULT_GAS_LIMIT,
                                FORWARD_QUEUED_CALLS_ENDPOINT,
                                MultiValueEncoded::<DebugApi, _>::new(),
                            )
                            .into_blockchain_call()
                            .from(&self.wallet_address)
                            .gas_limit("70,000,000")
                            .expect(TxExpect::ok());

                        steps.push(typed_sc_call);
                    },
                    CallNode::Vault(vault_rc) => {
                        // Call Vault
                        let vault_addr = {
                            let vault = (*vault_rc).borrow();
                            println!("child_name: {}, parent_name: {}", vault.name, &fwd_name);
                            vault.address.clone().unwrap()
                        };

                        // self.add_queued_call(
                        //     fwd_rc.clone(),
                        //     call_type.clone(),
                        //     vault_addr,
                        //     endpoint_name,
                        // )
                        // .await;

                        let typed_sc_call = self
                            .state
                            .forwarder_queue_from_addr(&fwd_addr_expr)
                            .add_queued_call(
                                call_type.clone(),
                                vault_addr,
                                DEFAULT_GAS_LIMIT,
                                endpoint_name,
                                MultiValueEncoded::<DebugApi, _>::new(),
                            )
                            .into_blockchain_call()
                            .from(&self.wallet_address)
                            .gas_limit("70,000,000")
                            .expect(TxExpect::ok());

                        steps.push(typed_sc_call);
                    },
                }
            }
        }
        println!("!!! Number of steps {}", steps.len());
        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_call_vec(&mut steps))
            .await;

        for step in steps.iter() {
            let result = step.response().handle_signal_error_event();
            if result.is_err() {
                println!(
                    "perform 'add_queued_call' failed with: {}",
                    result.err().unwrap()
                );
                continue;
            }
            println!("successfully performed action 'add_queued_call'");
        }
    }

    // pub async fn add_calls_to_all_fwds(
    //     &mut self,
    //     call_state: &CallState,
    //     call_type: QueuedCallType,
    //     endpoint_name: &str,
    // ) {
    //     for fwd_rc in &call_state.forwarders {
    //         self.add_queued_calls_to_children(fwd_rc.clone(), call_type.clone(), endpoint_name)
    //             .await;
    //     }
    // }

    pub async fn call_root(
        &mut self,
        call_state: &CallState,
        payment_token: EgldOrEsdtTokenIdentifier<DebugApi>,
        payment_nonce: u64,
        payment_amount: u64,
    ) {
        let root_addr = {
            let root_addr_ref = call_state.root.borrow();
            root_addr_ref.address.clone().unwrap()
        };
        let root_addr_bech32 = bech32::encode(&root_addr);
        let root_addr_expr = format!("bech32:{root_addr_bech32}");

        let sc_call_root_step = self
            .state
            .forwarder_queue_from_addr(&root_addr_expr)
            .forward_queued_calls()
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("70,000,000")
            .expect(TxExpect::ok());

        if payment_token.is_esdt() {
            let token_id_hex = payment_token.unwrap_esdt().to_string();
            let token_id = format!("str:{token_id_hex}");

            print!("token_id = {token_id}");
            let mut typed_sc_call =
                sc_call_root_step.esdt_transfer(token_id, payment_nonce, payment_amount);
            self.interactor.sc_call(&mut typed_sc_call).await;
        } else {
            let mut typed_sc_call = sc_call_root_step.egld_value(payment_amount);
            self.interactor.sc_call(&mut typed_sc_call).await;
        }
    }
}
