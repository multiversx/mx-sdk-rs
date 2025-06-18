use basic_interactor::{AdderInteract, Config};
use multiversx_sc_snippets::{imports::Bech32Address, sdk::gateway::SetStateAccount, test_wallets};
use serial_test::serial;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn simulator_upgrade_test() {
    let mut basic_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    basic_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

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
    let pairs = real_chain_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    let set_state_account = SetStateAccount::from(account).with_storage(pairs);
    let vec_state = vec![set_state_account];

    let set_state_response = simulator_interact.interactor.set_state(vec_state).await;

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() > 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);
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

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() > 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_overwrite_cs_test() {
    let account_address = test_wallets::mike();
    let account_address_2 = test_wallets::ivan();

    let mut real_chain_interact = AdderInteract::new(Config::load_config()).await;
    let simulator_interact = AdderInteract::new(Config::chain_simulator_config()).await;

    // now we should have current mike and ivan accounts in the set state file
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

    let storage = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage.len() > 1);

    println!("mike's storage keys in chain simulator {:#?}", storage);

    // overwrite accounts with empty
    let account_1 = SetStateAccount::from_address(
        Bech32Address::from(&account_address.to_address()).to_bech32_string(),
    );
    let account_2 = SetStateAccount::from_address(
        Bech32Address::from(&account_address_2.to_address()).to_bech32_string(),
    );

    let overwrite_vec = vec![account_1, account_2];

    simulator_interact
        .interactor
        .set_state_overwrite(overwrite_vec)
        .await
        .unwrap();

    simulator_interact
        .interactor
        .generate_blocks(2u64)
        .await
        .unwrap();

    // verify keys
    let storage_1 = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage_1.is_empty());

    println!("mike's storage keys in chain simulator {:#?}", storage_1);

    let storage_2 = simulator_interact
        .interactor
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage_2.is_empty());

    println!("ivan's storage keys in chain simulator {:#?}", storage_2);
}
