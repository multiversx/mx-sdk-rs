use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use multiversx_sc_meta_lib::cargo_toml::CargoTomlContents;
use multiversx_sc_meta_lib::cli::{BuildArgs, ContractCliAction};
use multiversx_sc_meta_lib::tools::generate_codehashes_in_output;

use crate::cli::{AllArgs, LocalBuildArgs, MetaLibArgs};
use crate::cmd::all::call_contract_meta;
use crate::folder_structure::{RelevantDirectories, RelevantDirectory};

use super::build_outcome::{ArtifactsBuildMetadata, ArtifactsBuildOptions, BuildOutcome};
use super::source::source_pack_contract;

/// Mirrors the Python `build_project` pipeline, but runs locally instead of inside Docker.
///
/// Steps:
/// 1. Discover all contracts (`multiversx.json` markers).
/// 2. Copy the project to `--build-root` (wipes it first; skips `target/` dirs).
/// 3. Snapshot `Cargo.lock` files.
/// 4. For each contract (filtered by `--contract` if set):
///    a. Clean build artifacts (`wasm/target/`, `meta/target/`, `output/`).
///    b. Run `sc-meta all build --target-dir <dir> --locked`.
///    c. Write the `.source.json` via `source_pack_contract`.
///    d. Clean again, keeping `output/`.
///    e. Copy `output/` to `--output/<contract_name>/`.
/// 5. Verify no `Cargo.lock` file changed (enforces `--locked`).
pub fn local_build(args: &LocalBuildArgs) {
    let project_folder = resolve_path(args.path.as_deref());

    let output_folder = {
        fs::create_dir_all(&args.output).unwrap();
        Path::new(&args.output).canonicalize().unwrap()
    };

    let build_root = PathBuf::from(args.build_root.as_deref().unwrap_or("/tmp/sc-build"));

    let cargo_target_dir = {
        let p = args.target_dir.as_deref().unwrap_or("/tmp/sc-target");
        fs::create_dir_all(p).unwrap();
        Path::new(p).canonicalize().unwrap()
    };

    // 1. Discover contracts
    let dirs = RelevantDirectories::find_all(&project_folder, &["target".to_string()]);
    if dirs.iter_contract_crates().count() == 0 {
        println!(
            "No contracts found (no multiversx.json) under: {}",
            project_folder.display()
        );
        return;
    }

    dirs.ensure_distinct_contract_names();

    // 2. Copy project to build root (wipes first, skips target/)
    println!("Copying project to build root: {}", build_root.display());
    copy_project_to_build_root(&project_folder, &build_root);

    // Canonicalize after the directory exists to resolve symlinks (e.g. /tmp → /private/tmp on macOS).
    // This ensures all subsequent path operations use the same resolved prefix.
    let build_root = build_root.canonicalize().unwrap();

    // 3. Snapshot Cargo.lock files
    let locks_before = snapshot_cargo_locks(&build_root);

    let target_dir_str = cargo_target_dir.to_string_lossy().into_owned();
    let all_args = AllArgs {
        command: ContractCliAction::Build(BuildArgs {
            locked: true,
            wasm_opt: !args.no_wasm_opt,
            target_dir_wasm: Some(target_dir_str.clone()),
            ..Default::default()
        }),
        meta_lib_args: MetaLibArgs {
            target_dir_meta: Some(target_dir_str),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut outcome = BuildOutcome::new(
        ArtifactsBuildMetadata::detect(),
        ArtifactsBuildOptions {
            package_whole_project_src: true,
            specific_contract: args.contract.clone(),
            no_wasm_opt: args.no_wasm_opt,
            build_root_folder: build_root.to_string_lossy().into_owned(),
        },
    );

    // 4. Build each contract
    for dir in dirs.iter_contract_crates() {
        let cargo_toml = CargoTomlContents::load_from_file(dir.path.join("Cargo.toml"));
        let contract_name = cargo_toml.package_name();

        if let Some(filter) = args.contract.as_deref() {
            if contract_name != filter {
                println!("Skipping: {contract_name}");
                continue;
            }
        }

        let relative = dir.path.strip_prefix(&project_folder).unwrap();
        let build_contract_folder = build_root.join(relative);
        let output_subfolder = output_folder.join(&contract_name);
        fs::create_dir_all(&output_subfolder).unwrap();

        println!("Building: {contract_name}");

        // a. Clean (remove output/ too)
        clean_contract(&build_contract_folder, true);

        // b. Build
        let build_dir = RelevantDirectory {
            path: build_contract_folder.clone(),
            ..dir.clone()
        };
        call_contract_meta(&build_dir, &all_args);

        // b2. Generate codehash for each .wasm in output/
        generate_codehashes_in_output(&build_contract_folder.join("output"));

        // c. Pack source into build_contract_folder/output/
        source_pack_contract(
            &build_root,
            &build_contract_folder,
            args.contract.as_deref(),
        );

        // d. Clean, keep output/
        clean_contract(&build_contract_folder, false);

        // e. Copy output/ to parent output subfolder
        copy_dir_contents(&build_contract_folder.join("output"), &output_subfolder);

        // f. Gather artifacts for artifacts.json
        outcome.gather(&contract_name, &output_subfolder);

        println!("Output: {}", output_subfolder.display());
    }

    // 5. Verify Cargo.lock unchanged
    let locks_after = snapshot_cargo_locks(&build_root);
    check_cargo_locks_unchanged(&locks_before, &locks_after);

    // 6. Write artifacts.json to the output folder root
    outcome.save(&output_folder);
}

fn resolve_path(path: Option<&str>) -> PathBuf {
    let p = path.unwrap_or(".");
    Path::new(p)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
}

/// Wipes `build_root` and copies the entire `project_folder` into it.
/// Skips `target/` directories to avoid copying large build artifacts.
fn copy_project_to_build_root(project_folder: &Path, build_root: &Path) {
    if build_root.exists() {
        fs::remove_dir_all(build_root).unwrap();
    }
    copy_dir_skip_target(project_folder, build_root);
}

fn copy_dir_skip_target(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    let Ok(read_dir) = fs::read_dir(src) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if name == "target" {
            continue;
        }
        let dest = dst.join(&name);
        if path.is_symlink() {
            let link_target = fs::read_link(&path).unwrap();
            #[cfg(unix)]
            std::os::unix::fs::symlink(&link_target, &dest).unwrap_or_else(|_| {
                eprintln!("Warning: could not create symlink: {}", dest.display())
            });
        } else if path.is_dir() {
            copy_dir_skip_target(&path, &dest);
        } else {
            fs::copy(&path, &dest).unwrap();
        }
    }
}

fn copy_dir_contents(src: &Path, dst: &Path) {
    if !src.is_dir() {
        return;
    }
    fs::create_dir_all(dst).unwrap();
    let Ok(read_dir) = fs::read_dir(src) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        let dest = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_contents(&path, &dest);
        } else {
            fs::copy(&path, &dest).unwrap();
        }
    }
}

/// Removes `wasm/target/` and `meta/target/` inside `contract_folder`.
/// If `clean_output` is true, also removes `output/`.
fn clean_contract(contract_folder: &Path, clean_output: bool) {
    let _ = fs::remove_dir_all(contract_folder.join("wasm").join("target"));
    let _ = fs::remove_dir_all(contract_folder.join("meta").join("target"));
    if clean_output {
        let _ = fs::remove_dir_all(contract_folder.join("output"));
    }
}

fn snapshot_cargo_locks(root: &Path) -> HashMap<PathBuf, Vec<u8>> {
    let mut map = HashMap::new();
    collect_cargo_locks(root, &mut map);
    map
}

fn collect_cargo_locks(dir: &Path, map: &mut HashMap<PathBuf, Vec<u8>>) {
    let Ok(read_dir) = fs::read_dir(dir) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().map(|n| n != "target").unwrap_or(true) {
                collect_cargo_locks(&path, map);
            }
        } else if path.file_name().map(|n| n == "Cargo.lock").unwrap_or(false) {
            if let Ok(contents) = fs::read(&path) {
                map.insert(path, contents);
            }
        }
    }
}

fn check_cargo_locks_unchanged(
    before: &HashMap<PathBuf, Vec<u8>>,
    after: &HashMap<PathBuf, Vec<u8>>,
) {
    let mut any_changed = false;
    for (path, before_contents) in before {
        match after.get(path) {
            Some(after_contents) if before_contents != after_contents => {
                eprintln!("Error: Cargo.lock changed during build: {}", path.display());
                any_changed = true;
            }
            None => {
                eprintln!("Warning: Cargo.lock disappeared: {}", path.display());
            }
            _ => {}
        }
    }
    if any_changed {
        panic!("One or more Cargo.lock files changed during build. Use --locked to prevent this.");
    }
}
