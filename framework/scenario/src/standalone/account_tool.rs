use super::scenario_cli::AccountArgs;
use multiversx_chain_scenario_format::serde_raw::{
    AccountRaw, EsdtFullRaw, EsdtInstanceRaw, EsdtRaw, ScenarioRaw, StepRaw, ValueSubTree,
};
use multiversx_sdk::{
    blockchain::CommunicationProxy,
    data::{address::Address, esdt::EsdtBalance},
};
use std::collections::{BTreeMap, HashMap};

pub async fn print_account_as_scenario_set_state(api: String, args: &AccountArgs) {
    let scenario_raw =
        retrieve_account_as_scenario_set_state(api, args.address.clone(), None).await;
    println!("{}", scenario_raw.to_json_string());
}

pub async fn retrieve_account_as_scenario_set_state(
    api: String,
    addr: String,
    custom_format: Option<String>,
) -> ScenarioRaw {
    let address = Address::from_bech32_string(&addr).unwrap();
    let blockchain = CommunicationProxy::new(api);
    let account = blockchain.get_account(&address).await.unwrap();
    let account_esdt = blockchain.get_account_esdt_tokens(&address).await.unwrap();
    let account_storage = blockchain.get_account_storage_keys(&address).await.unwrap();

    let addr_pretty = if custom_format.is_none() {
        if account.code.is_empty() {
            format!("address:{addr}")
        } else {
            format!("sc:{addr}")
        }
    } else {
        format!("{}:{addr}", custom_format.unwrap())
    };

    let mut accounts = BTreeMap::new();
    accounts.insert(
        addr_pretty,
        AccountRaw {
            nonce: Some(ValueSubTree::Str(account.nonce.to_string())),
            balance: Some(ValueSubTree::Str(account.balance.to_string())),
            esdt: convert_esdt(account_esdt),
            username: Some(ValueSubTree::Str(account.username.to_string())),
            storage: convert_storage(account_storage),
            comment: None,
            code: None,
            owner: None,
            developer_rewards: None,
        },
    );

    ScenarioRaw {
        check_gas: None,
        comment: None,
        gas_schedule: None,
        name: None,
        steps: vec![StepRaw::SetState {
            accounts,
            block_hashes: Vec::new(),
            new_addresses: Vec::new(),
            comment: None,
            current_block_info: None,
            previous_block_info: None,
        }],
    }
}

fn convert_storage(account_storage: HashMap<String, String>) -> BTreeMap<String, ValueSubTree> {
    account_storage
        .into_iter()
        .map(|(k, v)| (format!("0x{k}"), ValueSubTree::Str(format!("0x{v}"))))
        .collect()
}

fn convert_esdt(sdk_esdt: HashMap<String, EsdtBalance>) -> BTreeMap<String, EsdtRaw> {
    let mut result = BTreeMap::new();
    for (key, value) in sdk_esdt.into_iter() {
        let (token_identifier, nonce) = split_token_identifer_nonce(key);
        let esdt_raw = result
            .entry(token_identifier.clone())
            .or_insert(EsdtRaw::Full(EsdtFullRaw::default()));
        if let EsdtRaw::Full(esdt_full_raw) = esdt_raw {
            esdt_full_raw.instances.push(EsdtInstanceRaw {
                nonce: Some(ValueSubTree::Str(nonce.to_string())),
                balance: Some(ValueSubTree::Str(value.balance)),
                // TODO: add creator, royalties, etc ...
                ..Default::default()
            });
        }
    }
    result
}

fn split_token_identifer_nonce(full_identifier: String) -> (String, u64) {
    let tokens = full_identifier.split('-').collect::<Vec<_>>();
    match tokens.len() {
        2 => (full_identifier, 0),
        3 => (
            format!("{}-{}", tokens[0], tokens[1]),
            u64::from_str_radix(tokens[2], 16).unwrap(),
        ),
        _ => panic!("could not process token identifier: {full_identifier}"),
    }
}
