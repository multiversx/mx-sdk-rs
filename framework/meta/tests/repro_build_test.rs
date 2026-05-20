use std::{fs, path::Path};

use multiversx_sc_meta::{
    cli::{
        AllArgs, InitConfigArgs, MetaLibArgs, ReleaseNotesArgs, ReproducibleBuildBuildArgs,
        ReproducibleBuildLocalBuildArgs,
    },
    cmd::{
        all::call_all_meta,
        reproducible_builds::{
            BuildOutcome, PackedSource, SCHEMA_VERSION, docker_build, init_config, local_build,
            release_notes,
        },
    },
    folder_structure::{setup_workspace, strip_path},
};
use multiversx_sc_meta_lib::{
    cli::{BuildArgs, ContractCliAction},
    tools::find_current_workspace,
};

const TEST_DIR_PATH_LOCAL: &str = "contracts/test-reproducible-build/local";
const TEST_DIR_PATH_DOCKER: &str = "contracts/test-reproducible-build/full";
const CONTRACTS: [&str; 2] = ["adder", "crypto-kitties"];

/// Local-build half of the reproducible build test.
///
/// Copies adder and crypto-kitties (each with their own config), strips path
/// deps, then runs `local_build` per contract and prints release notes.
///
/// Run with:
///   cargo test -p multiversx-sc-meta --features repro-build-test repro_build_local
#[test]
#[cfg_attr(not(feature = "repro-build-test"), ignore)]
fn repro_build_local() {
    let workspace = find_current_workspace().unwrap();
    let test_dir = workspace.join(TEST_DIR_PATH_LOCAL);

    setup_build_dir(&workspace, &test_dir);

    for contract in CONTRACTS {
        let contract_dir = test_dir.join(contract);
        let target_dir_str = test_dir.join("sc-target-tmp").to_str().unwrap().to_string();

        // Pre-build without --locked to generate Cargo.lock files in the source
        // folder; they are then copied to the build root and the locked build succeeds.
        call_all_meta(&AllArgs {
            path: Some(contract_dir.to_str().unwrap().to_string()),
            command: ContractCliAction::Build(BuildArgs {
                locked: false,
                wasm_opt: false,
                target_dir_wasm: Some(target_dir_str.clone()),
                ..Default::default()
            }),
            meta_lib_args: MetaLibArgs {
                target_dir_meta: Some(target_dir_str.clone()),
                ..Default::default()
            },
            ..Default::default()
        });

        local_build(&ReproducibleBuildLocalBuildArgs {
            path: Some(contract_dir.to_str().unwrap().to_string()),
            output: Some("output-local".to_string()),
            build_root: Some(test_dir.join("sc-build-tmp").to_str().unwrap().to_string()),
            target_dir: Some(test_dir.join("sc-target-tmp").to_str().unwrap().to_string()),
            contract: None,
            no_wasm_opt: false,
            overwrite: true,
            packaged_src: None,
        });

        release_notes(&ReleaseNotesArgs {
            artifacts: contract_dir
                .join("output-local")
                .join("artifacts.json")
                .to_str()
                .unwrap()
                .to_string(),
            docker_image: None,
            output: None,
        });

        check_artifacts(
            &contract_dir.join("output-local"),
            expected_contracts(contract),
        );
    }
}

/// Docker-build half of the reproducible build test.
///
/// Copies adder and crypto-kitties (each with their own config), strips path
/// deps, then runs `docker_build` per contract (image and output are read from
/// each contract's config) and prints release notes.
///
/// Requires a working Docker daemon.
/// Run with:
///   cargo test -p multiversx-sc-meta --features repro-build-test repro_build_docker
#[test]
#[cfg_attr(not(feature = "repro-build-test"), ignore)]
fn repro_build_docker() {
    let workspace = find_current_workspace().unwrap();
    let test_dir = workspace.join(TEST_DIR_PATH_DOCKER);

    setup_build_dir(&workspace, &test_dir);

    for contract in CONTRACTS {
        let contract_dir = test_dir.join(contract);
        let target_dir_str = test_dir.join("sc-target-tmp").to_str().unwrap().to_string();

        // Pre-build without --locked to generate Cargo.lock files in the source
        // folder; they are then copied to the build root and the locked build succeeds.
        call_all_meta(&AllArgs {
            path: Some(contract_dir.to_str().unwrap().to_string()),
            command: ContractCliAction::Build(BuildArgs {
                locked: false,
                wasm_opt: false,
                target_dir_wasm: Some(target_dir_str.clone()),
                ..Default::default()
            }),
            meta_lib_args: MetaLibArgs {
                target_dir_meta: Some(target_dir_str.clone()),
                ..Default::default()
            },
            ..Default::default()
        });

        // docker-image, output ("output-rb"), and overwrite are all read from
        // the sc-reproducible-build.toml written per-contract by setup_build_dir.
        docker_build(&ReproducibleBuildBuildArgs {
            project: Some(contract_dir.to_str().unwrap().to_string()),
            docker_image: None,
            output: None,
            overwrite: false,
            contract: None,
            no_wasm_opt: false,
            build_root: None,
            no_docker_interactive: true,
            no_docker_tty: true,
            no_default_platform: false,
            cargo_verbose: false,
        });

        release_notes(&ReleaseNotesArgs {
            artifacts: contract_dir
                .join("output-rb")
                .join("artifacts.json")
                .to_str()
                .unwrap()
                .to_string(),
            docker_image: None,
            output: None,
        });

        check_artifacts(
            &contract_dir.join("output-rb"),
            expected_contracts(contract),
        );
    }
}

/// Clears `build_dir`, copies each contract into it, strips framework
/// `path = "..."` deps across the whole tree, then writes a default
/// `sc-reproducible-build.toml` inside each contract subdirectory.
fn expected_contracts(contract: &str) -> &'static [&'static str] {
    match contract {
        "adder" => &["adder"],
        "crypto-kitties" => &["kitty-auction", "kitty-genetic-alg", "kitty-ownership"],
        other => panic!("unknown contract: {other}"),
    }
}

/// Verifies the artifacts produced by a reproducible build:
/// - `artifacts.json` exists, parses correctly, and lists exactly the expected contracts
/// - every contract has a non-empty codehash
/// - every contract's output folder contains `.wasm`, `.codehash.txt`, and `.source.json`
/// - every `.source.json` parses correctly, matches schema version, and has source entries
fn check_artifacts(output_dir: &Path, expected_contracts: &[&str]) {
    let artifacts_path = output_dir.join("artifacts.json");
    assert!(
        artifacts_path.exists(),
        "artifacts.json not found: {}",
        artifacts_path.display()
    );

    let outcome: BuildOutcome = serde_json::from_str(&fs::read_to_string(&artifacts_path).unwrap())
        .unwrap_or_else(|e| panic!("failed to parse artifacts.json: {e}"));

    let mut actual: Vec<&str> = outcome.contracts.keys().map(String::as_str).collect();
    actual.sort_unstable();
    assert_eq!(
        actual, expected_contracts,
        "contracts in artifacts.json do not match expected"
    );

    for (contract_stem, entry) in &outcome.contracts {
        assert!(
            !entry.codehash.is_empty(),
            "empty codehash for {contract_stem}"
        );

        let contract_dir = output_dir.join(contract_stem);
        assert!(
            contract_dir.is_dir(),
            "output folder missing for {contract_stem}: {}",
            contract_dir.display()
        );

        let wasm = contract_dir.join(&entry.artifacts.bytecode);
        assert!(wasm.exists(), ".wasm missing: {}", wasm.display());

        let codehash_txt = contract_dir.join(format!("{contract_stem}.codehash.txt"));
        assert!(
            codehash_txt.exists(),
            ".codehash.txt missing: {}",
            codehash_txt.display()
        );

        let source_json_path = contract_dir.join(&entry.artifacts.src_package);
        assert!(
            source_json_path.exists(),
            ".source.json missing: {}",
            source_json_path.display()
        );

        let packed: PackedSource =
            serde_json::from_str(&fs::read_to_string(&source_json_path).unwrap())
                .unwrap_or_else(|e| panic!("failed to parse {}: {e}", source_json_path.display()));

        assert_eq!(
            packed.schema_version, SCHEMA_VERSION,
            "schema version mismatch for {contract_stem}"
        );
        assert!(
            !packed.entries.is_empty(),
            "no source entries in {}",
            source_json_path.display()
        );
    }
}

fn setup_build_dir(workspace: &Path, build_dir: &Path) {
    if build_dir.exists() {
        fs::remove_dir_all(build_dir).unwrap();
    }
    fs::create_dir_all(build_dir).unwrap();

    let examples = workspace.join("contracts").join("examples");
    for contract in CONTRACTS {
        copy_dir::copy_dir(examples.join(contract), build_dir.join(contract))
            .unwrap_or_else(|e| panic!("failed to copy {contract}: {e}"));
    }

    // The interactor is not needed for the build and pulls in extra dependencies.
    let adder_interactor = build_dir.join("adder").join("interactor");
    if adder_interactor.exists() {
        fs::remove_dir_all(&adder_interactor).unwrap();
    }

    strip_path(build_dir, &["target".to_string()]);

    for contract in CONTRACTS {
        setup_workspace(
            &build_dir.join(contract),
            &["wasm".to_string(), "target".to_string()],
        );
        init_config(&InitConfigArgs {
            path: Some(build_dir.join(contract).to_str().unwrap().to_string()),
            overwrite: true,
        });
    }
}
