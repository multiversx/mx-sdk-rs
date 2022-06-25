use elrond_sdk_erdrs::{
    blockchain::rpc::ElrondProxy,
    data::{
        address::Address as ErdrsAddress, network_config::NetworkConfig, transaction::Transaction,
        vm::VmValueRequest,
    },
    interactors::wallet::Wallet,
};
use elrond_wasm_debug::{
    elrond_wasm::{
        elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
        types::{Address, ContractCall},
    },
    mandos_system::model::{AddressValue, ScCallStep},
    DebugApi, HashMap,
};

pub struct Interactor {
    pub proxy: ElrondProxy,
    pub network_config: NetworkConfig,
    pub signing_wallets: HashMap<Address, Wallet>,
}

impl Interactor {
    pub async fn new(gateway_url: &str) -> Self {
        let proxy = ElrondProxy::new(gateway_url.to_string());
        let network_config = proxy.get_network_config().await.unwrap();
        Self {
            proxy,
            network_config,
            signing_wallets: HashMap::new(),
        }
    }

    pub fn register_wallet(&mut self, wallet: Wallet) -> Address {
        let address = erdrs_address_to_h256(wallet.address());
        self.signing_wallets.insert(address.clone(), wallet);
        address
    }

    fn sc_call_to_tx(&self, sc_call_step: &ScCallStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_call_step.tx.egld_value.value.to_string(),
            sender: mandos_to_erdrs_address(&sc_call_step.tx.from),
            receiver: mandos_to_erdrs_address(&sc_call_step.tx.to),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_call_step.tx.gas_limit.value,
            data: Some(base64::encode(sc_call_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub async fn mandos_sc_call(&mut self, sc_call_step: ScCallStep) -> &mut Self {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.sc_call_to_tx(&sc_call_step);
        transaction.nonce = self.recall_nonce(sender_address).await;

        let wallet = self
            .signing_wallets
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        let signature = wallet.sign_tx(&transaction);
        transaction.signature = Some(hex::encode(signature));
        println!("transaction {:#?}", transaction);

        let tx_hash = self.proxy.send_transaction(&transaction).await.unwrap();
        println!("tx_hash {}", tx_hash);

        self
    }

    pub async fn vm_query<OriginalResult, RequestedResult>(
        &mut self,
        contract_call: ContractCall<DebugApi, OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_address = address_h256_to_erdrs(&contract_call.to.to_address());
        let req = VmValueRequest {
            sc_address: sc_address.clone(),
            func_name: String::from_utf8(contract_call.endpoint_name.to_boxed_bytes().into_vec())
                .unwrap(),
            args: contract_call
                .arg_buffer
                .raw_arg_iter()
                .map(|arg| hex::encode(&arg.to_boxed_bytes().as_slice()))
                .collect(),
            caller: sc_address,
            value: "0".to_string(),
        };
        let result = self
            .proxy
            .execute_vmquery(&req)
            .await
            .expect("error executing VM query");

        // println!("{:#?}", result);

        let mut raw_results: Vec<Vec<u8>> = result
            .data
            .return_data
            .iter()
            .map(|result| base64::decode(result).expect("query result base64 decode error"))
            .collect();
        RequestedResult::multi_decode_or_handle_err(&mut raw_results, PanicErrorHandler).unwrap()
    }

    async fn recall_nonce(&self, address: &Address) -> u64 {
        let erdrs_address = address_h256_to_erdrs(address);
        let account = self
            .proxy
            .get_account(&erdrs_address)
            .await
            .expect("failed to retrieve account nonce");
        account.nonce
    }
}

fn mandos_to_erdrs_address(mandos_address: &AddressValue) -> ErdrsAddress {
    let bytes = mandos_address.value.as_array();
    ErdrsAddress::from_bytes(*bytes)
}

fn address_h256_to_erdrs(address: &Address) -> ErdrsAddress {
    let bytes = address.as_array();
    ErdrsAddress::from_bytes(*bytes)
}

fn erdrs_address_to_h256(erdrs_address: ErdrsAddress) -> Address {
    erdrs_address.to_bytes().into()
}
