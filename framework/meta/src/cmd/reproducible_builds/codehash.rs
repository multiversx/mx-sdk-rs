use std::{
    fs,
    path::{Path, PathBuf},
};

use multiversx_sc_meta_lib::tools::{
    codehash_path_for, compute_codehash, find_codehash_files, find_wasm_files,
};

use crate::cli::{
    CodehashArgs, CodehashClearArgs, CodehashCliAction, CodehashGenerateArgs, CodehashPrintArgs,
    CodehashPrintFormat, CodehashTargetArgs, CodehashVerifyArgs,
};

pub fn codehash(args: &CodehashArgs) {
    match &args.command {
        CodehashCliAction::Generate(a) => generate(a),
        CodehashCliAction::Print(a) => print(a),
        CodehashCliAction::Verify(a) => verify(a),
        CodehashCliAction::Clear(a) => clear(a),
    }
}

fn exit_with_error(msg: String) -> ! {
    eprintln!("Error: {msg}");
    std::process::exit(1);
}

fn collect_targets(target: &CodehashTargetArgs) -> Vec<PathBuf> {
    let path = Path::new(target.path.as_deref().unwrap_or("."));
    if !path.exists() {
        exit_with_error(format!("path does not exist: {}", path.display()));
    }
    let files = find_wasm_files(path, !target.no_recursive, &target.ignore);
    if files.is_empty() {
        exit_with_error(format!("no .wasm files found in {}", path.display()));
    }
    files
}

fn generate(args: &CodehashGenerateArgs) {
    let output_dir = args.output_dir.as_deref().map(Path::new);
    if let Some(dir) = output_dir {
        fs::create_dir_all(dir)
            .unwrap_or_else(|e| exit_with_error(format!("creating {}: {e}", dir.display())));
    }

    for wasm in collect_targets(&args.target) {
        let mut dest = codehash_path_for(&wasm);
        if let Some(dir) = output_dir {
            dest = dir.join(dest.file_name().unwrap());
        }
        if args.skip_existing && dest.exists() {
            println!("Skipped (exists): {}", dest.display());
            continue;
        }
        let hash = compute_codehash(&wasm);
        fs::write(&dest, &hash)
            .unwrap_or_else(|e| exit_with_error(format!("writing {}: {e}", dest.display())));
        println!("{hash}  {}", dest.display());
    }
}

fn print(args: &CodehashPrintArgs) {
    let entries: Vec<(PathBuf, String)> = collect_targets(&args.target)
        .into_iter()
        .map(|wasm| {
            let hash = compute_codehash(&wasm);
            (wasm, hash)
        })
        .collect();

    match args.format {
        CodehashPrintFormat::Plain => {
            for (path, hash) in &entries {
                println!("{hash}  {}", path.display());
            }
        }
        CodehashPrintFormat::Json => {
            let json: Vec<_> = entries
                .iter()
                .map(|(path, hash)| {
                    serde_json::json!({ "path": path.to_string_lossy(), "codehash": hash })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
    }
}

fn verify(args: &CodehashVerifyArgs) {
    let mut failures = 0;
    for wasm in collect_targets(&args.target) {
        let codehash_path = codehash_path_for(&wasm);
        let Ok(expected) = fs::read_to_string(&codehash_path) else {
            if args.allow_missing {
                println!("MISSING (allowed): {}", wasm.display());
            } else {
                println!("MISSING: {}", codehash_path.display());
                failures += 1;
            }
            continue;
        };
        let actual = compute_codehash(&wasm);
        if expected.trim().eq_ignore_ascii_case(&actual) {
            println!("OK: {}", wasm.display());
        } else {
            println!(
                "MISMATCH: {} (file: {}, computed: {actual})",
                wasm.display(),
                expected.trim()
            );
            failures += 1;
        }
    }
    if failures > 0 {
        exit_with_error(format!("{failures} codehash check(s) failed"));
    }
}

fn clear(args: &CodehashClearArgs) {
    let path = Path::new(args.target.path.as_deref().unwrap_or("."));
    if !path.exists() {
        exit_with_error(format!("path does not exist: {}", path.display()));
    }
    let files = find_codehash_files(path, !args.target.no_recursive, &args.target.ignore);
    if files.is_empty() {
        println!("No codehash files found in {}", path.display());
        return;
    }
    for file in &files {
        if args.dry_run {
            println!("Would delete: {}", file.display());
        } else {
            fs::remove_file(file)
                .unwrap_or_else(|e| exit_with_error(format!("deleting {}: {e}", file.display())));
            println!("Deleted: {}", file.display());
        }
    }
}
