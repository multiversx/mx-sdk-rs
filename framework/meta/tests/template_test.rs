use std::{fs, process::Command};

use convert_case::{Case, Casing};
use multiversx_sc_meta::{
    cmd::template::{
        ContractCreator, ContractCreatorTarget, RepoSource, RepoVersion, template_names_from_repo,
    },
    version_history::{self, LAST_TEMPLATE_VERSION},
};
use multiversx_sc_meta_lib::tools::find_current_workspace;

const TEMPLATE_TEMP_DIR_NAME: &str = "template-test";
const BUILD_CONTRACTS: bool = true;

#[test]
fn test_template_list() {
    let workspace_path = find_current_workspace().unwrap();
    let repo_source = RepoSource::from_local_path(workspace_path);
    let mut template_names = template_names_from_repo(&repo_source);
    template_names.sort();
    assert_eq!(
        template_names,
        [
            "adder".to_string(),
            "crypto-zombies".to_string(),
            "empty".to_string(),
            "ping-pong-egld".to_string(),
        ]
    );
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_adder() {
    template_test_current("adder", "examples", "new-adder");

    cargo_check_interactor("examples", "new-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_crypto_zombies() {
    template_test_current("crypto-zombies", "examples", "new-crypto-zombies");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_empty() {
    template_test_current("empty", "examples", "new-empty");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_ping_pong_egld() {
    template_test_current("ping-pong-egld", "examples", "new-ping-pong-egld");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn test_correct_naming() {
    assert_eq!(
        "myNew42-correct_Empty".to_string().to_case(Case::Kebab),
        "my-new-42-correct-empty"
    );

    template_test_current("empty", "examples", "my1New2_3-correct_Empty");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_locked_test() {
    let workspace_path = find_current_workspace().unwrap();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME).join("examples"),
        new_name: "new-empty-locked".to_string(),
    };

    let repo_source = RepoSource::from_local_path(workspace_path.clone());

    prepare_target_dir(&target);

    ContractCreator::new(
        &repo_source,
        "empty".to_string(),
        target.clone(),
        true,
        None,
    )
    .create_contract(LAST_TEMPLATE_VERSION);

    // Build once to generate Cargo.lock
    build_contract(&target);

    // Build with --locked flag (should pass)
    let build_result = try_build_contract(&target, true);
    assert!(build_result, "build with --locked should pass");

    // Remove Cargo.lock from wasm directory
    let wasm_cargo_lock = target.contract_dir().join("wasm").join("Cargo.lock");
    assert!(
        wasm_cargo_lock.exists(),
        "Cargo.lock should exist in wasm directory"
    );
    fs::remove_file(&wasm_cargo_lock).expect("failed to remove Cargo.lock");

    // Build with --locked flag again (should fail)
    let build_result = try_build_contract(&target, true);
    assert!(
        !build_result,
        "build with --locked should fail after removing Cargo.lock"
    );
}

/// Recreates the folder structure in `contracts`, on the same level.
/// This way, the relative paths are still valid in this case,
/// and we can test the templates with the framework version of the current branch.
fn template_test_current(template_name: &str, sub_path: &str, new_name: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME).join(sub_path),
        new_name: new_name.to_string().to_case(Case::Kebab),
    };

    let repo_source = RepoSource::from_local_path(workspace_path);

    prepare_target_dir(&target);

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        true,
        Some("New Author <test@multiversx.com>".to_string()),
    )
    .create_contract(LAST_TEMPLATE_VERSION);

    if BUILD_CONTRACTS {
        build_contract(&target);
    }
    cargo_test(&target);
}

#[tokio::test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
async fn template_released_adder() {
    template_test_released("adder", "released-adder").await;

    cargo_check_interactor("", "released-adder");
}

#[tokio::test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
async fn template_released_crypto_zombies() {
    template_test_released("crypto-zombies", "released-crypto-zombies").await;
}

#[tokio::test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
async fn template_released_empty() {
    template_test_released("empty", "released-empty").await;
}

/// These tests fully replicate the templating process. They
/// - download the last released version of the repo,
/// - create proper contracts,
/// - build the newly created contracts (to wasm)
/// - run all tests (including Go scenarios) on them.
async fn template_test_released(template_name: &str, new_name: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME),
        new_name: new_name.to_string(),
    };

    let temp_dir_path = workspace_path
        .join(TEMPLATE_TEMP_DIR_NAME)
        .join("temp-download")
        .join(new_name);
    let repo_source = RepoSource::download_from_github(
        RepoVersion::Tag(version_history::LAST_TEMPLATE_VERSION.to_string()),
        temp_dir_path,
    )
    .await;

    prepare_target_dir(&target);

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        false,
        None,
    )
    .create_contract(LAST_TEMPLATE_VERSION);

    if BUILD_CONTRACTS {
        build_contract(&target);
    }
    cargo_test(&target);
}

fn prepare_target_dir(target: &ContractCreatorTarget) {
    fs::create_dir_all(&target.target_path).unwrap();

    let contract_dir = target.contract_dir();
    if contract_dir.exists() {
        fs::remove_dir_all(&contract_dir).unwrap();
    }
}

pub fn cargo_test(target: &ContractCreatorTarget) {
    let workspace_target_dir = find_current_workspace().unwrap().join("target");

    let mut args = vec![
        "test",
        "--target-dir",
        workspace_target_dir.to_str().unwrap(),
    ];
    if BUILD_CONTRACTS {
        args.push("--features");
        args.push("multiversx-sc-scenario/compiled-sc-tests");
    }

    let exit_status = Command::new("cargo")
        .args(args)
        .current_dir(target.contract_dir())
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}

fn try_build_contract(target: &ContractCreatorTarget, locked: bool) -> bool {
    let workspace_target_dir = find_current_workspace().unwrap().join("target");

    let mut args = vec![
        "run",
        "build",
        "--target-dir",
        workspace_target_dir.to_str().unwrap(),
    ];
    if locked {
        args.push("--locked");
    }

    let exit_status = Command::new("cargo")
        .args(args)
        .current_dir(target.contract_dir().join("meta"))
        .spawn()
        .expect("failed to spawn contract build process")
        .wait()
        .expect("contract build process was not running");

    exit_status.success()
}

pub fn build_contract(target: &ContractCreatorTarget) {
    assert!(
        try_build_contract(target, false),
        "contract build process failed"
    );
}

fn cargo_check_interactor(sub_path: &str, new_name: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target_path = workspace_path
        .join(TEMPLATE_TEMP_DIR_NAME)
        .join(sub_path)
        .join(new_name)
        .join("interactor");

    let exit_status = Command::new("cargo")
        .arg("check")
        .current_dir(target_path)
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}
