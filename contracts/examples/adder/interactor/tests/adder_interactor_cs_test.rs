use adder_interactor::{AdderInteractor, Config, GeneralConfig};
use multiversx_sc_snippets::imports::*;
use serial_test::serial;

fn chain_simulator_config() -> Config {
    Config {
        general: GeneralConfig {
            contract_path: "../output/adder.mxsc.json".into(),
        },
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_test_wallet("mike"),
        wallet: WalletConfig::from_test_wallet("ivan"),
    }
}

async fn test_interactor() -> AdderInteractor {
    let config = chain_simulator_config();
    let interactor = Interactor::empty()
        .with_current_dir(env!("CARGO_MANIFEST_DIR"))
        .with_config(&config)
        .await;
    AdderInteractor {
        interactor,
        config,
        state: AutoSave::no_save_default(),
    }
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_add_upgrade_test() {
    let mut adder_interact = test_interactor().await;

    adder_interact.generate_blocks(2).await;

    adder_interact.deploy().await;

    adder_interact.generate_blocks(2).await;

    adder_interact.add(1u32).await;

    adder_interact.generate_blocks(2).await;

    // Sum will be 1
    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 1u32.into());

    adder_interact
        .upgrade(7u32, &adder_interact.config.owner.address(), None)
        .await;

    adder_interact.generate_blocks(2).await;

    // Sum will be the updated value of 7
    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());

    // Upgrade fails
    adder_interact
        .upgrade(
            10u32,
            &adder_interact.config.wallet.address(),
            Some("upgrade is allowed only for owner"),
        )
        .await;

    adder_interact.generate_blocks(2).await;

    // Sum will remain 7
    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 7u32.into());
}
