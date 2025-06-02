use crate::sdk_core::data::esdt::EsdtBalance;
use multiversx_chain_scenario_format::interpret_trait::IntoRaw;
use multiversx_sc_scenario::{
    imports::Bech32Address,
    scenario_model::{Account, BytesKey, BytesValue, Scenario, SetStateStep, Step},
};
use multiversx_sdk::gateway::{GatewayAsyncService, SetStateAccount};
use multiversx_sdk::gateway::{
    GetAccountEsdtRolesRequest, GetAccountEsdtTokensRequest, GetAccountRequest,
    GetAccountStorageRequest,
};
use std::collections::{BTreeMap, HashMap};

/// Called directly from CLI, from `sc-meta`.
///
/// Retrieves an account data via the API,
/// then formats it as a scenario set state step.
pub async fn print_account_as_scenario_set_state<GatewayProxy: GatewayAsyncService>(
    gateway_proxy: GatewayProxy,
    address_bech32_string: String,
) {
    let address = Bech32Address::from_bech32_string(address_bech32_string);
    let (_, set_state) = retrieve_account_as_scenario_set_state(&gateway_proxy, &address).await;
    let scenario = build_scenario(set_state);
    println!("{}", scenario.into_raw().to_json_string());
}

fn build_scenario(set_state: SetStateStep) -> Scenario {
    Scenario {
        name: None,
        comment: None,
        check_gas: None,
        steps: vec![Step::SetState(set_state)],
    }
}

pub async fn retrieve_account_as_scenario_set_state<GatewayProxy: GatewayAsyncService>(
    api: &GatewayProxy,
    bech32_address: &Bech32Address,
) -> (SetStateAccount, SetStateStep) {
    let hrp = bech32_address.as_hrp();
    let address = bech32_address.as_address();
    let sdk_account = api
        .request(GetAccountRequest::new(hrp, address))
        .await
        .unwrap();

    let account_esdt = api
        .request(GetAccountEsdtTokensRequest::new(hrp, address))
        .await
        .unwrap_or_else(|err| {
            eprintln!("failed to retrieve ESDT tokens for address {bech32_address}: {err}");
            HashMap::new()
        });
    let account_esdt_roles = api
        .request(GetAccountEsdtRolesRequest::new(hrp, address))
        .await
        .unwrap_or_else(|err| {
            eprintln!("failed to retrieve ESDT roles for address {bech32_address}: {err}");
            HashMap::new()
        });
    let account_storage = api
        .request(GetAccountStorageRequest::new(hrp, address))
        .await
        .unwrap_or_else(|err| {
            panic!("failed to retrieve storage for address {bech32_address}: {err}")
        });

    let account_state = set_account(
        sdk_account.clone(),
        account_storage.clone(),
        account_esdt,
        account_esdt_roles,
    );

    let set_state_account = SetStateAccount::from(sdk_account).with_storage(account_storage);
    let set_state_step = SetStateStep::new();
    (
        set_state_account,
        set_state_step.put_account(bech32_address, account_state),
    )
}

fn set_account(
    account: crate::sdk::data::account::Account,
    account_storage: HashMap<String, String>,
    account_esdt: HashMap<String, EsdtBalance>,
    account_esdt_roles: HashMap<String, Vec<String>>,
) -> Account {
    let mut account_state = Account::new()
        .nonce(account.nonce)
        .balance(account.balance.as_str())
        .code(BytesValue::from_hex(&account.code));
    account_state.username = Some(format!("str:{}", account.username.as_str()).into());
    account_state.storage = convert_storage(account_storage);

    for (_, esdt_balance) in account_esdt.iter() {
        let token_id_expr = format!("str:{}", esdt_balance.token_identifier);
        account_state =
            account_state.esdt_balance(token_id_expr.as_str(), esdt_balance.balance.as_str());
    }

    for (token_id, esdt_roles) in account_esdt_roles {
        let token_id_expr = format!("str:{token_id}");
        account_state = account_state.esdt_roles(token_id_expr.as_str(), esdt_roles);
    }

    account_state
}

fn convert_storage(account_storage: HashMap<String, String>) -> BTreeMap<BytesKey, BytesValue> {
    account_storage
        .into_iter()
        .filter(|(k, _)| !k.starts_with("454c524f4e44"))
        .map(|(k, v)| (BytesKey::from_hex(&k), BytesValue::from_hex(&v)))
        .collect()
}
