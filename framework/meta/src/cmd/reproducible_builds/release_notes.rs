use std::{fs, path::Path};

use crate::cli::ReleaseNotesArgs;

use super::build_outcome::BuildOutcome;

/// CLI entry point for `sc-meta reproducible-build release-notes`.
///
/// Reads `artifacts.json` produced by a previous `rb build` or `rb local-build`
/// and writes a Markdown fragment suitable for a GitHub release body.
///
/// Output format mirrors the Python release-notes script in `mx-sc-actions`:
///
/// ```text
/// Built using Docker image: **multiversx/sdk-rust-contract-builder:<tag>**.
///
/// ## Codehashes (blake2b):
/// **adder.wasm**: `<hex>`
/// **adder_dbg.wasm**: `<hex>`
/// ```
pub fn release_notes(args: &ReleaseNotesArgs) {
    let artifacts_path = Path::new(&args.artifacts);
    let text = fs::read_to_string(artifacts_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", artifacts_path.display()));
    let outcome: BuildOutcome = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", artifacts_path.display()));

    let notes = build_notes(&outcome, args.docker_image.as_deref());

    match &args.output {
        Some(path) => {
            fs::write(path, &notes).unwrap_or_else(|e| panic!("Failed to write {path}: {e}"));
            println!("Release notes written to: {path}");
        }
        None => print!("{notes}"),
    }
}

fn build_notes(outcome: &BuildOutcome, docker_image: Option<&str>) -> String {
    let mut out = String::new();

    if let Some(image) = docker_image {
        out.push_str(&format!("Built using Docker image: **{image}**.\n\n"));
    }

    out.push_str("## Codehashes (blake2b):\n");

    for entry in outcome.contracts.values() {
        out.push_str(&format!(
            "**{}**: `{}`\n",
            entry.artifacts.bytecode, entry.codehash
        ));
    }

    out
}
