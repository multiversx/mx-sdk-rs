use multiversx_sc_meta::{
    cli::{
        CodehashArgs, CodehashClearArgs, CodehashCliAction, CodehashGenerateArgs,
        CodehashPrintArgs, CodehashTargetArgs, CodehashVerifyArgs,
    },
    cmd::reproducible_builds::codehash,
};
use multiversx_sc_meta_lib::tools::{compute_codehash, find_codehash_files, find_wasm_files};
use std::{
    fs,
    path::{Path, PathBuf},
};

const ADDER_WASM: &str = "tests/cs_tx_cli_test/adder.wasm";

/// Expected Blake2b-256 code hash of `adder.wasm`.
const ADDER_CODEHASH: &str = "3d2cebbce167957486e66abbe9191afd64bdb8a46b2b39143699199313082e58";

/// Base directory for temporary test artefacts (ignored by git).
const TEMP_DIR: &str = "tests/temp/codehash";

/// Creates a fresh test folder containing `a/adder.wasm` and `a/b/adder2.wasm`,
/// plus a `target/adder.wasm` that should be ignorable.
fn setup(test_name: &str) -> PathBuf {
    let root = PathBuf::from(TEMP_DIR).join(test_name);
    let _ = fs::remove_dir_all(&root);
    for sub in ["a", "a/b", "target"] {
        fs::create_dir_all(root.join(sub)).unwrap();
    }
    fs::copy(ADDER_WASM, root.join("a/adder.wasm")).unwrap();
    fs::copy(ADDER_WASM, root.join("a/b/adder2.wasm")).unwrap();
    fs::copy(ADDER_WASM, root.join("target/adder.wasm")).unwrap();
    root
}

fn target(path: &Path) -> CodehashTargetArgs {
    CodehashTargetArgs {
        path: Some(path.to_str().unwrap().to_string()),
        no_recursive: false,
        ignore: vec!["target".to_string()],
    }
}

fn run(action: CodehashCliAction) {
    codehash(&CodehashArgs { command: action });
}

fn generate(path: &Path) {
    run(CodehashCliAction::Generate(CodehashGenerateArgs {
        target: target(path),
        ..Default::default()
    }));
}

#[test]
fn codehash_compute() {
    assert_eq!(compute_codehash(Path::new(ADDER_WASM)), ADDER_CODEHASH);
}

#[test]
fn codehash_find_files() {
    let root = setup("find");
    let ignore = vec!["target".to_string()];

    assert_eq!(find_wasm_files(&root, true, &ignore).len(), 2);
    assert_eq!(find_wasm_files(&root, true, &[]).len(), 3);
    assert_eq!(find_wasm_files(&root.join("a"), false, &[]).len(), 1);
    assert_eq!(
        find_wasm_files(&root.join("a/adder.wasm"), true, &[]),
        vec![root.join("a/adder.wasm")]
    );
    assert!(find_codehash_files(&root, true, &[]).is_empty());
}

#[test]
fn codehash_generate_folder_recursive() {
    let root = setup("generate_folder");
    generate(&root);

    for file in ["a/adder.codehash.txt", "a/b/adder2.codehash.txt"] {
        assert_eq!(fs::read_to_string(root.join(file)).unwrap(), ADDER_CODEHASH);
    }
    // ignored folder is not processed
    assert!(!root.join("target/adder.codehash.txt").exists());
}

#[test]
fn codehash_generate_no_recursive() {
    let root = setup("generate_no_recursive");
    run(CodehashCliAction::Generate(CodehashGenerateArgs {
        target: CodehashTargetArgs {
            no_recursive: true,
            ..target(&root.join("a"))
        },
        ..Default::default()
    }));

    assert!(root.join("a/adder.codehash.txt").exists());
    assert!(!root.join("a/b/adder2.codehash.txt").exists());
}

#[test]
fn codehash_generate_single_file_output_dir() {
    let root = setup("generate_single_file");
    let out_dir = root.join("out");
    run(CodehashCliAction::Generate(CodehashGenerateArgs {
        target: target(&root.join("a/adder.wasm")),
        output_dir: Some(out_dir.to_str().unwrap().to_string()),
        skip_existing: false,
    }));

    assert_eq!(
        fs::read_to_string(out_dir.join("adder.codehash.txt")).unwrap(),
        ADDER_CODEHASH
    );
    // nothing written next to the wasm, and the other contract was not processed
    assert!(!root.join("a/adder.codehash.txt").exists());
    assert!(!root.join("a/b/adder2.codehash.txt").exists());
}

#[test]
fn codehash_generate_skip_existing() {
    let root = setup("generate_skip_existing");
    let existing = root.join("a/adder.codehash.txt");
    fs::write(&existing, "keep-me").unwrap();

    run(CodehashCliAction::Generate(CodehashGenerateArgs {
        target: target(&root),
        output_dir: None,
        skip_existing: true,
    }));

    assert_eq!(fs::read_to_string(&existing).unwrap(), "keep-me");
    assert_eq!(
        fs::read_to_string(root.join("a/b/adder2.codehash.txt")).unwrap(),
        ADDER_CODEHASH
    );
}

#[test]
fn codehash_print_writes_nothing() {
    let root = setup("print");
    run(CodehashCliAction::Print(CodehashPrintArgs {
        target: target(&root),
        ..Default::default()
    }));

    assert!(find_codehash_files(&root, true, &[]).is_empty());
}

/// Verify exits the process on failure, so only the passing cases can run in-process.
#[test]
fn codehash_verify_ok() {
    let root = setup("verify_ok");
    generate(&root);

    run(CodehashCliAction::Verify(CodehashVerifyArgs {
        target: target(&root),
        allow_missing: false,
    }));

    // missing hash files are fine when explicitly allowed
    let other = setup("verify_allow_missing");
    run(CodehashCliAction::Verify(CodehashVerifyArgs {
        target: target(&other),
        allow_missing: true,
    }));
}

#[test]
fn codehash_clear() {
    let root = setup("clear");
    generate(&root);
    let ignored_hash = root.join("target/adder.codehash.txt");
    fs::write(&ignored_hash, "x").unwrap();

    // dry run deletes nothing
    run(CodehashCliAction::Clear(CodehashClearArgs {
        target: target(&root),
        dry_run: true,
    }));
    assert_eq!(find_codehash_files(&root, true, &[]).len(), 3);

    // single file
    run(CodehashCliAction::Clear(CodehashClearArgs {
        target: target(&root.join("a/adder.wasm")),
        dry_run: false,
    }));
    assert!(!root.join("a/adder.codehash.txt").exists());
    assert!(root.join("a/b/adder2.codehash.txt").exists());

    // whole folder, respecting --ignore
    run(CodehashCliAction::Clear(CodehashClearArgs {
        target: target(&root),
        dry_run: false,
    }));
    assert!(!root.join("a/b/adder2.codehash.txt").exists());
    assert!(ignored_hash.exists());

    // .wasm files are never deleted
    assert_eq!(find_wasm_files(&root, true, &[]).len(), 3);
}
