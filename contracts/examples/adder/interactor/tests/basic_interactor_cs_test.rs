use adder::adder_proxy::{SovereignConfig, StakeArgs};
use basic_interactor::{AdderInteract, Config};
use multiversx_sc_snippets::{
    imports::*, sdk::gateway::SetStateAccount, test_wallets, InteractorRunAsync,
};
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "fails, but it shouldn't"]
async fn simulator_sovereign_config_test() {
    let mut basic_interact = AdderInteract::new(Config::chain_simulator_config()).await;
    let stake_args = StakeArgs::<StaticApi> {
        token_id: TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"),
        amount: BigUint::from(2u64),
    };

    let config = SovereignConfig::<StaticApi> {
        min_validators: 1u64,
        max_validators: 18u64,
        min_stake: BigUint::from(100u64),
        opt_additional_stake_required: Some(ManagedVec::from_single_item(stake_args)),
    };

    let first_adder = basic_interact.deploy().await;
    basic_interact.set_storage(config).await;

    let _second_adder = basic_interact.deploy().await;
    let config_from_address = basic_interact.get_storage_from_address(first_adder).await;

    println!("Config from address:\n {:?}", config_from_address)
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut basic_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    basic_interact.deploy().await;
    basic_interact.add(1u32).await;

    // Sum will be 1
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 1u32.into());

    basic_interact
        .upgrade(7u32, &basic_interact.adder_owner_address.clone(), None)
        .await;

    // Sum will be the updated value of 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());

    // Upgrade fails
    basic_interact
        .upgrade(
            10u32,
            &basic_interact.wallet_address.clone(),
            Some("upgrade is allowed only for owner"),
        )
        .await;

    // Sum will remain 7
    let sum = basic_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_cs_test() {
    let account_address = test_wallets::mike();

    let real_chain_interact = AdderInteract::new(Config::load_config()).await;
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    let account = real_chain_interact
        .interactor
        .get_account(&account_address.to_address())
        .await;
    let keys = real_chain_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    let set_state_account = SetStateAccount::from(account).with_keys(keys);
    let vec_state = vec![set_state_account];

    let set_state_response = simulator_interact.interactor.set_state(vec_state).await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_from_file_cs_test() {
    let account_address = test_wallets::mike();
    let account_address_2 = test_wallets::ivan();

    let mut real_chain_interact = AdderInteract::new(Config::load_config()).await;
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // now we should have current mike account in the set state file
    real_chain_interact
        .interactor
        .retrieve_account(&Bech32Address::from(&account_address.to_address()))
        .await;

    real_chain_interact
        .interactor
        .retrieve_account(&Bech32Address::from(&account_address_2.to_address()))
        .await;

    let set_state_response = simulator_interact
        .interactor
        .set_state_for_saved_accounts()
        .await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());
}

#[tokio::test]
#[serial]
#[ignore = "fails before status check"]
async fn send_to_non_existent_address_test() {
    let registered_wallet = test_wallets::ivan();
    let not_registered_wallet = test_wallets::alice();

    let mut real_chain_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // send to not registered address
    real_chain_interact
        .interactor
        .tx()
        .from(&registered_wallet.to_address())
        .to(&not_registered_wallet.to_address())
        .egld(1u64)
        .run()
        .await;

    // send back -> should fail
    real_chain_interact
        .interactor
        .tx()
        .from(&not_registered_wallet.to_address())
        .to(&registered_wallet.to_address())
        .egld(1u64)
        .run()
        .await;
}
