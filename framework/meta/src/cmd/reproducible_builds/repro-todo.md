# Reproducible Build — Missing Features in `sc-meta`

Reference implementations:
- Python builder: `mx-sdk-rust-contract-builder/multiversx_sdk_rust_contract_builder/`
- Docker invocation: `mx-sdk-rust-contract-builder/build_with_docker.py`
- mxpy CLI docs: `mx-sdk-py-cli/docs/reproducible-build.md`

---

## 1. `docker-build` subcommand *(most critical)*

There is no Docker-based build command at all. `local-build` runs purely on the host. A new `docker-build` subcommand is needed in `ReproducibleBuildCliAction`, mirroring `mxpy contract reproducible-build` and `build_with_docker.py`.

**Required `DockerBuildArgs` flags:**

| Flag | Default | Notes |
|---|---|---|
| `--docker-image` | *required* | Pinned tag e.g. `multiversx/sdk-rust-contract-builder:v8.0.0` |
| `--project` | cwd | Host path, mounted as `/project` |
| `--output` | `<project>/output-docker/` | Host path, mounted as `/output` |
| `--contract` | *(all)* | Filter by name from `Cargo.toml` |
| `--no-wasm-opt` | false | Forwarded to container entrypoint |
| `--build-root` | *(container default)* | Forwarded to container entrypoint |
| `--no-docker-interactive` | false | Omit `--interactive` (CI mode) |
| `--no-docker-tty` | false | Omit `--tty` (CI mode) |
| `--no-default-platform` | false | Skip `--platform linux/amd64` |
| `--cargo-verbose` | false | Sets `CARGO_TERM_VERBOSE=true` in container env |

**Docker command construction** (`std::process::Command`):
```
docker run
  [--platform linux/amd64]           # unless --no-default-platform
  [--interactive] [--tty]            # unless --no-docker-*
  --user <uid>:<gid>                 # from libc::getuid/getgid on Unix; skip/warn on Windows
  --rm
  --volume <project>:/project
  --volume <output>:/output
  --volume <cargo-target-dir>:/rust/cargo-target-dir
  --volume <cargo-registry>:/rust/registry
  --volume <cargo-git>:/rust/git
  --env CARGO_TERM_VERBOSE=false
  <image>
  --project project                  # container-side path
  [--contract <name>]
  [--no-wasm-opt]
  [--build-root <root>]
```

The cargo cache volumes (`/tmp/multiversx_sdk_rust_contract_builder/{cargo-target-dir,cargo-registry,cargo-git}`) should be created on the host before running, mirroring `build_with_docker.py`.

**Docker availability check:** Run `docker info` (or `docker --version`) before attempting; emit a clear error if absent.

---

## 2. `artifacts.json` output from `local-build`

The Python builder writes `<output>/artifacts.json` containing build metadata, build options, and per-contract entries (version, codehash, wasm/abi/source paths). `local_build.rs` produces none of this. The file is consumed by downstream tooling and the GitHub Actions release workflow (for blake2b hash notes in release descriptions).

**Required structure** (mirrors `BuildOutcome.to_dict()`):
```json
{
  "buildMetadata": {
    "versionRust": "…",
    "versionScTool": "…",
    "versionWasmOpt": "…",
    "targetPlatform": "…"
  },
  "buildOptions": {
    "specificContract": null,
    "noWasmOpt": false,
    "buildRootFolder": "…"
  },
  "contracts": {
    "adder": {
      "version": "0.0.0",
      "codehash": "…",
      "wasmPath": "…",
      "abiPath": "…",
      "srcPackagePath": "…"
    }
  }
}
```

---

## 3. `BuildMetadata` population in `.source.json`

`source.rs` always emits `BuildMetadata::default()` (all fields `None`). Both `local-build` and `docker-build` must populate these fields:

- **`versionRust`** — parse from `rustc --version`
- **`versionScTool`** — `sc-meta --version` or `env!("CARGO_PKG_VERSION")`
- **`versionWasmOpt`** — parse from `wasm-opt --version` (when wasm-opt is enabled)
- **`targetPlatform`** — `linux/amd64` when in Docker; on `local-build` use the host target triple (or `linux/amd64` if forced)

In the Docker case these come from the Dockerfile's `ENV BUILD_METADATA_*` variables set at image build time. The `local-build` path must detect them at runtime by invoking the tools.

---

## 4. `buildRootFolder` should reflect the actual build path

In `source_pack_contract`, `buildRootFolder` is currently set to the host `project_folder` path. When building inside Docker (or `local-build` copying to `/tmp/sc-build`), this must be the **build-root path** — the path that was used when `sc-meta all build` ran — so that a downstream verifier can reproduce the exact layout.

`source_pack_contract` should accept a `build_root: &Path` argument and write that into `BuildOptions::build_root_folder` instead of `project_folder`.

---

## 5. Dockerfile with `sc-meta` as entrypoint

The existing `Dockerfile` in `mx-sdk-rust-contract-builder` uses Python's `main.py` as its entrypoint. To make `sc-meta reproducible-build docker-build` self-contained, a Dockerfile whose entrypoint is `sc-meta` is needed:

```dockerfile
ENTRYPOINT ["sc-meta", "reproducible-build", "local-build", \
    "--output", "/output", \
    "--target-dir", "/rust/cargo-target-dir"]
```

The Dockerfile must pin the same toolchain versions and set the same `BUILD_METADATA_*` env vars as the Python builder's Dockerfile. Decision needed: does this live in `mx-sdk-rs` (e.g. `framework/meta/Dockerfile`) or remain in `mx-sdk-rust-contract-builder`?

---

## 6. `--packaged-src` mode in `local-build`

Build from an existing `.source.json` by unpacking it to a temp folder, mirroring Python's `HARDCODED_UNWRAP_FOLDER = /tmp/unwrapped` path. Not present at all in the current Rust implementation.

This mode is needed for the `mxpy contract verify` / contract verification service flow, which re-builds from the packaged source.

Steps:
1. Parse the `.source.json` file.
2. Unpack all `entries` back to the filesystem under `/tmp/unwrapped/` (base64-decode each file content).
3. Read `metadata.buildOptions.buildRootFolder` to set the correct `--build-root`.
4. Proceed with the standard `local-build` pipeline from that folder.

---

## 7. `CARGO_NET_GIT_FETCH_WITH_CLI=true` in build subprocess

`local_build.rs` calls `call_contract_meta` but never sets this env var. The Python builder sets it explicitly to avoid high memory usage when Cargo fetches registry indexes and git dependencies (see https://github.com/rust-lang/cargo/issues/10583). Should be set as an env override on the build subprocess.

---

## 8. Output folder non-empty guard

`local_build.rs` silently overwrites previous outputs. The Python builder raises an error if the output folder is not empty, preventing accidental mixing of artifacts from different builds. Should add a guard (or a `--force` flag to skip it).

---

## Summary

| # | Gap | Complexity |
|---|---|---|
| 1 | `docker-build` subcommand + Docker invocation | High |
| 2 | `artifacts.json` per-build summary | Medium |
| 3 | `BuildMetadata` detection + population in `.source.json` | Medium |
| 4 | `buildRootFolder` reflects actual build path in `.source.json` | Low |
| 5 | Dockerfile with `sc-meta` as entrypoint | Medium |
| 6 | `--packaged-src` build-from-JSON mode | Medium |
| 7 | `CARGO_NET_GIT_FETCH_WITH_CLI=true` in build subprocess | Low |
| 8 | Output-folder non-empty guard | Low |
