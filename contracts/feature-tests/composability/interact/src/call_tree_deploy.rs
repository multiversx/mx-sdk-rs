use crate::{call_tree::CallState, comp_interact_controller::ComposabilityInteract};

use forwarder_queue::ProxyTrait as _;
use multiversx_sc_snippets::{
    multiversx_sc::{
        codec::multi_types::OptionalValue,
        types::{BoxedBytes, CodeMetadata, ManagedBuffer},
    },
    multiversx_sc_scenario::{
        bech32,
        scenario_format::interpret_trait::InterpreterContext,
        scenario_model::{IntoBlockchainCall, TxExpect, TypedScDeploy},
        DebugApi,
    },
    StepBuffer,
};
use vault::ProxyTrait as _;

impl ComposabilityInteract {
    pub async fn deploy_call_tree_contracts(&mut self, call_state: &CallState) {
        let mut typed_vault_deploys = self.typed_sc_deploy_vault(call_state).await;
        let mut typed_forwarder_deploys = self.typed_sc_deploy_forwarder_queue(call_state).await;

        let mut steps = Vec::new();
        for typed_sc_deploy in &mut typed_vault_deploys {
            steps.push(typed_sc_deploy.as_mut());
        }
        for typed_sc_deploy in &mut typed_forwarder_deploys {
            steps.push(typed_sc_deploy.as_mut());
        }

        self.interactor
            .multi_sc_exec(StepBuffer::from_sc_deploy_vec(&mut steps))
            .await;

        let mut vault_iter = call_state.vaults.iter();
        for step in typed_vault_deploys.iter() {
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

        let mut fwd_iter = call_state.forwarders.iter();
        for step in typed_forwarder_deploys.iter() {
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

    pub async fn typed_sc_deploy_vault(
        &mut self,
        call_state: &CallState,
    ) -> Vec<TypedScDeploy<OptionalValue<ManagedBuffer<DebugApi>>>> {
        let mut typed_vault_deploys = Vec::new();
        for _ in call_state.vaults.iter() {
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

            typed_vault_deploys.push(typed_sc_deploy);
        }
        typed_vault_deploys
    }

    pub async fn typed_sc_deploy_forwarder_queue(
        &mut self,
        call_state: &CallState,
    ) -> Vec<TypedScDeploy<()>> {
        let mut typed_forwarder_deploys = Vec::new();

        for _ in call_state.forwarders.iter() {
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

            typed_forwarder_deploys.push(typed_sc_deploy);
        }
        typed_forwarder_deploys
    }
}
