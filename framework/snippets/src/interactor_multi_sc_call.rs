use crate::{
    interactor_multi_sc_process::{update_nonces_and_sign_tx, SenderSet, Txs},
    Interactor,
};

use multiversx_sc_scenario::{
    mandos_system::ScenarioRunner,
    scenario_model::{ScCallStep, ScCallStepBuffer, TxResponse},
};
use multiversx_sdk::data::transaction::Transaction;

impl Interactor {
    pub async fn multiple_sc_calls_raw_results(&mut self, mut buffer: ScCallStepBuffer<'_>) {
        for step in &buffer.refs {
            self.pre_runners.run_sc_call_step(&**step);
        }

        let senders = retrieve_senders(buffer.refs.as_slice());
        self.recall_senders_nonce(senders).await;

        let txs = self.retrieve_txs(buffer.refs.as_slice());
        let results = self.process_txs(txs).await;

        for (i, sc_call_step) in buffer.refs.iter_mut().enumerate() {
            sc_call_step.response = Some(TxResponse::new(results.get(i).unwrap().clone()));
        }

        for step in &buffer.refs {
            self.post_runners.run_sc_call_step(&**step);
        }
    }

    fn retrieve_txs(&mut self, sc_call_steps: &[&mut ScCallStep]) -> Vec<Transaction> {
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

fn retrieve_senders(sc_call_steps: &[&mut ScCallStep]) -> SenderSet {
    let mut senders = SenderSet::new();

    for sc_call_step in sc_call_steps {
        let sender_address = &sc_call_step.tx.from.value;
        senders.insert(sender_address.clone());
    }
    senders
}
