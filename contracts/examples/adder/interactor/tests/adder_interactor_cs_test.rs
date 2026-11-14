use adder_interactor::{AdderInteractor, Config, GeneralConfig};
use multiversx_sc_snippets::imports::*;
use serial_test::serial;

/// An address with no known private key: the raw bytes are a SHA-256 digest, not
/// a public key derived from a seed, so no valid signature for it can exist.
/// Transactions from it carry an all-zero signature, accepted here because the
/// chain simulator runs with `--bypass-txs-signature` (its default), as a shadow
/// fork would.
const NO_SIG_OWNER: &str = "erd1nte2lvrxjgmy9hgs5d88tf59c6gnuas9064x46stl8lgr3fqxhhqmj4kkr";

fn chain_simulator_config() -> Config {
    Config {
        general: GeneralConfig {
            contract_path: "../output/adder.mxsc.json".into(),
        },
        connection: ConnectionConfig::chain_simulator(),
        owner: WalletConfig::from_no_sig(NO_SIG_OWNER),
        wallet: WalletConfig::from_test_wallet("ivan"),
        ledger_wallet: None,
        relayer: Some(WalletConfig::from_test_wallet("s1mon")),
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

    // The owner deploys and upgrades without being able to sign anything.
    assert!(!adder_interact.config.owner.wallet().can_sign());
    assert!(adder_interact.config.wallet.wallet().can_sign());

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

    adder_interact.add_relayed(3u32).await;

    adder_interact.generate_blocks(2).await;

    // Sum will be 7 + 3 = 10
    let sum = adder_interact.get_sum().await;
    assert_eq!(sum, 10u32.into());
}
