//! Chain simulator tests for `sc-meta tx system esdt`.

use std::{path::Path, process::Command};

use multiversx_chain_core::std::base64_decode;
use multiversx_sc_meta_lib::tools::find_current_workspace;
use multiversx_sc_snippets::Interactor;
use multiversx_sdk::wallet::Wallet;
use serde_json::Value;
use serial_test::serial;

const CHAIN_SIMULATOR_URL: &str = "http://localhost:8085";

/// The issue cost in the default chain simulator config (0.05 EGLD).
const ISSUE_COST: &str = "50000000000000000";

/// The default gas limit of the ESDT system SC commands.
const ESDT_DEFAULT_GAS: u64 = 60_000_000;

struct EsdtTestContext<'a> {
    sc_meta_bin: &'a str,
    owner_pem: &'a Path,
    owner_address: &'a str,
    outfiles_dir: &'a Path,
}

impl EsdtTestContext<'_> {
    /// Runs `sc-meta tx system esdt <args>` and returns stdout, asserting success.
    fn run(&self, args: &[&str]) -> String {
        let output = Command::new(self.sc_meta_bin)
            .args(["tx", "system", "esdt"])
            .args(args)
            .output()
            .expect("failed to execute sc-meta");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        println!("esdt {args:?} stdout:\n{stdout}");
        assert!(
            output.status.success(),
            "sc-meta tx system esdt {args:?} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        stdout
    }

    /// Runs a transaction command, broadcasts it and waits for the result.
    /// Returns the outfile JSON.
    fn tx(&self, name: &str, args: &[&str]) -> Value {
        let outfile = self
            .outfiles_dir
            .join(format!("esdt-{name}-cs.interaction.json"));
        let mut all_args = args.to_vec();
        all_args.extend_from_slice(&[
            "--pem",
            self.owner_pem.to_str().unwrap(),
            "--proxy",
            CHAIN_SIMULATOR_URL,
            "--send",
            "--wait-result",
            "--outfile",
            outfile.to_str().unwrap(),
        ]);
        self.run(&all_args);

        let content = std::fs::read_to_string(&outfile).expect("failed to read outfile");
        let json: Value = serde_json::from_str(&content).expect("invalid outfile JSON");
        assert_eq!(
            json["transactionOnNetwork"]["status"].as_str().unwrap(),
            "success",
            "transaction {name} did not succeed"
        );
        json
    }

    /// Runs a query command and parses its JSON output.
    fn query(&self, args: &[&str]) -> Value {
        let mut all_args = args.to_vec();
        all_args.extend_from_slice(&["--proxy", CHAIN_SIMULATOR_URL]);
        let stdout = self.run(&all_args);
        serde_json::from_str(stdout.trim()).expect("query output is not JSON")
    }

    fn token_properties(&self, token: &str) -> Value {
        self.query(&["token-properties", token])
    }
}

/// Extracts the first topic (the token identifier) of the log event with the given identifier.
fn token_id_from_log(outfile_json: &Value, event_identifier: &str) -> String {
    let events = outfile_json["transactionOnNetwork"]["logs"]["events"]
        .as_array()
        .expect("no log events");
    let event = events
        .iter()
        .find(|event| event["identifier"].as_str() == Some(event_identifier))
        .unwrap_or_else(|| panic!("no {event_identifier} log event"));
    let topic = event["topics"][0].as_str().unwrap();
    String::from_utf8(base64_decode(topic).unwrap()).unwrap()
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_esdt_system_sc_cli() {
    let workspace = find_current_workspace().unwrap();
    let test_artefacts_dir = workspace.join("framework/meta/tests/cs_tx_cli_test");
    let outfiles_dir = test_artefacts_dir.join("outfiles");
    std::fs::create_dir_all(&outfiles_dir).unwrap();
    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

    // Generate the wallets via the CLI so the files are not stored in the repo.
    // Relayed v3 requires the relayer to be in the same shard as the sender (alice and s1mon are).
    let owner_pem = test_artefacts_dir.join("alice.pem");
    let relayer_pem = test_artefacts_dir.join("s1mon.pem");
    generate_test_wallet(sc_meta_bin, "alice", &owner_pem);
    generate_test_wallet(sc_meta_bin, "s1mon", &relayer_pem);
    let owner_address = Wallet::from_pem_file(&owner_pem)
        .unwrap()
        .to_address()
        .to_bech32_default()
        .to_bech32_string();

    // The ESDT system SC is only enabled from epoch 1.
    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);
    interactor.generate_blocks_until_epoch(2).await.unwrap();

    let ctx = EsdtTestContext {
        sc_meta_bin,
        owner_pem: &owner_pem,
        owner_address: &owner_address,
        outfiles_dir: &outfiles_dir,
    };

    // config
    let config = ctx.query(&["contract-config"]);
    assert_eq!(config["issueCost"], ISSUE_COST);

    // issue fungible, the issue cost is resolved from the config
    let issue = ctx.tx(
        "issue-fungible",
        &[
            "issue-fungible",
            "--name",
            "CliTestToken",
            "--ticker",
            "CLI",
            "--initial-supply",
            "1000000",
            "--num-decimals",
            "6",
        ],
    );
    assert_eq!(issue["emittedTransaction"]["value"], ISSUE_COST);
    assert_eq!(issue["emittedTransaction"]["gasLimit"], ESDT_DEFAULT_GAS);
    let token = token_id_from_log(&issue, "issue");
    assert!(token.starts_with("CLI-"));

    let properties = ctx.token_properties(&token);
    assert_eq!(properties["name"], "CliTestToken");
    assert_eq!(properties["type"], "Fungible");
    assert_eq!(properties["owner"], ctx.owner_address);
    assert_eq!(properties["mintedValue"], "1000000");
    assert_eq!(properties["numDecimals"], 6);
    assert_eq!(properties["canWipe"], true);
    assert_eq!(properties["isPaused"], false);

    // roles, by short and by protocol name
    ctx.tx(
        "set-special-role",
        &[
            "set-special-role",
            &token,
            "--address",
            ctx.owner_address,
            "--roles",
            "LocalMint,ESDTRoleLocalBurn",
        ],
    );
    let roles = ctx.query(&["special-roles", &token]);
    let owner_roles = roles
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["address"] == ctx.owner_address)
        .expect("owner has no roles");
    assert_eq!(
        owner_roles["roles"],
        serde_json::json!(["ESDTRoleLocalMint", "ESDTRoleLocalBurn"])
    );

    // control changes
    ctx.tx(
        "control-changes",
        &["control-changes", &token, "--can-wipe", "false"],
    );
    assert_eq!(ctx.token_properties(&token)["canWipe"], false);

    // pause, relayed, with the default gas limit increased for the relayer
    let pause = ctx.tx(
        "pause-relayed",
        &[
            "pause",
            &token,
            "--relayer-pem",
            relayer_pem.to_str().unwrap(),
        ],
    );
    assert_eq!(
        pause["emittedTransaction"]["gasLimit"],
        ESDT_DEFAULT_GAS + 50_000
    );
    assert_eq!(ctx.token_properties(&token)["isPaused"], true);

    ctx.tx("unpause", &["unpause", &token]);
    assert_eq!(ctx.token_properties(&token)["isPaused"], false);

    // dynamic SFT: no decimals argument
    let register_dynamic = ctx.tx(
        "register-dynamic",
        &[
            "register-dynamic",
            "--name",
            "CliDynamicSft",
            "--ticker",
            "CLIDSFT",
            "--token-type",
            "SFT",
        ],
    );
    assert_eq!(
        register_dynamic["emittedTransactionData"],
        "registerDynamic@436c6944796e616d6963536674@434c4944534654@534654"
    );
    let dynamic_token = token_id_from_log(&register_dynamic, "registerDynamic");
    assert_eq!(ctx.token_properties(&dynamic_token)["type"], "DynamicSFT");
}

fn generate_test_wallet(sc_meta_bin: &str, name: &str, path: &Path) {
    let status = Command::new(sc_meta_bin)
        .args([
            "wallet",
            "test-wallet",
            "--name",
            name,
            "--path",
            path.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|_| panic!("failed to generate {name}.pem"));
    assert!(
        status.success(),
        "sc-meta wallet test-wallet failed for {name}"
    );
}
