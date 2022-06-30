use crate::{mandos_to_erdrs_address, Interactor};
use elrond_sdk_erdrs::data::{
    address::Address as ErdrsAddress,
    transaction::{Transaction, TransactionOnNetwork},
};
use elrond_wasm_debug::{
    bech32,
    elrond_wasm::{
        elrond_codec::{CodecFrom, TopEncodeMulti},
        types::Address,
    },
    mandos_system::model::{ScDeployStep, TypedScDeploy},
};
use log::info;

const DEPLOY_RECEIVER: [u8; 32] = [0u8; 32];
const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";

impl Interactor {
    fn sc_deploy_to_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.egld_value.value.to_string(),
            sender: mandos_to_erdrs_address(&sc_deploy_step.tx.from),
            receiver: ErdrsAddress::from_bytes(DEPLOY_RECEIVER),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64::encode(sc_deploy_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub async fn send_sc_deploy(&mut self, sc_call_step: ScDeployStep) -> String {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.sc_deploy_to_tx(&sc_call_step);
        transaction.nonce = self.recall_nonce(sender_address).await;
        self.sign_tx(sender_address, &mut transaction);
        self.proxy.send_transaction(&transaction).await.unwrap()
    }

    pub async fn sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_call_step: ScDeployStep = typed_sc_call.into();
        let tx_hash = self.send_sc_deploy(sc_call_step).await;
        println!("deploy tx hash: {}", tx_hash);
        info!("deploy tx hash: {}", tx_hash);
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        let new_address = extract_new_deployed_address(&tx);
        let call_result = self.extract_sc_call_result(tx);
        (new_address, call_result)
    }
}

fn extract_new_deployed_address(tx: &TransactionOnNetwork) -> Address {
    let logs = tx.logs.as_ref().expect("logs expected after deploy");
    let first_event_log = logs
        .events
        .get(0)
        .expect("at least one log expected after deploy");
    assert_eq!(
        first_event_log.identifier.as_str(),
        LOG_IDENTIFIER_SC_DEPLOY,
        "First log after deploy expected to be 'SCDeploy'"
    );
    let topics = first_event_log.topics.as_ref().expect("missing topics");
    assert_eq!(topics.len(), 2, "`SCDeploy` is expected to have 2 topics");
    let address_raw = base64::decode(topics.get(0).unwrap()).unwrap();
    let address = Address::from_slice(address_raw.as_slice());
    info!("new address: {}", bech32::encode(&address));
    address
}
