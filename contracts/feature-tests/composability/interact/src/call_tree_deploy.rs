use std::{cell::RefCell, rc::Rc};

use crate::{
    call_tree::{CallNode, CallState, ForwarderQueueTarget, VaultTarget},
    comp_interact_controller::ComposabilityInteract,
};

use forwarder_queue::{ProxyTrait as _, QueuedCallType};
use multiversx_sc_snippets::{
    multiversx_sc::{
        codec::multi_types::OptionalValue,
        types::{Address, BoxedBytes, CodeMetadata, EgldOrEsdtTokenIdentifier},
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

const ADD_QUEUED_CALL_ENDPOINT: &str = "add_queued_call";

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
        let mut vault = vault_rc.borrow_mut();
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
        println!("{} address: {new_address_bech32}", &vault.name);

        vault.address = Some(new_address);
    }

    pub async fn deploy_forwarder_queue(&mut self, fwd_rc: Rc<RefCell<ForwarderQueueTarget>>) {
        let mut fwd = fwd_rc.borrow_mut();
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
        println!("{} address: {new_address_bech32}", &fwd.name);

        fwd.address = Some(new_address);
    }

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

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_promises_address(&new_address_expr);
    }

    pub async fn add_queued_call(
        &mut self,
        fwd_rc: Rc<RefCell<ForwarderQueueTarget>>,
        call_type: QueuedCallType,
        to: Address,
        endpoint_name: &str,
        payment_token: EgldOrEsdtTokenIdentifier<DebugApi>,
        payment_nonce: u64,
        payment_amount: u64,
    ) {
        let fwd = fwd_rc.borrow_mut();
        let fwd_addr = fwd.address.clone().unwrap();

        let _ = self
            .interactor
            .sc_call(
                self.state
                    .forwarder_queue_from_addr(&bech32::encode(&fwd_addr))
                    .add_queued_call(
                        call_type,
                        to,
                        endpoint_name,
                        payment_token,
                        payment_nonce,
                        payment_amount,
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
        payment_token: EgldOrEsdtTokenIdentifier<DebugApi>,
        payment_nonce: u64,
        payment_amount: u64,
    ) {
        let fwd = fwd_rc.borrow();
        for child in &fwd.children {
            match child {
                CallNode::ForwarderQueue(child_fwd_rc) => {
                    // forward_queued_calls to ForwarderQueue's children
                    let child_fwd = (*child_fwd_rc).borrow();
                    let child_fwd_addr = child_fwd.address.clone().unwrap();

                    self.add_queued_call(
                        fwd_rc.clone(),
                        call_type.clone(),
                        child_fwd_addr,
                        ADD_QUEUED_CALL_ENDPOINT,
                        payment_token.clone(),
                        payment_nonce,
                        payment_amount,
                    )
                    .await;
                },
                CallNode::Vault(vault_rc) => {
                    // Call Vault
                    let vault = (*vault_rc).borrow_mut();
                    let vault_addr = vault.address.clone().unwrap();
                    self.add_queued_call(
                        fwd_rc.clone(),
                        call_type.clone(),
                        vault_addr,
                        endpoint_name,
                        payment_token.clone(),
                        payment_nonce,
                        payment_amount,
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
        payment_token: EgldOrEsdtTokenIdentifier<DebugApi>,
        payment_nonce: u64,
        payment_amount: u64,
    ) {
        for fwd_rc in &call_state.forwarders {
            self.add_queued_calls_to_children(
                fwd_rc.clone(),
                call_type.clone(),
                endpoint_name,
                payment_token.clone(),
                payment_nonce,
                payment_amount,
            )
            .await;
        }
    }
}
