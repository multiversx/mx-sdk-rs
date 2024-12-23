use crate::{
    call_tree::CallState, comp_interact_controller::ComposabilityInteract, forwarder_queue_proxy,
    vault_proxy,
};

use multiversx_sc_snippets::imports::*;

impl ComposabilityInteract {
    pub async fn deploy_call_tree_contracts(
        &mut self,
        call_state: &CallState,
    ) -> (Vec<Bech32Address>, Vec<Bech32Address>) {
        let vault_deploy_addresses = self.typed_sc_deploy_vault(call_state).await;
        let forwarder_deploy_addresses = self.typed_sc_deploy_forwarder_queue(call_state).await;

        let mut vault_iter = call_state.vaults.iter();
        for address in vault_deploy_addresses.iter() {
            let rc_vault = vault_iter.next().unwrap();
            let mut vault = rc_vault.borrow_mut();
            println!("New vault {0} deployed address: {1}", vault.name, address);

            vault.address = Some(address.to_address());
        }

        let mut fwd_iter = call_state.forwarders.iter();
        for address in forwarder_deploy_addresses.iter() {
            let rc_fwd = fwd_iter.next().unwrap();
            let mut fwd = rc_fwd.borrow_mut();
            println!("New forwarder {0} deployed address: {1}", fwd.name, address);

            fwd.address = Some(address.to_address());
        }

        (vault_deploy_addresses, forwarder_deploy_addresses)
    }

    pub async fn typed_sc_deploy_vault(&mut self, call_state: &CallState) -> Vec<Bech32Address> {
        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in call_state.vaults.iter() {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(vault_proxy::VaultProxy)
                    .init(OptionalValue::<BoxedBytes>::None)
                    .code(&self.vault_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewBech32Address)
            });
        }

        buffer.run().await
    }

    pub async fn typed_sc_deploy_forwarder_queue(
        &mut self,
        call_state: &CallState,
    ) -> Vec<Bech32Address> {
        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in call_state.forwarders.iter() {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                    .init()
                    .code(&self.forw_queue_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewBech32Address)
            });
        }

        buffer.run().await
    }
}
