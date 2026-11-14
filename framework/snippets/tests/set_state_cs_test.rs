// use adder_interactor::{BasicInteractor, Config, GeneralConfig};
use multiversx_sc_snippets::imports::*;
use serial_test::serial;

async fn cs_interactor() -> HttpInteractor {
    HttpInteractor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&ConnectionConfig::chain_simulator())
        .await
}

async fn real_chain_reader_interactor() -> HttpInteractor {
    HttpInteractor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&ConnectionConfig {
            gateway_uri: "https://devnet-gateway.multiversx.com".to_owned(),
            chain_type: ChainType::Real,
        })
        .await
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn set_state_cs_test() {
    let account_address = test_wallets::mike();

    let real_chain_interact = real_chain_reader_interactor().await;
    let simulator_interact = cs_interactor().await;

    let account = real_chain_interact
        .get_account(&account_address.to_address())
        .await;
    let pairs = real_chain_interact
        .get_account_storage(&account_address.to_address())
        .await;

    let set_state_account = SetStateAccount::from(account).with_storage(pairs);
    let vec_state = vec![set_state_account];

    let set_state_response = simulator_interact.set_state(vec_state).await;

    simulator_interact.generate_blocks(2u64).await.unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
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

    let mut real_chain_interact = real_chain_reader_interactor().await;
    let simulator_interact = cs_interactor().await;

    // now we should have current mike account in the set state file
    real_chain_interact
        .retrieve_account(&Bech32Address::from(&account_address.to_address()))
        .await;

    real_chain_interact
        .retrieve_account(&Bech32Address::from(&account_address_2.to_address()))
        .await;

    let set_state_response = simulator_interact.set_state_for_saved_accounts().await;

    simulator_interact.generate_blocks(2u64).await.unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
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

    let mut real_chain_interact = real_chain_reader_interactor().await;
    let simulator_interact = cs_interactor().await;

    // now we should have current mike and ivan accounts in the set state file
    real_chain_interact
        .retrieve_account(&Bech32Address::from(&account_address.to_address()))
        .await;

    real_chain_interact
        .retrieve_account(&Bech32Address::from(&account_address_2.to_address()))
        .await;

    let set_state_response = simulator_interact.set_state_for_saved_accounts().await;

    simulator_interact.generate_blocks(2u64).await.unwrap();

    assert!(set_state_response.is_ok());

    let storage = simulator_interact
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
        .set_state_overwrite(overwrite_vec)
        .await
        .unwrap();

    simulator_interact.generate_blocks(2u64).await.unwrap();

    // verify keys
    let storage_1 = simulator_interact
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage_1.is_empty());

    println!("mike's storage keys in chain simulator {:#?}", storage_1);

    let storage_2 = simulator_interact
        .get_account_storage(&account_address.to_address())
        .await;

    assert!(storage_2.is_empty());

    println!("ivan's storage keys in chain simulator {:#?}", storage_2);
}
