use crate::{interactor_multi_sc_process::{update_nonces_and_sign_tx, Txs, SenderSet}, Interactor, InteractorResult};

use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner, multiversx_sc::codec::multi_types::IgnoreValue,
    scenario_model::ScCallStep,
};
use multiversx_sdk::data::transaction::Transaction;

impl Interactor {
    pub async fn multiple_sc_calls_raw_results(
        &mut self,
        sc_call_steps: &[ScCallStep],
    ) -> Vec<InteractorResult<IgnoreValue>> {
        self.pre_runners.run_multi_sc_call_step(sc_call_steps);

        let senders = retrieve_senders(sc_call_steps);
        self.recall_senders_nonce(senders).await;

        let txs = self.retrieve_txs(sc_call_steps);
        let results = self.process_txs(txs).await;

        self.post_runners.run_multi_sc_call_step(sc_call_steps);

        results.into_iter().map(InteractorResult::new).collect()
    }

    fn retrieve_txs(&mut self, sc_call_steps: &[ScCallStep]) -> Vec<Transaction> {
        let mut txs = Txs::new();

        for sc_call_step in sc_call_steps {
            let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
            let sender_address = &sc_call_step.tx.from.value;
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

fn retrieve_senders(sc_call_steps: &[ScCallStep]) -> SenderSet {
    let mut senders = SenderSet::new();

    for sc_call_step in sc_call_steps {
        let sender_address = &sc_call_step.tx.from.value;
        senders.insert(sender_address.clone());
    }
    senders
}
