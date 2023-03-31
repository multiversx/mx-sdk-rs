use crate::{
    interactor_multi_sc_process::{update_nonces_and_sign_tx, SenderSet, Txs},
    Interactor, InteractorResult,
};

use multiversx_sc_scenario::{mandos_system::ScenarioRunner, scenario_model::ScDeployStep};
use multiversx_sdk::data::transaction::Transaction;

impl Interactor {
    pub async fn multiple_sc_deploy_results(
        &mut self,
        sc_deploy_steps: &[ScDeployStep],
    ) -> Vec<InteractorResult<()>> {
        self.pre_runners.run_multi_sc_deploy_step(sc_deploy_steps);

        let senders = retrieve_senders_deploy(sc_deploy_steps);
        self.recall_senders_nonce(senders).await;

        let txs = self.retrieve_txs_deploy(sc_deploy_steps);
        let results = self.process_txs(txs).await;

        self.post_runners.run_multi_sc_deploy_step(sc_deploy_steps);

        results.into_iter().map(InteractorResult::new).collect()
    }

    fn retrieve_txs_deploy(&mut self, sc_deploy_steps: &[ScDeployStep]) -> Vec<Transaction> {
        let mut txs = Txs::new();

        for sc_deploy_step in sc_deploy_steps {
            let mut transaction = self.sc_deploy_to_blockchain_tx(sc_deploy_step);
            let sender_address = &sc_deploy_step.tx.from.value;
            let sender = self
                .sender_map
                .get_mut(sender_address)
                .expect("sender not registered");

            update_nonces_and_sign_tx(&mut transaction, sender);
            txs.push(transaction);
        }
        txs
    }
}

fn retrieve_senders_deploy(sc_deploy_steps: &[ScDeployStep]) -> SenderSet {
    let mut senders = SenderSet::new();

    for sc_deploy_step in sc_deploy_steps {
        let sender_address = &sc_deploy_step.tx.from.value;
        senders.insert(sender_address.clone());
    }
    senders
}
