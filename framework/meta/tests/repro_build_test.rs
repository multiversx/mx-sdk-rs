use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use multiversx_sc_meta::{
    cli::{
        AllArgs, InitConfigArgs, MetaLibArgs, ReleaseNotesArgs, ReproducibleBuildBuildArgs,
        ReproducibleBuildLocalBuildArgs, SourcePackArgs,
    },
    cmd::{
        all::call_all_meta,
        reproducible_builds::{
            BuildOutcome, PackedSource, SCHEMA_VERSION, SourceFileEntry, SourceMetadata,
            docker_build, init_config, local_build, release_notes, source_pack,
            unpack_packaged_src, unpack_packed_source,
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
const TEST_DIR_PATH_SOURCE_PACK: &str = "contracts/test-reproducible-build/source-pack";
const TEST_DIR_PATH_SOURCE_ROUNDTRIP: &str = "contracts/test-reproducible-build/source-roundtrip";
const TEST_DIR_PATH_SOURCE_ROUNDTRIP_OUT: &str =
    "contracts/test-reproducible-build/source-roundtrip-out";
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

    setup_build_dir(&workspace, &test_dir, &CONTRACTS);

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

    setup_build_dir(&workspace, &test_dir, &CONTRACTS);

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

fn setup_build_dir(workspace: &Path, build_dir: &Path, contracts: &[&str]) {
    if build_dir.exists() {
        fs::remove_dir_all(build_dir).unwrap();
    }
    fs::create_dir_all(build_dir).unwrap();

    let examples = workspace.join("contracts").join("examples");
    for contract in contracts {
        copy_dir::copy_dir(examples.join(contract), build_dir.join(contract))
            .unwrap_or_else(|e| panic!("failed to copy {contract}: {e}"));
    }

    // The interactor is not needed for the build and pulls in extra dependencies.
    let adder_interactor = build_dir.join("adder").join("interactor");
    if adder_interactor.exists() {
        fs::remove_dir_all(&adder_interactor).unwrap();
    }

    strip_path(build_dir, &["target".to_string()]);

    for contract in contracts {
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

/// Packs the adder contract and verifies the resulting `.source.json` is
/// well-formed: correct schema version, correct contract name, non-empty
/// entries, `Cargo.toml` and at least one `src/*.rs` file included.
#[test]
fn source_pack_entries() {
    let workspace = find_current_workspace().unwrap();
    let test_dir = workspace.join(TEST_DIR_PATH_SOURCE_PACK);

    setup_build_dir(&workspace, &test_dir, &["adder"]);

    let adder_dir = test_dir.join("adder");
    source_pack(&SourcePackArgs {
        path: Some(adder_dir.to_str().unwrap().to_string()),
        contract: None,
    });

    let source_json = find_source_json(&adder_dir.join("output"));
    let text = fs::read_to_string(&source_json).unwrap();
    let packed: PackedSource =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("failed to parse source.json: {e}"));

    assert_eq!(packed.schema_version, SCHEMA_VERSION);
    assert_eq!(packed.metadata.contract_name, "adder");
    assert!(!packed.entries.is_empty(), "entries must not be empty");

    let paths: HashSet<&str> = packed.entries.iter().map(|e| e.path.as_str()).collect();
    assert!(paths.contains("Cargo.toml"), "Cargo.toml must be present");
    assert!(
        paths
            .iter()
            .any(|p| p.starts_with("src/") && p.ends_with(".rs")),
        "at least one src/*.rs file must be present; got: {paths:?}"
    );
}

/// Packs adder, then unpacks the `.source.json` and verifies every entry's
/// file is present in the output directory with identical content.
#[test]
fn source_pack_roundtrip() {
    let workspace = find_current_workspace().unwrap();
    let test_dir = workspace.join(TEST_DIR_PATH_SOURCE_ROUNDTRIP);
    let unpack_dir = workspace.join(TEST_DIR_PATH_SOURCE_ROUNDTRIP_OUT);

    setup_build_dir(&workspace, &test_dir, &["adder"]);
    if unpack_dir.exists() {
        fs::remove_dir_all(&unpack_dir).unwrap();
    }

    let adder_dir = test_dir.join("adder");
    source_pack(&SourcePackArgs {
        path: Some(adder_dir.to_str().unwrap().to_string()),
        contract: None,
    });

    let source_json = find_source_json(&adder_dir.join("output"));
    let text = fs::read_to_string(&source_json).unwrap();
    let packed: PackedSource = serde_json::from_str(&text).unwrap();

    let (unpacked_folder, _build_root) = unpack_packaged_src(&source_json, &unpack_dir).unwrap();

    for entry in &packed.entries {
        let original = adder_dir.join(&entry.path);
        let restored = unpacked_folder.join(&entry.path);

        assert!(restored.exists(), "unpacked file missing: {}", entry.path);

        if original.exists() {
            assert_eq!(
                fs::read(&original).unwrap(),
                fs::read(&restored).unwrap(),
                "content mismatch for {}",
                entry.path
            );
        }
    }
}

/// Unpacking a `.source.json` that contains `../` must return an error.
#[test]
fn source_unpack_rejects_parent_dir_path() {
    let packed = make_packed_source_with_entry("../escape.txt");
    let err = unpack_packed_source(&packed, &std::env::temp_dir().join("sc-meta-test-traverse"))
        .unwrap_err();
    assert!(
        err.to_string().contains("path traversal"),
        "expected path traversal error, got: {err:#}"
    );
}

/// Unpacking a `.source.json` that contains an absolute path must return an error.
#[test]
fn source_unpack_rejects_absolute_path() {
    let packed = make_packed_source_with_entry("/etc/passwd");
    let err = unpack_packed_source(&packed, &std::env::temp_dir().join("sc-meta-test-traverse"))
        .unwrap_err();
    assert!(
        err.to_string().contains("absolute path"),
        "expected absolute path error, got: {err:#}"
    );
}

fn find_source_json(output_dir: &Path) -> PathBuf {
    fs::read_dir(output_dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", output_dir.display()))
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".source.json"))
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("no .source.json found in {}", output_dir.display()))
}

fn make_packed_source_with_entry(path: &str) -> PackedSource {
    PackedSource {
        schema_version: SCHEMA_VERSION.to_string(),
        metadata: SourceMetadata {
            contract_name: "test".to_string(),
            contract_version: "0.0.0".to_string(),
            build_metadata: None,
            build_options: None,
        },
        entries: vec![SourceFileEntry {
            path: path.to_string(),
            content: "aGVsbG8=".to_string(),
            module: ".".to_string(),
            dependency_depth: 0,
            is_test_file: false,
        }],
    }
}
