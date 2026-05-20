# Reproducible Build — Feature Status in `sc-meta`

Reference implementations:
- Python builder: `mx-sdk-rust-contract-builder/multiversx_sdk_rust_contract_builder/`
- Docker invocation: `mx-sdk-rust-contract-builder/build_with_docker.py`
- mxpy CLI docs: `mx-sdk-py-cli/docs/reproducible-build.md`

---

## 1. `docker-build` subcommand — ✅ DONE

Implemented in `docker_build.rs`. All `DockerBuildArgs` flags are present:
`--docker-image`, `--project`, `--output`, `--contract`, `--no-wasm-opt`, `--build-root`,
`--no-docker-interactive`, `--no-docker-tty`, `--no-default-platform`, `--cargo-verbose`, `--force`.

Docker command uses `--platform linux/amd64`, `--user <uid>:<gid>` (Unix), `--rm`, all five
volume mounts, `CARGO_TERM_VERBOSE` env var, and a `docker info` availability check.
Cargo cache dirs under `/tmp/multiversx_sc_meta_builder/` are created before the run.

---

## 2. `artifacts.json` output from `local-build` — ✅ DONE

`build_outcome.rs` implements `ArtifactsBuildMetadata`, `ArtifactsBuildOptions`,
`ContractOutcomeEntry`, and `BuildOutcome`. `local_build.rs` calls `outcome.gather()`
per contract and `outcome.save()` at the end, writing `<output>/artifacts.json`
with the required structure.

---

## 3. `BuildMetadata` population in `.source.json` — ⚠️ PARTIAL

`ArtifactsBuildMetadata::detect()` is implemented and correctly populates `buildMetadata`
in `artifacts.json` (reads `BUILD_METADATA_*` env vars in Docker; runs `rustc --version`,
`wasm-opt --version`, etc. on local builds).

**Missing:** `source_pack_contract` in `source_pack.rs` hardcodes `build_metadata: None`,
so `.source.json` files never contain `buildMetadata`. The detection logic from
`ArtifactsBuildMetadata::detect()` needs to be passed through and written there too.

---

## 4. `buildRootFolder` reflects actual build path in `.source.json` — ✅ DONE (with caveat)

`source_pack_contract` is called with `&build_root` (the copied build directory), not the
original `project_folder`. The value is stored as `buildOptions.buildRootFolder` in
`.source.json`.

**Caveat (macOS):** `build_root.canonicalize()` in `local_build.rs` resolves
`/tmp/sc-build` → `/private/tmp/sc-build` on macOS, so the stored path differs from
Linux. This can cause codehash mismatches if the path is embedded in panic messages or
debug symbols by `rustc`. Not an issue inside Docker (Linux only).

---

## 5. Dockerfile with `sc-meta` as entrypoint — ✅ DONE

`framework/meta/Dockerfile` exists. It pins `VERSION_RUST`, `VERSION_SC_META`,
`VERSION_WASM_OPT`, installs from crates.io, sets all `BUILD_METADATA_*` env vars, and
uses `sc-meta reproducible-build local-build` as the entrypoint. A `Dockerfile.local`
variant builds from the local source tree for development.

---

## 6. `--packaged-src` mode in `local-build` — ✅ DONE

`local_build.rs` accepts `--packaged-src <path>`. When set, it calls `unpack_packaged_src`
(in `source_unpack.rs`) to extract the `.source.json` to `HARDCODED_UNWRAP_FOLDER`
(`/tmp/unwrapped/`), reads `buildRootFolder` from the JSON metadata, and proceeds with
the standard build pipeline from that folder. `--path` and `--packaged-src` are validated
as mutually exclusive.

---

## 7. `CARGO_NET_GIT_FETCH_WITH_CLI=true` in build subprocess — ✅ DONE

Added `.env("CARGO_NET_GIT_FETCH_WITH_CLI", "true")` to the `Command` in
`call_in_dir` (`contract_meta_call.rs`). This is the single spawn site for all
`cargo run` calls through `ContractMetaCall`, so it covers both reproducible builds
and regular `sc-meta all build` invocations.

---

## 8. Output folder non-empty guard — ✅ DONE

Both `local_build.rs` and `docker_build.rs` check for a non-empty output folder and abort
with a clear error message. Both accept `--force` to wipe the folder instead of aborting.

---

## Summary

| # | Feature | Status |
|---|---|---|
| 1 | `docker-build` subcommand + Docker invocation | ✅ Done |
| 2 | `artifacts.json` per-build summary | ✅ Done |
| 3 | `BuildMetadata` in `.source.json` | ⚠️ Partial (`artifacts.json` ✅, `.source.json` ❌) |
| 4 | `buildRootFolder` = actual build path in `.source.json` | ✅ Done (macOS `/private/tmp` caveat) |
| 5 | Dockerfile with `sc-meta` as entrypoint | ✅ Done |
| 6 | `--packaged-src` build-from-JSON mode | ✅ Done |
| 7 | `CARGO_NET_GIT_FETCH_WITH_CLI=true` in build subprocess | ✅ Done |
| 8 | Output-folder non-empty guard + `--force` | ✅ Done |
