use std::{cell::RefCell, rc::Rc};

use crate::{
    call_tree::{CallNode, CallState, ForwarderQueueTarget, VaultTarget},
    comp_interact_controller::ComposabilityInteract,
};

use forwarder_queue::{ProxyTrait as _, QueuedCallType};
use multiversx_sc_snippets::{
    multiversx_sc::{
        codec::multi_types::OptionalValue,
        types::{Address, BoxedBytes, CodeMetadata, EgldOrEsdtTokenIdentifier, MultiValueEncoded},
    },
    multiversx_sc_scenario::{
        bech32,
        scenario_format::interpret_trait::InterpreterContext,
        scenario_model::{IntoBlockchainCall, TxExpect},
        DebugApi,
    },
};
use promises_features::ProxyTrait as _;
use vault::ProxyTrait as _;

const FORWARD_QUEUED_CALLS_ENDPOINT: &str = "forward_queued_calls";

impl ComposabilityInteract {
    pub async fn deploy_call_tree_contracts(&mut self, call_state: &CallState) {
        for vault_rc in &call_state.vaults {
            self.deploy_vault(vault_rc.clone()).await;
        }
        for fwd_rc in &call_state.forwarders {
            self.deploy_forwarder_queue(fwd_rc.clone()).await;
        }
    }

    pub async fn deploy_vault(&mut self, vault_rc: Rc<RefCell<VaultTarget>>) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<OptionalValue<BoxedBytes>> =
            self.interactor
                .sc_deploy(
                    self.state
                        .default_vault_address()
                        .init(OptionalValue::<BoxedBytes>::None)
                        .into_blockchain_call()
                        .from(&self.wallet_address)
                        .code_metadata(CodeMetadata::all())
                        .contract_code(
                            "file:../vault/output/vault.wasm",
                            &InterpreterContext::default(),
                        )
                        .gas_limit("70,000,000")
                        .expect(TxExpect::ok()),
                )
                .await;

        let result = deploy_result.new_deployed_address();
        let new_address = result.expect("deploy failed");
        let new_address_bech32 = bech32::encode(&new_address);

        let mut vault = vault_rc.borrow_mut();
        println!("{} address: {new_address_bech32}", &vault.name);
        vault.address = Some(new_address);
    }

    pub async fn deploy_forwarder_queue(&mut self, fwd_rc: Rc<RefCell<ForwarderQueueTarget>>) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_forwarder_queue_address()
                    .init()
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../forwarder-queue/output/forwarder-queue.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;

        let result = deploy_result.new_deployed_address();
        let new_address = result.expect("deploy failed");
        let new_address_bech32 = bech32::encode(&new_address);

        let mut fwd = fwd_rc.borrow_mut();
        println!("{} address: {new_address_bech32}", &fwd.name);
        fwd.address = Some(new_address);
    }

    #[allow(dead_code)]
    pub async fn deploy_promises(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_promises_address()
                    .init()
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../../promises/output/promises.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;

        let result = deploy_result.new_deployed_address();
        if result.is_err() {
            println!("deploy failed: {}", result.err().unwrap());
            return;
        }

        let new_address_bech32 = bech32::encode(&result.unwrap());
        println!("Promises address: {new_address_bech32}");

        // self.state.set_promises_address(&new_address_expr);
    }

    pub async fn add_queued_call(
        &mut self,
        fwd_rc: Rc<RefCell<ForwarderQueueTarget>>,
        call_type: QueuedCallType,
        to: Address,
        endpoint_name: &str,
    ) {
        let fwd_addr = {
            let fwd = fwd_rc.borrow();
            fwd.address.clone().unwrap()
        };

        let fwd_addr_bech32 = bech32::encode(&fwd_addr);
        let fwd_addr_expr = format!("bech32:{fwd_addr_bech32}");

        let _ = self
            .interactor
            .sc_call(
                self.state
                    .forwarder_queue_from_addr(&fwd_addr_expr)
                    .add_queued_call(
                        call_type,
                        to,
                        endpoint_name,
                        MultiValueEncoded::<DebugApi, _>::new(),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;
    }

    pub async fn add_queued_calls_to_children(
        &mut self,
        fwd_rc: Rc<RefCell<ForwarderQueueTarget>>,
        call_type: QueuedCallType,
        endpoint_name: &str,
    ) {
        let (fwd_name, fwd_children) = {
            let fwd = fwd_rc.borrow();
            (fwd.name.clone(), fwd.children.clone())
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

                    self.add_queued_call(
                        fwd_rc.clone(),
                        call_type.clone(),
                        child_fwd_addr,
                        FORWARD_QUEUED_CALLS_ENDPOINT,
                    )
                    .await;
                },
                CallNode::Vault(vault_rc) => {
                    // Call Vault
                    let vault_addr = {
                        let vault = (*vault_rc).borrow();
                        println!("child_name: {}, parent_name: {}", vault.name, &fwd_name);
                        vault.address.clone().unwrap()
                    };

                    self.add_queued_call(
                        fwd_rc.clone(),
                        call_type.clone(),
                        vault_addr,
                        endpoint_name,
                    )
                    .await;
                },
            }
        }
    }

    pub async fn add_calls_to_all_fwds(
        &mut self,
        call_state: &CallState,
        call_type: QueuedCallType,
        endpoint_name: &str,
    ) {
        for fwd_rc in &call_state.forwarders {
            self.add_queued_calls_to_children(fwd_rc.clone(), call_type.clone(), endpoint_name)
                .await;
        }
    }

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
            self.interactor
                .sc_call(sc_call_root_step.esdt_transfer(token_id, payment_nonce, payment_amount))
                .await;
        } else {
            self.interactor
                .sc_call(sc_call_root_step.egld_value(payment_amount))
                .await;
        }
    }
}
