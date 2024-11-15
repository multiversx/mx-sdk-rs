use std::{fs, process::Command};

use convert_case::{Case, Casing};
use multiversx_sc_meta::{
    cmd::template::{
        template_names_from_repo, ContractCreator, ContractCreatorTarget, RepoSource, RepoVersion,
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
    template_test_current(
        "adder",
        "examples",
        "new-adder",
        "Alin Cruceat <alin.cruceat@multiversx.com>",
    );

    cargo_check_interactor("examples", "new-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_crypto_zombies() {
    template_test_current("crypto-zombies", "examples", "new-crypto-zombies", "");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_empty() {
    template_test_current("empty", "examples", "new-empty", "");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn template_current_ping_pong_egld() {
    template_test_current("ping-pong-egld", "examples", "new-ping-pong-egld", "");
}

#[test]
#[cfg_attr(not(feature = "template-test-current"), ignore)]
fn test_correct_naming() {
    assert_eq!(
        "myNew42-correct_Empty".to_string().to_case(Case::Kebab),
        "my-new-42-correct-empty"
    );

    template_test_current("empty", "examples", "my1New2_3-correct_Empty", "");
}

/// Recreates the folder structure in `contracts`, on the same level.
/// This way, the relative paths are still valid in this case,
/// and we can test the templates with the framework version of the current branch.
fn template_test_current(template_name: &str, sub_path: &str, new_name: &str, new_author: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target = ContractCreatorTarget {
        target_path: workspace_path.join(TEMPLATE_TEMP_DIR_NAME).join(sub_path),
        new_name: new_name.to_string().to_case(Case::Kebab),
    };

    let repo_source = RepoSource::from_local_path(workspace_path);

    prepare_target_dir(&target);

    let author = if new_author.is_empty() {
        None
    } else {
        Some(new_author.to_string())
    };

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        true,
        author,
    )
    .create_contract(LAST_TEMPLATE_VERSION);

    if BUILD_CONTRACTS {
        build_contract(&target);
    }
    cargo_test(&target);
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_adder() {
    template_test_released(
        "adder",
        "released-adder",
        "Alin Cruceat <alin.cruceat@multiversx.com>",
    );

    cargo_check_interactor("", "released-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_crypto_zombies() {
    template_test_released("crypto-zombies", "released-crypto-zombies", "");
}

#[test]
#[cfg_attr(not(feature = "template-test-released"), ignore)]
fn template_released_empty() {
    template_test_released("empty", "released-empty", "");
}

/// These tests fully replicate the templating process. They
/// - download the last released version of the repo,
/// - create proper contracts,
/// - build the newly created contracts (to wasm)
/// - run all tests (including Go scenarios) on them.
fn template_test_released(template_name: &str, new_name: &str, new_author: &str) {
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

    let author = if new_author.is_empty() {
        None
    } else {
        Some(new_author.to_string())
    };

    ContractCreator::new(
        &repo_source,
        template_name.to_string(),
        target.clone(),
        false,
        author,
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

fn cargo_check_interactor(sub_path: &str, new_name: &str) {
    let workspace_path = find_current_workspace().unwrap();
    let target_path = workspace_path
        .join(TEMPLATE_TEMP_DIR_NAME)
        .join(sub_path)
        .join(new_name)
        .join("interact");

    let exit_status = Command::new("cargo")
        .arg("check")
        .current_dir(target_path)
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}
