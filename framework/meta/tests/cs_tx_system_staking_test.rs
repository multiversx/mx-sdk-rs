//! Chain simulator tests for `sc-meta tx system` governance, delegation-manager, delegation and validator.
//!
//! Note: the default chain simulator configuration does not allow new nodes to be activated
//! ("number of nodes too high"), so the tests cover node registration, but not node activation.
//! The chain simulator also does not verify BLS signatures, so dummy signatures are used.

use std::{path::Path, process::Command};

use multiversx_sc_meta_lib::tools::find_current_workspace;
use multiversx_sc_snippets::{Interactor, imports::SetStateAccount};
use multiversx_sdk::{gateway::NetworkStatusRequest, wallet::Wallet};
use serde_json::{Value, json};
use serial_test::serial;

const CHAIN_SIMULATOR_URL: &str = "http://localhost:8085";

/// The owner needs more than the 100 EGLD the CLI funds automatically: 100,000 EGLD.
const OWNER_BALANCE: &str = "100000000000000000000000";

/// Minimum gas limit and gas per data byte, in the chain simulator network config.
const MIN_GAS_LIMIT: u64 = 50_000;
const GAS_PER_DATA_BYTE: u64 = 1_500;

/// Test BLS public keys (from `tools/interactor-delegation-func-calls`).
const BLS_KEY_1: &str = "62b55cc67494ea9488e4406a2ebd52ffb2f8d58ea08eb37c15a3b5c7398f9b4209f9fa8b93c9d09e6f5a2aef06adc20d412c4ebbaccbc88556f2d05ff519243c1411a4847efb45887d2b6c7ae0e304da006f7d3565d9aa505fe7b2e170cdaf09";
const BLS_KEY_2: &str = "69ee4118e974b4dedaed1b49f630eeaac9bce10993b75f1be8d64594034ae6333394f1de4cacada14f3ee28582a8950b791bd0de492ba8737e8cfc0a66e8d0fb77763c2db5cced46b99a7f094a977908e34f0cc66044ae1b69505aacbfb17589";

const EGLD: u128 = 1_000_000_000_000_000_000;

struct SystemTestContext<'a> {
    sc_meta_bin: &'a str,
    owner_pem: &'a Path,
    outfiles_dir: &'a Path,
}

impl SystemTestContext<'_> {
    /// Runs `sc-meta tx system <args>` and returns stdout, asserting success.
    fn run(&self, args: &[&str]) -> String {
        let output = Command::new(self.sc_meta_bin)
            .args(["tx", "system"])
            .args(args)
            .output()
            .expect("failed to execute sc-meta");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        println!("system {args:?} stdout:\n{stdout}");
        assert!(
            output.status.success(),
            "sc-meta tx system {args:?} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        stdout
    }

    /// Runs a transaction command, broadcasts it and waits for the result.
    /// Returns the outfile JSON.
    fn tx(&self, name: &str, args: &[&str]) -> Value {
        let outfile = self
            .outfiles_dir
            .join(format!("system-{name}-cs.interaction.json"));
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
        serde_json::from_str(&content).expect("invalid outfile JSON")
    }

    /// Runs a query command and parses its JSON output.
    fn query(&self, args: &[&str]) -> Value {
        let mut all_args = args.to_vec();
        all_args.extend_from_slice(&["--proxy", CHAIN_SIMULATOR_URL]);
        let stdout = self.run(&all_args);
        serde_json::from_str(stdout.trim()).expect("query output is not JSON")
    }
}

/// The default gas limit, as computed by sc-meta: operation cost + minimum gas limit + data cost.
fn expected_default_gas(outfile_json: &Value, operation_gas: u64) -> u64 {
    let data_len = outfile_json["emittedTransactionData"]
        .as_str()
        .unwrap()
        .len() as u64;
    operation_gas + MIN_GAS_LIMIT + GAS_PER_DATA_BYTE * data_len
}

fn dummy_signature() -> String {
    "11".repeat(48)
}

fn egld(amount: u128) -> String {
    (amount * EGLD).to_string()
}

async fn current_epoch(interactor: &Interactor) -> u64 {
    interactor
        .proxy()
        .http_request(NetworkStatusRequest::default())
        .await
        .unwrap()
        .epoch_number
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_staking_system_sc_cli() {
    let workspace = find_current_workspace().unwrap();
    let test_artefacts_dir = workspace.join("framework/meta/tests/cs_tx_cli_test");
    let outfiles_dir = test_artefacts_dir.join("outfiles");
    std::fs::create_dir_all(&outfiles_dir).unwrap();
    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

    // Generate the wallet via the CLI so the file is not stored in the repo.
    let owner_pem = test_artefacts_dir.join("bob.pem");
    generate_test_wallet(sc_meta_bin, "bob", &owner_pem);
    let owner_address = Wallet::from_pem_file(&owner_pem).unwrap().to_address();
    let owner = owner_address.to_bech32_default().to_bech32_string();

    // The system SCs are only enabled after the first epochs.
    let interactor = Interactor::new(CHAIN_SIMULATOR_URL)
        .await
        .use_chain_simulator(true);
    interactor.generate_blocks_until_epoch(2).await.unwrap();

    // Fund the owner, keeping the current account nonce.
    let mut owner_account = SetStateAccount::from(interactor.get_account(&owner_address).await);
    owner_account.balance = OWNER_BALANCE.to_string();
    interactor.set_state(vec![owner_account]).await.unwrap();

    let ctx = SystemTestContext {
        sc_meta_bin,
        owner_pem: &owner_pem,
        outfiles_dir: &outfiles_dir,
    };

    let contract = delegation_manager_steps(&ctx);
    delegation_steps(&ctx, &contract, &owner);
    validator_steps(&ctx, &owner);
    governance_steps(&ctx, &interactor, &owner).await;
}

/// Creates a delegation contract, returns its address.
fn delegation_manager_steps(ctx: &SystemTestContext<'_>) -> String {
    let config = ctx.query(&["delegation-manager", "contract-config"]);
    let min_deposit = config["minDeposit"].as_str().unwrap().to_owned();
    assert_eq!(min_deposit, egld(1250));

    // the initial deposit is resolved from the config
    let create = ctx.tx(
        "create-delegation",
        &[
            "delegation-manager",
            "create-new-delegation-contract",
            "--service-fee",
            "1000",
        ],
    );
    assert_eq!(create["emittedTransaction"]["value"], min_deposit);
    assert_eq!(
        create["emittedTransaction"]["gasLimit"],
        expected_default_gas(&create, 60_000_000)
    );

    let contract = ctx.query(&["delegation-manager", "contract-config"])["lastContract"]
        .as_str()
        .unwrap()
        .to_owned();
    let contracts = ctx.query(&["delegation-manager", "contract-addresses"]);
    assert!(contracts.as_array().unwrap().contains(&json!(contract)));
    contract
}

fn delegation_steps(ctx: &SystemTestContext<'_>, contract: &str, owner: &str) {
    let config = ctx.query(&["delegation", contract, "contract-config"]);
    assert_eq!(config["owner"], owner);
    assert_eq!(config["serviceFee"], 1000);

    ctx.tx(
        "delegation-set-metadata",
        &[
            "delegation",
            contract,
            "set-metadata",
            "--name",
            "CLI Provider",
            "--website",
            "https://example.com",
            "--identifier",
            "cliprovider",
        ],
    );
    assert_eq!(
        ctx.query(&["delegation", contract, "metadata"]),
        json!({
            "name": "CLI Provider",
            "website": "https://example.com",
            "identifier": "cliprovider",
        })
    );

    ctx.tx(
        "delegation-delegate",
        &["delegation", contract, "delegate", "--value", &egld(10)],
    );
    let totals = ctx.query(&["delegation", contract, "totals"]);
    assert_eq!(totals["totalActiveStake"], egld(1260));
    assert_eq!(totals["numUsers"], 1);

    // nodes, with explicit keys and signatures
    let signature = dummy_signature();
    ctx.tx(
        "delegation-add-nodes",
        &[
            "delegation",
            contract,
            "add-nodes",
            "--bls-keys",
            BLS_KEY_1,
            "--signatures",
            &signature,
        ],
    );
    let node_states = ctx.query(&["delegation", contract, "node-states"]);
    assert_eq!(node_states["notStaked"], json!([BLS_KEY_1]));
    assert_eq!(
        ctx.query(&["delegation", contract, "totals"])["numNodes"],
        1
    );

    ctx.tx(
        "delegation-remove-nodes",
        &[
            "delegation",
            contract,
            "remove-nodes",
            "--bls-keys",
            BLS_KEY_1,
        ],
    );
    let node_states = ctx.query(&["delegation", contract, "node-states"]);
    assert_eq!(node_states["notStaked"], json!([]));

    ctx.tx(
        "delegation-undelegate",
        &["delegation", contract, "undelegate", "--amount", &egld(1)],
    );
    let delegator = ctx.query(&["delegation", contract, "delegator", owner]);
    assert_eq!(delegator["activeStake"], egld(1259));
    assert_eq!(delegator["undelegated"][0]["amount"], egld(1));
}

fn validator_steps(ctx: &SystemTestContext<'_>, owner: &str) {
    // the stake is registered, even though the node cannot be activated on the default chain simulator
    let signature = dummy_signature();
    let stake = ctx.tx(
        "validator-stake",
        &[
            "validator",
            "stake",
            "--bls-keys",
            BLS_KEY_2,
            "--signatures",
            &signature,
            "--value",
            &egld(2500),
        ],
    );
    assert_eq!(
        stake["emittedTransactionData"],
        format!("stake@01@{BLS_KEY_2}@{signature}")
    );
    assert_eq!(
        stake["emittedTransaction"]["gasLimit"],
        expected_default_gas(&stake, 5_000_000)
    );
    assert_eq!(
        ctx.query(&["validator", "total-staked", owner])["totalStaked"],
        egld(2500)
    );

    ctx.tx(
        "validator-unstake-tokens",
        &["validator", "unstake-tokens", "--amount", &egld(1)],
    );
    let unstaked = ctx.query(&["validator", "unstaked-tokens", owner]);
    assert_eq!(unstaked[0]["amount"], egld(1));
}

async fn governance_steps(ctx: &SystemTestContext<'_>, interactor: &Interactor, owner: &str) {
    let config = ctx.query(&["governance", "config"]);
    let proposal_fee = config["proposalFee"].as_str().unwrap().to_owned();
    let proposal_nonce = config["lastProposalNonce"].as_u64().unwrap() + 1;

    // the proposal fee is resolved from the config
    let start_epoch = current_epoch(interactor).await + 1;
    let propose = ctx.tx(
        "governance-propose",
        &[
            "governance",
            "propose",
            "--commit-hash",
            "6db132d759482f9f3515fe3ca8f72a8d6dc61244",
            "--start-vote-epoch",
            &start_epoch.to_string(),
            "--end-vote-epoch",
            &(start_epoch + 1).to_string(),
        ],
    );
    assert_eq!(propose["emittedTransaction"]["value"], proposal_fee);

    let proposal_nonce_str = proposal_nonce.to_string();
    let proposal = ctx.query(&["governance", "proposal", &proposal_nonce_str]);
    assert_eq!(proposal["issuer"], owner);
    assert_eq!(
        proposal["commitHash"],
        "6db132d759482f9f3515fe3ca8f72a8d6dc61244"
    );
    assert_eq!(proposal["closed"], false);

    // the voting power comes from the delegation and the validator stake
    let voting_power = ctx.query(&["governance", "voting-power", owner])["votingPower"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_ne!(voting_power, "0");

    interactor
        .generate_blocks_until_epoch(start_epoch)
        .await
        .unwrap();
    ctx.tx(
        "governance-vote",
        &[
            "governance",
            "vote",
            "--proposal-nonce",
            &proposal_nonce_str,
            "--vote",
            "yes",
        ],
    );
    let proposal = ctx.query(&["governance", "proposal", &proposal_nonce_str]);
    assert_eq!(proposal["yes"], voting_power);
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
