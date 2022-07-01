use crate::{address_h256_to_erdrs, mandos_to_erdrs_address, Interactor};
use elrond_sdk_erdrs::data::transaction::Transaction;
use elrond_wasm_debug::{
    elrond_wasm::{
        elrond_codec::{CodecFrom, TopEncodeMulti},
        types::{Address, ContractCall},
    },
    mandos_system::model::{ScCallStep, TransferStep, TxCall, TypedScCall},
    DebugApi,
};
use log::info;

fn contract_call_to_tx_data(contract_call: &ContractCall<DebugApi, ()>) -> String {
    let mut result =
        String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec()).unwrap();
    for argument in contract_call.arg_buffer.raw_arg_iter() {
        result.push('@');
        result.push_str(hex::encode(argument.to_boxed_bytes().as_slice()).as_str());
    }
    result
}

impl Interactor {
    fn tx_call_to_blockchain_tx(&self, tx_call: &TxCall) -> Transaction {
        let contract_call = tx_call.to_contract_call().convert_to_esdt_transfer_call();
        let contract_call_tx_data = contract_call_to_tx_data(&contract_call);
        let data = if contract_call_tx_data.is_empty() {
            None
        } else {
            Some(base64::encode(contract_call_tx_data))
        };

        Transaction {
            nonce: 0,
            value: contract_call.egld_payment.to_alloc().to_string(),
            sender: mandos_to_erdrs_address(&tx_call.from),
            receiver: address_h256_to_erdrs(&contract_call.to.to_address()),
            gas_price: self.network_config.min_gas_price,
            gas_limit: tx_call.gas_limit.value,
            data,
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub(crate) fn sign_tx(&self, sender_address: &Address, transaction: &mut Transaction) {
        let wallet = self
            .signing_wallets
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        let signature = wallet.sign_tx(transaction);
        transaction.signature = Some(hex::encode(signature));
        info!("transaction {:#?}", transaction);
    }

    pub async fn send_sc_call(&mut self, sc_call_step: ScCallStep) -> String {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
        transaction.nonce = self.recall_nonce(sender_address).await;
        self.sign_tx(sender_address, &mut transaction);
        self.proxy.send_transaction(&transaction).await.unwrap()
    }

    pub async fn sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_call_step: ScCallStep = typed_sc_call.into();
        let tx_hash = self.send_sc_call(sc_call_step).await;
        println!("sc call tx hash: {}", tx_hash);
        info!("sc call tx hash: {}", tx_hash);
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        self.extract_sc_call_result(tx)
    }

    pub async fn transfer(&mut self, transfer_step: TransferStep) -> String {
        let sender_address = &transfer_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&transfer_step.tx.to_tx_call());
        transaction.nonce = self.recall_nonce(sender_address).await;
        self.sign_tx(sender_address, &mut transaction);
        self.proxy.send_transaction(&transaction).await.unwrap()
    }
}
