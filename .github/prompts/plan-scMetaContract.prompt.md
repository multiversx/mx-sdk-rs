# Plan: sc-meta tx CLI

## TL;DR
Add `sc-meta tx` as a new top-level subcommand to `framework/meta` covering **seven actions**: **deploy, call, upgrade, query** (maps to `mxpy contract`) and **new, send, sign** (maps to `mxpy tx`). The relay command (relayer co-signing) is deferred to V2. All underlying blockchain logic already exists in `framework/snippets`. The main new work is: CLI arg definitions (clap), glue code connecting CLI args to the interactor's raw APIs, and JSON output formatting.

## Decisions
- V1 commands: `deploy`, `call`, `upgrade`, `query`, `new`, `send`, `sign` (relay deferred to V2; verify/unverify deferred)
- Wallet: PEM file (`--pem`) + JSON keystore (`--keyfile`); no Ledger in V1
- Argument format: mandos-compatible — reuse existing `BytesValue::interpret_from` with `InterpreterContext::new().with_dir(cwd)`; **no custom parser needed**
- Broadcast: opt-in via `--send` (defaults to false); without it, the signed tx JSON is written to `--outfile`/stdout and not broadcast
- `--wait-result` only takes effect when `--send` is also set
- Config: pure CLI flags (`--gateway`, `--chain-id`); no config file
- Guardian/relayer multi-sig: deferred to V2

---

## Phase 1 — CLI Scaffolding (clap + dispatch)

**1a. Add `Tx` variant to `StandaloneCliAction`**
- File: `framework/meta/src/cli/cli_args_standalone.rs`
- Add: `#[command(name = "tx")] Tx(TxCliArgs)`
- Import `TxCliArgs` from new module

**1b. Create args structs**
- New file: `framework/meta/src/cmd/tx/tx_cli_args.rs`
- `TxCliArgs { #[command(subcommand)] command: TxCliAction }`
- `TxCliAction` enum with variants: `Deploy(DeployArgs)`, `Call(CallArgs)`, `Upgrade(UpgradeArgs)`, `Query(QueryArgs)`, `New(NewArgs)`, `Send(SendArgs)`, `Sign(SignArgs)`
- Shared arg groups (use `#[derive(Args)]` + `#[command(flatten)]`):
  - `GatewayArgs { --gateway <url>, --chain-id <str> }`
  - `SenderArgs { --pem <path>, --keyfile <path>, --sender-wallet-index <u32 default 0> }`
  - `TxArgs { --gas-limit <u64>, --gas-price <u64 default 1_000_000_000>, --nonce <u64 optional>, --value <u64 default 0>, --send (bool flag default false), --wait-result (only meaningful with --send), --outfile <path optional> }`
  - `MetadataArgs { --metadata-not-upgradeable, --metadata-not-readable, --metadata-payable, --metadata-payable-by-sc }` (deploy + upgrade only)
- `DeployArgs`: GatewayArgs + SenderArgs + TxArgs + MetadataArgs + `--bytecode <path>`, `--arguments <Vec<String>>`
- `CallArgs`: GatewayArgs + SenderArgs + TxArgs + positional `contract <String>`, `--function <name>`, `--arguments <Vec<String>>`, `--token-transfers <Vec<String>> (flat list: TOKEN AMOUNT TOKEN AMOUNT ...)`
- `UpgradeArgs`: GatewayArgs + SenderArgs + TxArgs + MetadataArgs + positional `contract <String>`, `--bytecode <path>`, `--function <name optional>`, `--arguments <Vec<String>>`
- `QueryArgs`: GatewayArgs only + positional `contract <String>`, `--function <name>`, `--arguments <Vec<String>>`
- `NewArgs`: GatewayArgs + SenderArgs + TxArgs + `--receiver <bech32>`, `--data <str optional>`, `--data-file <path optional>`, `--token-transfers <Vec<String>>`
  - `--data` and `--token-transfers` are mutually exclusive
- `SendArgs`: `--gateway <url>` (no chain-id needed) + `--infile <path>`, `--outfile <path optional>`, `--wait-result`
- `SignArgs`: SenderArgs + `--infile <path>`, `--outfile <path optional>`, `--send` (if set: also broadcast after signing), GatewayArgs (optional, only used if `--send`)

**1c. Add dispatch arm**
- File: `framework/meta/src/cli/cli_standalone_main.rs`
- Add `Some(StandaloneCliAction::Tx(args)) => tx_cli(args).await,`

**1d. Wire module**
- File: `framework/meta/src/cmd/mod.rs`
- Add `pub mod tx;`

---

## Phase 2 — Argument Encoding (reuse BytesValue)

No new file needed. Argument encoding is already fully implemented via `BytesValue::interpret_from`, which handles the mandos expression format identically.

Inline pattern used in each handler:
```rust
let context = InterpreterContext::new().with_dir(std::env::current_dir().unwrap());
let encoded: Vec<Vec<u8>> = args.arguments.iter()
    .map(|s| BytesValue::interpret_from(s.as_str(), &context).value)
    .collect();
```

- `BytesValue` — `framework/scenario/src/scenario/model/value/bytes_value.rs`
- `InterpreterContext` — `sdk/scenario-format/src/interpret_trait.rs` (has `with_dir(PathBuf)`)
- Supported expressions: `0x{hex}`, `str:{text}`, `addr:{bech32}`, `true`/`false`, decimal integers, `file:{path}`, `mxsc:{path}`, etc.
- Both types are re-exported via `multiversx_sc_snippets::imports::*`
- **`arg_parser.rs` is not a planned file.**

---

## Phase 3 — Interactor + Wallet Setup (`interactor_setup.rs`)

- File: `framework/meta/src/cmd/tx/interactor_setup.rs`
- `pub async fn build_interactor(gateway: &str) -> Interactor`
  - Calls `Interactor::new(gateway_url).await`
- `pub async fn load_wallet_and_register(interactor: &mut Interactor, sender_args: &SenderArgs) -> Address`
  - If `--pem`: `Wallet::from_pem_file(path)`; if `--keyfile`: `Wallet::from_keystore_secret(path, password)` (prompt stdin)
  - `interactor.register_wallet(wallet).await` → sender `Address`
- Note: `Interactor`, `Wallet`, `Address` from `multiversx_sc_snippets`; already a dep of `framework/meta`

---

## Phase 4A — Deploy Handler (`tx_deploy.rs`)

- File: `framework/meta/src/cmd/tx/tx_deploy.rs`
- `pub async fn tx_deploy(args: &DeployArgs) -> TxOutput`
  1. Build interactor, load+register sender
  2. Encode `args.arguments` via `BytesValue::interpret_from` → `encoded_args: Vec<Vec<u8>>`
  3. Read bytecode from `--bytecode` path
  4. Compute `CodeMetadata` from `MetadataArgs` flags
  5. Build tx:
     ```
     interactor.tx()
       .from(&sender_addr)
       .gas(args.tx.gas_limit)
       .raw_deploy()
       .code(bytecode)
       .code_metadata(metadata)
       .argument(arg_bytes) × N
       .returns(ReturnsNewBech32Address)
       .run().await
     ```
  6. **Verify**: `.code_metadata()` and `.argument()` are chainable on `raw_deploy()` — check `framework/base/src/types/interaction/tx.rs`
  7. If `--send`: broadcast (and await if `--wait-result`); else: serialize signed tx JSON to `--outfile`/stdout

---

## Phase 4B — Call Handler (`tx_call.rs`)

- File: `framework/meta/src/cmd/tx/tx_call.rs`
- `pub async fn tx_call(args: &CallArgs) -> TxOutput`
  1. Build interactor, load+register sender
  2. Encode arguments via `BytesValue::interpret_from`
  3. Parse `--token-transfers` flat list into ESDT payment(s)
  4. Build tx:
     ```
     interactor.tx()
       .from(&sender_addr)
       .to(&contract_addr)
       .gas(gas_limit)
       .raw_call(function_name)
       .argument(arg_bytes) × N
       .payment(esdt_payments)   // optional
       .returns(ReturnsHandledOrError::new())
       .run().await
     ```
  5. **Gap**: verify `.payment()` is chainable with `.raw_call()` in the untyped path; if not, encode MultiESDTTransfer manually
  6. If `--send`: broadcast; else: serialize signed tx JSON

---

## Phase 4C — Upgrade Handler (`tx_upgrade.rs`)

- File: `framework/meta/src/cmd/tx/tx_upgrade.rs`
- `pub async fn tx_upgrade(args: &UpgradeArgs) -> TxOutput`
  1. Build interactor, load+register sender
  2. Encode arguments via `BytesValue::interpret_from`
  3. Read bytecode, compute `CodeMetadata`
  4. Build tx using `.raw_upgrade()`:
     ```
     interactor.tx()
       .from(&sender_addr)
       .to(&contract_addr)
       .gas(gas_limit)
       .raw_upgrade()
       .code(bytecode)
       .code_metadata(metadata)
       .argument(arg_bytes) × N
       .returns(ReturnsHandledOrError::new())
       .run().await
     ```
  5. If `--send`: broadcast; else: serialize signed tx JSON

---

## Phase 4D — Query Handler (`tx_query.rs`)

- File: `framework/meta/src/cmd/tx/tx_query.rs`
- `pub async fn tx_query(args: &QueryArgs) -> QueryOutput`
  - No wallet needed; always runs immediately (no `--send` flag)
  - Encode arguments via `BytesValue::interpret_from`
  - Build query:
    ```
    interactor.query()
      .to(&contract_addr)
      .raw_call(function_name)
      .argument(arg_bytes) × N
      .returns(ReturnsResultUnmanaged)
      .run().await
    ```
  - V1: print raw hex bytes of each return value

---

## Phase 4E — New Handler (`tx_new.rs`)

- File: `framework/meta/src/cmd/tx/tx_new.rs`
- `pub async fn tx_new(args: &NewArgs) -> TxOutput`
  - Generic transaction: EGLD transfer or ESDT transfer to `--receiver`
  - `--data` / `--data-file`: raw payload string or file (mutually exclusive with `--token-transfers`)
  - `--token-transfers`: ESDT payments (same flat-list format as `call`)
  - Build and sign tx; if `--send`: broadcast; else: serialize signed tx JSON to `--outfile`/stdout

---

## Phase 4F — Send Handler (`tx_send.rs`)

- File: `framework/meta/src/cmd/tx/tx_send.rs`
- `pub async fn tx_send(args: &SendArgs) -> TxOutput`
  - Loads a previously saved signed tx JSON from `--infile`
  - Broadcasts via the interactor/proxy (`--gateway`)
  - Optionally awaits result if `--wait-result`
  - No wallet required

---

## Phase 4G — Sign Handler (`tx_sign.rs`)

- File: `framework/meta/src/cmd/tx/tx_sign.rs`
- `pub async fn tx_sign(args: &SignArgs) -> TxOutput`
  - Loads an unsigned tx JSON from `--infile`
  - Loads wallet via `SenderArgs`, signs the tx
  - If `--send`: broadcasts after signing; else: writes signed tx JSON to `--outfile`/stdout

---

## Phase 5 — Output Formatting (`output.rs`)

- File: `framework/meta/src/cmd/tx/output.rs`
- `TxOutput` struct: `{ tx: Option<SignedTxJson>, tx_hash: Option<String>, contract_address: Option<String> }`
  - `tx`: populated when `--send` is **not** set (offline/save mode)
  - `tx_hash` + `contract_address`: populated when `--send` is set
- `QueryOutput` struct: `{ return_values: Vec<String> }`
- Serialize to JSON via `serde_json`; write to `--outfile` or stdout
- Print tx hash / contract address to stderr for human visibility

---

## Phase 6 — Integration (`mod.rs`)

- File: `framework/meta/src/cmd/tx/mod.rs`
- `pub async fn tx_cli(args: &TxCliArgs)`
- Dispatches to all seven handlers

---

## Relevant Files

### Modify
- `framework/meta/src/cli/cli_args_standalone.rs` — add `Tx(TxCliArgs)` variant
- `framework/meta/src/cli/cli_standalone_main.rs` — add dispatch arm
- `framework/meta/src/cmd/mod.rs` — add `pub mod tx`

### Create
- `framework/meta/src/cmd/tx/mod.rs`
- `framework/meta/src/cmd/tx/tx_cli_args.rs`
- `framework/meta/src/cmd/tx/interactor_setup.rs`
- `framework/meta/src/cmd/tx/tx_deploy.rs`
- `framework/meta/src/cmd/tx/tx_call.rs`
- `framework/meta/src/cmd/tx/tx_upgrade.rs`
- `framework/meta/src/cmd/tx/tx_query.rs`
- `framework/meta/src/cmd/tx/tx_new.rs`
- `framework/meta/src/cmd/tx/tx_send.rs`
- `framework/meta/src/cmd/tx/tx_sign.rs`
- `framework/meta/src/cmd/tx/output.rs`

### Key Reference Files (read-only)
- `framework/meta/src/cmd/wallet.rs` — pattern for an existing async command
- `framework/meta/src/cmd/retrieve_address.rs` — pattern for gateway + async cmd
- `contracts/examples/adder/interactor/src/basic_interactor.rs` — interactor setup/usage pattern
- `framework/snippets/src/interactor/interactor_base.rs` — `Interactor::new()`, `register_wallet()`
- `framework/base/src/types/interaction/tx.rs` — `raw_call`, `raw_deploy`, `raw_upgrade`, `.argument()`
- `sdk/core/src/wallet.rs` — `Wallet::from_pem_file()`, `from_keystore_secret()`
- `framework/scenario/src/scenario/model/value/bytes_value.rs` — `BytesValue::interpret_from` (argument encoding)
- `sdk/scenario-format/src/interpret_trait.rs` — `InterpreterContext::with_dir()` (sets cwd for file-based expressions)

---

## Verification
1. `sc-meta tx deploy --bytecode output/adder.wasm --gateway https://devnet-gateway.multiversx.com --chain-id D --gas-limit 5000000 --pem ~/wallets/test.pem` → writes signed tx JSON to stdout
2. `sc-meta tx deploy --bytecode output/adder.wasm --gateway https://devnet-gateway.multiversx.com --chain-id D --gas-limit 5000000 --pem ~/wallets/test.pem --send --wait-result` → deploys and waits for result
3. `sc-meta tx call erd1... --function add --arguments 5 --gateway ... --chain-id D --gas-limit 3000000 --pem ~/wallets/test.pem --send`
4. `sc-meta tx query erd1... --function getSum --gateway https://devnet-gateway.multiversx.com` → prints hex result
5. `sc-meta tx upgrade erd1... --bytecode output/adder.wasm --gateway ... --chain-id D --gas-limit 5000000 --pem ~/wallets/test.pem --send`
6. `sc-meta tx new --receiver erd1... --value 1000000000000000000 --gateway ... --chain-id D --gas-limit 50000 --pem ~/wallets/test.pem` → writes signed tx JSON to stdout
7. `sc-meta tx new ... --outfile tx.json && sc-meta tx send --infile tx.json --gateway ...` → offline sign then broadcast
8. `sc-meta tx sign --infile tx.json --pem ~/wallets/test.pem --outfile signed.json`
9. Compile check: `cargo build -p multiversx-sc-meta`
10. End-to-end: `sc-meta cs start` then deploy/call/query cycle

---

## Further Considerations
1. **`.argument()` API shape on raw_deploy/raw_call**: Verify that `raw_deploy()` and `raw_call()` results expose a chainable `.argument(&[u8])` method — check `framework/base/src/types/interaction/tx.rs`. If absent, this must be added first.
2. **ESDT + raw_call combination**: Verify `.payment()` is chainable with `.raw_call()` in the untyped path; if not, encode MultiESDTTransfer manually in data field.
3. **Keystore password**: Confirm `Wallet::from_keystore_secret()` signature — prompt interactively via stdin or accept `--passfile`.
4. **Saved tx JSON format**: The format used by `tx_send` / `tx_sign` (loaded from `--infile`) must match what the interactor serializes in offline mode. Verify this is a stable, documented struct.
5. **relay (V2)**: `mxpy tx relay` adds a relayer co-signature to a saved tx. Deferred until relayer/guardian support is added to the interactor.
