use std::{cell::RefCell, rc::Rc};

use crate::{
    call_tree::{CallState, ForwarderQueueTarget, VaultTarget},
    comp_interact_controller::ComposabilityInteract,
};

use forwarder_raw::ProxyTrait as _;
use multiversx_sc_snippets::{
    multiversx_sc::{
        codec::multi_types::OptionalValue,
        types::{BoxedBytes, CodeMetadata},
    },
    multiversx_sc_scenario::{
        bech32,
        scenario_format::interpret_trait::InterpreterContext,
        scenario_model::{IntoBlockchainCall, TxExpect},
    },
};
use promises_features::ProxyTrait as _;
use vault::ProxyTrait as _;

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
                    .default_forwarder_raw_address()
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
}
