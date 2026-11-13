# Interactor `dispatch`: error handling and dispatch settings

## Goal

Give interactors a single terminal method, `dispatch()`, that:

- follows the dispatch settings configured on the interactor (sign only and save, send without waiting, or send and wait);
- returns every error as a value: gateway and network failures, failed transactions, results that don't match expectations, and transactions that were saved or sent but not executed;
- keeps the current builder syntax unchanged.

`run()` stays, redefined as `dispatch()` followed by a panic on error.

```rust
interactor.dispatch_settings = args.dispatch_settings();

let token_id = interactor
    .tx()
    .from(&wallet)
    .to(ESDTSystemSCAddress)
    .gas(60_000_000)
    .typed(ESDTSystemSCProxy)
    .issue_fungible(...)
    .returns(ReturnsNewTokenIdentifier)
    .dispatch()
    .await?;   // Err(DispatchError::Interrupted(..)) when the tx was only saved or sent
```

## Current state

### Two separate code paths

- `run()` (`InteractorRunAsync`) goes through `sc_call` / `sc_deploy` / `transfer` in `interactor_scenario/`: sign, optionally simulate gas, broadcast, wait, `parse_tx_response`, then the result handlers.
- `run_with_settings(&TxDispatchSettings)` (`InteractorIntoSdkTransaction`) goes through `set_nonce_sign_and_dispatch` / `broadcast_and_save` in `interactor_dispatch.rs`. It returns `anyhow::Result<()>` and discards the result handlers, even when it waited for the result.

### The default expectation is not enforced on the network

- `tx_to_step` folds the handlers' `item_preprocessing` into `step.expect`, starting from `TxExpect::ok()` (status 0).
  - `ExpectError` overrides the expected status and message.
  - `ReturnsHandledOrError` sets both to `*`.
- The in-memory VM enforces it: `run_vm/sc_call.rs` and `sc_deploy.rs` call `check_tx_output`.
- The interactor never checks `step.expect`. `save_response` only uses it to build trace expectations.

What happens when a transaction fails on chain:

| Handlers | Today |
|---|---|
| none | No error: `run()` returns normally |
| `ReturnsResult` / `ReturnsResultAs` / `ReturnsResultUnmanaged` | Panic: "ReturnsResult expects that transaction is successful" |
| `ReturnsNewTokenIdentifier`, `ReturnsNewAddress`, … | Panic with a misleading message ("missing returned token identifier") |
| `ExpectError(code, msg)` | Nothing is asserted, so a failure with another code, or a success, goes unnoticed |
| `ReturnsHandledOrError` | Correct: `Err(TxResponseStatus)` |

### Process exits and panics on the network path

- Broadcast failures and failed simulate-gas estimates call `process::exit(1)` (`interactor_sc_call.rs`, `interactor_sc_deploy.rs`, `interactor_transfer.rs`, `interactor_vm_query.rs`).
- Fetching the transaction and generating blocks use `.unwrap()` / `.expect()`.

## Design

### 1. Dispatch settings live on the interactor

Today `TxDispatchSettings { send, wait_result, outfile }` is passed to `run_with_settings` on every call. Instead it becomes a field of `InteractorBase`, reshaped so that `wait_result` without `send` cannot be represented:

```rust
pub struct TxDispatchSettings {
    /// Where to write the signed tx (and, if awaited, `transactionOnNetwork`).
    pub output: TxOutputTarget,
    /// `None`: do not broadcast.
    pub send: Option<TxSendSettings>,
}

/// Destination of the `TxOutputFile` JSON (the mxpy-compatible content type).
#[derive(Default)]
pub enum TxOutputTarget {
    /// Not written anywhere.
    #[default]
    NotWritten,
    Stdout,
    File(PathBuf),
}

pub struct TxSendSettings {
    pub wait_result: bool,
}
```

- Stdout is an explicit target, not something implied by "not sent and no outfile". Sending and printing to stdout can therefore be combined too.
- `output: TxOutputTarget::NotWritten` together with `send: None` signs the tx and discards it. That combination is representable but useless, and the CLI never produces it.
- The default is `send: Some(TxSendSettings { wait_result: true })` with `output: TxOutputTarget::NotWritten`, which is today's `run()` behaviour.
- The CLI sets it once from its arguments: `interactor.dispatch_settings = args.dispatch_settings()`.
- Call sites don't mention it. The interactor acts as the "effect handler" for dispatch.
- Queries ignore it: they have no side effects and always run. That keeps "resolve the payment with a query, then build the tx" working even in save-only mode.

### 2. Two kinds of failure, kept apart

- **On-chain outcomes.** The transaction executed and failed. Result handlers control this:
  - With no explicit handling, a failure becomes an error (the default `TxExpect::ok()`).
  - `ReturnsHandledOrError` turns it into a value, `Ok(Err(status))`.
  - `ExpectError` states which failure is expected, so the failure is the success case.
- **Execution errors.** These are problems in running the dispatch itself: the gateway, signing, I/O, simulation, or a transaction not executed (yet).

Both reach the caller through the single `DispatchError` type. On-chain outcomes only get there when no handler took responsibility for them.

### 3. `DispatchError`

```rust
pub enum DispatchError {
    /// Tx was signed (and saved) or sent, but not executed / not awaited.
    /// Not a failure: the requested work was done, but no result can be produced.
    Interrupted(TxInterrupted),

    /// Tx executed and failed, while the default expectation (success) was in place.
    TxFailed(TxResponseStatus),

    /// A result handler rejected the response:
    /// - an explicit expectation did not match (e.g. `ExpectError` but the tx succeeded,
    ///   or failed with a different code/message);
    /// - later: a handler could not produce its value (missing token identifier,
    ///   decode error), see step 6.
    ResultHandler(ResultHandlerError),

    /// Simulate-gas could not estimate (failed or out of gas).
    Simulate(GasResult),

    /// Broadcasting, fetching the tx, generating blocks, other gateway failures.
    Gateway(anyhow::Error),

    /// Writing the output file.
    Io(std::io::Error),
}

/// What happened to a tx that was not executed / not awaited.
/// Saving and sending are orthogonal: a tx can be saved, sent, or both.
pub struct TxInterrupted {
    /// Where the signed tx was written (a copy of the setting).
    pub output: TxOutputTarget,
    /// The hash, if the tx was broadcast (its result was not awaited).
    pub tx_hash: Option<String>,
}
```

Mapping from `TxDispatchSettings`:

| `send` | `output` | Outcome |
|---|---|---|
| `None` | `t` | `Interrupted { output: t, tx_hash: None }` |
| `Some { wait_result: false }` | `t` | `Interrupted { output: t, tx_hash: Some(h) }` |
| `Some { wait_result: true }` | `t` | Executed: the handlers run. The output, if any, also gets `transactionOnNetwork` |

- `DispatchError` implements `std::error::Error`, so `?` works in `anyhow` code.
- The CLI's top-level handler treats `Interrupted` as success: it prints the status (saved to `…`, or sent with its hash) and exits 0. Every other variant exits 1.
- Names to settle: `TxInterrupted` vs `TxDeferred`; `ResultHandler` vs `ExpectationMismatch` (for now it only covers expectations).

### 4. Terminal methods

| Method | Returns | Notes |
|---|---|---|
| `dispatch()` | `Result<T, DispatchError>` | New main entry point for txs (call, deploy, upgrade, transfer) |
| `run()` | `T` | `dispatch()` + panic with the error's `Display`; later deprecated |
| `simulate_gas()` | `GasResult` | Unchanged |
| query `run()` | `T` | Unchanged for now; queries get their own fallible method later (no `Interrupted`, different error set) |

`run_with_settings` and `InteractorIntoSdkTransaction::run_with_settings` are removed in favour of `dispatch()` plus the interactor's settings. `into_sdk_transaction` stays.

### 5. One pipeline

`sc_call`, `sc_deploy`, `transfer` and `set_nonce_sign_and_dispatch` / `broadcast_and_save` merge into one pipeline. Each stage either continues or returns an error:

1. Build the SDK transaction from the step.
2. Set the nonce (`set_tx_nonce_update_sender`) and sign.
3. Simulate gas if requested (`Simulate`), then re-sign.
4. If `send` is `None`: write to `output` (`Io`), then stop with `Interrupted { output, tx_hash: None }`.
5. Otherwise, write a `File` target now (`Io`), so the signed tx is kept even if broadcasting fails. A `Stdout` target is written once, in step 7 or 9, so stdout holds a single JSON document.
6. Broadcast (`Gateway`).
7. If `wait_result` is false: write a `Stdout` target, with the hash. Then stop with `Interrupted { output, tx_hash: Some(hash) }`.
8. Generate blocks if on the chain simulator, then fetch and parse into a `TxResponse` (`Gateway`).
9. Write `transactionOnNetwork` to `output`: update the file, or print to stdout.
10. Pre/post runners (traces), and the new-token-identifier bookkeeping.
11. **Check `step.expect` against the response** (new). This uses the same comparison as `check_tx_output` in the VM, but returns an error instead of panicking:
    - Default expectation and the tx failed → `TxFailed(status)`.
    - Explicit expectation (`ExpectError`, `ExpectStatus`, `ExpectMessage`) not met → `ResultHandler(..)`.
    - `*` (`ReturnsHandledOrError`) → always passes.
12. Run the result handlers: `list_process_result`, then `flatten_unpack`.

Because of step 11, `ReturnsResult` and the other value handlers only ever see responses that passed the check. Their `.expect(...)` panics stop being reachable in normal use.

Nonce note: steps 2 and 5–7 advance the cached sender nonce even when nothing is executed. That's correct for "sign several txs offline" and for "fire without waiting".

### 6. Later: fallible result handlers

Handlers can still panic when the data they extract is missing or can't be decoded, e.g. `ReturnsNewTokenIdentifier` on a tx that issued nothing, or `ReturnsResult` with a wrong type. Making that a `ResultHandler` error needs a fallible variant of `RHListItemExec::item_process_result` in `multiversx-sc` (base). That is a wider change to a public trait, so it is kept out of the first iteration.

## Behaviour changes (release notes)

- `run()` now panics when a transaction fails on chain and no handler takes care of the failure. Previously it carried on silently when there were no handlers.
- `ExpectError` / `ExpectStatus` / `ExpectMessage` are now enforced on real networks, not just in the in-memory VM.
- Gateway failures panic with a message (via `run()`) or are returned (via `dispatch()`) instead of calling `process::exit(1)`.
- `run_with_settings` is replaced by `dispatch()` plus `interactor.dispatch_settings`.

## Plan

1. **Error types.** ✅ Add `DispatchError`, `TxInterrupted` and `ResultHandlerError` in `framework/snippets` (`interactor_dispatch_error.rs`), plus `TxOutputTarget` (`interactor_dispatch.rs`), which `TxInterrupted` needs.
2. **Settings on the interactor.** (reshape and `Default` ✅, interactor field pending)
   - Reshape `TxDispatchSettings` so that `send: Option<TxSendSettings>` carries `wait_result`.
   - Rename `outfile` to `output: TxOutputTarget`.
   - Update `dispatch_settings()` in `cli_args_tx.rs`. clap already rejects `--wait-result` without `--send` (`requires = "send"`), so the mappings are direct:
     - `send` = `send.then_some(TxSendSettings { wait_result })`;
     - `output` = `TxOutputTarget::file_or_stdout(outfile)`: `File(path)` if `--outfile` is given, `Stdout` otherwise, even when sending. That is today's CLI behaviour, and mxpy's.
   - The CLI flag stays `--outfile`, matching mxpy.
   - Add a `dispatch_settings` field to `InteractorBase`, defaulting to send and wait.
3. **Expectation check.** Add a non-panicking counterpart of `check_tx_output` that works on `TxExpect` and `TxResponse`, so the VM and the interactor share the comparison logic.
4. **Unified pipeline for SC calls.** Rewrite `sc_call` as a fallible pipeline, add `dispatch()` for calls, and redefine `run()` on top of it.
   - Testbed: `system_sc_interact.rs` and the chain-simulator tests.
5. **Deploy, upgrade, transfer.** Do the same, removing `run_with_settings`.
6. **Remove `process::exit` / `unwrap` from the network path**, including `retrieve_tx_on_network` and `generate_blocks_until_tx_processed`.
7. **`sc-meta tx` CLI.** Switch it to `interactor.dispatch_settings` and `dispatch()`. Have the top-level error handler treat `Interrupted` as success.
8. **Tests.** On the chain simulator:
   - a failed tx with no handlers;
   - `ExpectError` against both a success and a different error;
   - `ReturnsHandledOrError`;
   - save-only, send-only, and send-and-save interrupts, including the nonce advancing correctly across several saved txs.
9. **Migrate the interactors** in `contracts/` and `tools/` to `dispatch()` where they benefit from it. `run()` keeps working for the rest.
10. **Later:**
    - fallible result handlers (design step 6);
    - a fallible query method;
    - deprecating `run()`.
