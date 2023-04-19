use std::{cell::RefCell, rc::Rc};

use crate::{
    call_tree::{CallState, ForwarderQueueTarget, VaultTarget},
    comp_interact_controller::ComposabilityInteract,
};

use forwarder_queue::ProxyTrait as _;
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
    StepBuffer,
};
use vault::ProxyTrait as _;

impl ComposabilityInteract {
    pub async fn deploy_call_tree_contracts(&mut self, call_state: &CallState) {
        self.deploy_vault(&call_state.vaults).await;
        self.deploy_forwarder_queue(&call_state.forwarders).await;
    }

    pub async fn deploy_vault(&mut self, vaults: &Vec<Rc<RefCell<VaultTarget>>>) {
        let mut steps = Vec::new();
        for _ in vaults.iter() {
            let typed_sc_deploy = self
                .state
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
                .expect(TxExpect::ok());
            steps.push(typed_sc_deploy);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_deploy_vec(&mut steps))
            .await;

        let mut vault_iter = vaults.iter();
        for step in steps.iter() {
            let result = step.response().new_deployed_address();
            if result.is_err() {
                println!("deploy failed: {}", result.err().unwrap());
                return;
            }

            let new_address_bech32 = bech32::encode(result.as_ref().unwrap());
            let rc_vault = vault_iter.next().unwrap();
            let mut vault = rc_vault.borrow_mut();
            println!(
                "New vault {0} deployed address: {1}",
                vault.name, new_address_bech32
            );

            vault.address = Some(result.unwrap());
        }
    }

    pub async fn deploy_forwarder_queue(
        &mut self,
        forwarders: &Vec<Rc<RefCell<ForwarderQueueTarget>>>,
    ) {
        let mut steps = Vec::new();

        for _ in forwarders.iter() {
            let typed_sc_deploy = self
                .state
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
                .expect(TxExpect::ok());
            steps.push(typed_sc_deploy);
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_deploy_vec(&mut steps))
            .await;

        let mut fwd_iter = forwarders.iter();
        for step in steps.iter() {
            let result = step.response().new_deployed_address();
            if result.is_err() {
                println!("deploy failed: {}", result.err().unwrap());
                return;
            }

            let new_address_bech32 = bech32::encode(result.as_ref().unwrap());
            let rc_fwd = fwd_iter.next().unwrap();
            let mut fwd = rc_fwd.borrow_mut();
            println!(
                "New vault {0} deployed address: {1}",
                fwd.name, new_address_bech32
            );

            fwd.address = Some(result.unwrap());
        }
    }
}
