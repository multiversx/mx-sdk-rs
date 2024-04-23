use crate::{
    imports::Bech32Address,
    scenario_model::{Account, BytesKey, BytesValue, Scenario, SetStateStep},
};

use multiversx_chain_scenario_format::interpret_trait::IntoRaw;
use multiversx_sdk::{
    blockchain::CommunicationProxy,
    data::{address::Address, esdt::EsdtBalance},
};
use std::collections::{BTreeMap, HashMap};

pub async fn print_account_as_scenario_set_state(
    api: &CommunicationProxy,
    address: &Bech32Address,
) {
    let set_state = retrieve_account_as_scenario_set_state(api, address).await;
    let scenario = build_scenario(set_state);
    println!("{}", scenario.into_raw().to_json_string());
}

fn build_scenario(set_state: SetStateStep) -> Scenario {
    Scenario {
        name: None,
        comment: None,
        check_gas: None,
        steps: vec![crate::scenario_model::Step::SetState(set_state)],
    }
}

pub async fn retrieve_account_as_scenario_set_state(
    api: &CommunicationProxy,
    address: &Bech32Address,
) -> SetStateStep {
    let sdk_address = Address::from_bech32_string(address.to_bech32_str()).unwrap();
    let sdk_account = api.get_account(&sdk_address).await.unwrap();

    let account_esdt = api
        .get_account_esdt_tokens(&sdk_address)
        .await
        .unwrap_or_else(|err| {
            panic!("failed to retrieve ESDT tokens for address {address}: {err}")
        });
    let account_esdt_roles = api
        .get_account_esdt_roles(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve ESDT roles for address {address}: {err}"));
    let account_storage = api
        .get_account_storage_keys(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve storage for address {address}: {err}"));

    let account_state = set_account(
        sdk_account,
        account_storage,
        account_esdt,
        account_esdt_roles,
    );

    let set_state_step = SetStateStep::new();
    set_state_step.put_account(address, account_state)
}

pub fn set_account(
    account: multiversx_sdk::data::account::Account,
    account_storage: HashMap<String, String>,
    account_esdt: HashMap<String, EsdtBalance>,
    account_esdt_roles: HashMap<String, Vec<String>>,
) -> Account {
    let mut account_state = Account::new()
        .nonce(account.nonce)
        .balance(account.balance.as_str())
        .code(account.code);
    account_state.username = Some(account.username.as_str().into());
    account_state.storage = convert_storage(account_storage);

    for (_, esdt_balance) in account_esdt.iter() {
        account_state = account_state.esdt_balance(
            format!("str:{}", esdt_balance.token_identifier).as_str(),
            esdt_balance.balance.as_str(),
        );
    }

    for (token_id, esdt_roles) in account_esdt_roles {
        account_state = account_state.esdt_roles(token_id.as_str(), esdt_roles);
    }

    account_state
}

fn convert_storage(account_storage: HashMap<String, String>) -> BTreeMap<BytesKey, BytesValue> {
    account_storage
        .into_iter()
        .filter(|(k, _)| !k.starts_with("454c524f4e44"))
        .map(|(k, v)| (BytesKey::from(k.as_str()), BytesValue::from(v)))
        .collect()
}
