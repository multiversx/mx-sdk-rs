use std::{fs, process::Command};

use multiversx_sc_meta::{
    cmd::standalone::template::{
        template_names_from_repo, ContractCreator, ContractCreatorTarget, RepoSource, RepoVersion,
    },
    find_workspace::find_current_workspace,
    version_history::{self, LAST_TEMPLATE_VERSION},
};

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

/// Recreates the folder structure in `contracts`, on the same level.
/// This way, the relative paths are still valid in this case,
/// and we can test the templates with the framework version of the current branch.
fn template_test_current(template_name: &str, sub_path: &str, new_name: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME).join(sub_path),
        new_name: new_name.to_string(),
    };

    let repo_source = RepoSource::from_local_path(workspace_path);

    prepare_target_dir(&target);

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        true,
    )
    .create_contract(LAST_TEMPLATE_VERSION.to_string());

    if BUILD_CONTRACTS {
        build_contract(&target);
    }
    cargo_test(&target);
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_adder() {
    template_test_released("adder", "released-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_crypto_zombies() {
    template_test_released("crypto-zombies", "released-crypto-zombies");
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_empty() {
    template_test_released("empty", "released-empty");
}

/// These tests fully replicate the templating process. They
/// - download the last released version of the repo,
/// - create proper contracts,
/// - build the newly created contracts (to wasm)
/// - run all tests (including Go scenarios) on them.
fn template_test_released(template_name: &str, new_name: &str) {
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
    );

    prepare_target_dir(&target);

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        false,
    )
    .create_contract(LAST_TEMPLATE_VERSION.to_string());

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
        args.push("multiversx-sc-scenario/run-go-tests");
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

pub fn build_contract(target: &ContractCreatorTarget) {
    let workspace_target_dir = find_current_workspace().unwrap().join("target");

    let exit_status = Command::new("cargo")
        .args([
            "run",
            "build",
            "--target-dir",
            workspace_target_dir.to_str().unwrap(),
        ])
        .current_dir(target.contract_dir().join("meta"))
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract build process failed");
}
