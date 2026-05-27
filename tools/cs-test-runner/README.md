# cs-test-runner

A tool that builds the necessary smart contracts, starts the chain simulator, runs all chain simulator integration tests, and shuts the simulator down afterwards.

It is used both locally during development and in CI.

## Usage

From the workspace root:

```sh
cargo run -p cs-test-runner
```

The tool will:

1. Build each contract listed in `CS_TESTS` (those with non-empty `build_paths`) using `sc-meta all build`.
2. Start the chain simulator in the background (`sc-meta cs start`). Its output is suppressed.
3. Run every test file listed in `CS_TESTS` with `cargo test -p <package> --test <file> --features chain-simulator-tests`.
4. Stop the chain simulator (`sc-meta cs stop`) on exit, even if a test panics.

Exit code is `0` only if every build and test step succeeds. The number of failures is printed at the end otherwise.

## Prerequisites

- `sc-meta` must be installed and available on `PATH`:
  ```sh
  cargo install --path framework/meta --locked
  ```
- `wasm-opt` must be available on `PATH` (required for contract builds):
  ```sh
  cargo install wasm-opt
  ```
- Docker must be running (the chain simulator runs as a Docker container).
- The chain simulator image must be pulled:
  ```sh
  sc-meta cs install
  ```

## Adding a new test

Edit `CS_TESTS` in `src/main.rs`:

```rust
CsTest {
    build_paths: &["contracts/path/to/my-contract"],
    package: "my-interactor",
    test_file: "my_cs_test",
},
```

- `build_paths`: workspace-relative paths passed to `sc-meta all build`. Leave empty (`&[]`) if no wasm build is needed (e.g. system contract tests).
- `package`: the Cargo package name containing the integration test.
- `test_file`: the filename under `tests/` without the `.rs` extension.

The test file must enable the `chain-simulator-tests` feature and annotate each test with `#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]` and `#[serial_test::serial]`.
