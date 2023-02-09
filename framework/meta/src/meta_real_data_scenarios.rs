use std::collections::{BTreeMap, HashMap};

use multiversx_chain_scenario_format::serde_raw::{ScenarioRaw, StepRaw, AccountRaw, ValueSubTree, EsdtRaw};
use multiversx_sdk::blockchain::CommunicationProxy;
use multiversx_sdk::data::address::Address;
use crate::cli_args::RealDataScenarioArgs;

use tokio::runtime::Runtime;

pub fn call_real_data_scenario_builder(args: &RealDataScenarioArgs) {
    let addr = Address::from_bech32_string(&args.address).unwrap();
    let api = args.api.to_string();

    let blockchain = CommunicationProxy::new(api);
    // let future_res = async move {
    //     blockchain.get_account_storage_keys(&addr).await.unwrap()
    // };

    let account = Runtime::new().unwrap().block_on(blockchain.get_account(&addr)).unwrap();
    let account_esdt = Runtime::new().unwrap().block_on(blockchain.get_account_esdt_tokens(&addr)).unwrap();
    let account_storage = Runtime::new().unwrap().block_on(blockchain.get_account_storage_keys(&addr));
    println!("account storage keys {:?}\n account = {:?}", account_storage.unwrap(), account);

    
    // let esdt = BTreeMap<String, EsdtRaw>::from(account_esdt);
    // let accounts = BTreeMap::new();

    // let account_raw = AccountRaw {
    //     nonce: Some(ValueSubTree::Str(account.nonce.to_string())),
    //     balance: Some(ValueSubTree::Str(account.balance.to_string())),
    //     esdt,
    //     username: Some(ValueSubTree::Str(account.address.to_string())),
    //     storage: account_storage,
    //     comment: None,
    //     code: None,
    //     owner: None,
    //     developer_rewards: None,
    // };
    // accounts.insert("address:owner".to_string(), account_raw);

    // let _real_data_scenario = ScenarioRaw {
    //     check_gas: None,
    //     comment: None,
    //     gas_schedule: None,
    //     name: None,
    //     steps: vec![StepRaw::SetState {
    //         accounts: accounts,
    //         block_hashes: Vec::new(),
    //         new_addresses: Vec::new(),
    //         comment: None,
    //         current_block_info: None,
    //         previous_block_info: None,
    //     }],
    // };

}
