# sc-meta tx — Missing features vs mxpy

## `--timeout` for `--wait-result`

mxpy accepts `--timeout N` (default 100 s) when `--wait-result` is set and uses
`AwaitingOptions { timeout_in_milliseconds }` to bound the polling loop.
`retrieve_tx_on_network` has internal exponential backoff capped at `MAX_RETRIES = 8`
for *network errors*, but the pending-transaction branch (`_ => { continue; }`) loops
forever with no deadline — so `--wait-result` can hang indefinitely on a stuck tx.

**Fix:** add `--timeout <seconds>` to `TxArgs` and thread it through
`broadcast_and_save` → `fetch_tx_on_network`, wrapping the call with
`tokio::time::timeout`.

---

## Explorer URL after broadcast

mxpy calls `log_explorer_transaction(chain, hash, explorer_url)` after a
successful send-only broadcast.  We only print the raw hash.

**Fix:** after printing `Transaction hash: {tx_hash}` in `broadcast_and_save`,
derive the explorer URL from the chain ID and print a clickable link.

---

## `--simulate` (dry-run)

mxpy supports `--simulate` as an alternative to `--send`, which posts the
transaction to the proxy's simulation endpoint and returns cost/execution info
without committing state.

**Fix:** add `--simulate` to `TxArgs`, implement a `simulate_and_save` helper
analogous to `broadcast_and_save`.

---

## Gas estimation

mxpy can omit `--gas-limit` and estimate gas automatically via
`SmartContractController` (backed by `GasLimitEstimator` and an optional
`--gas-limit-multiplier`).  We always require an explicit `--gas-limit`.

**Fix:** make `gas_limit` in `TxArgs` an `Option<u64>`; when absent, call the
proxy's cost-estimation endpoint and optionally multiply by a configurable factor.

---

## Re-sign if version/options are altered

mxpy's `alter_transaction_and_sign_again_if_needed` re-signs the transaction
when `--version` or `--options` differ from the already-signed values.
We don't support post-build alteration of these fields.

**Fix:** in `sign_and_dispatch`, apply version/options overrides before signing
(or re-sign if altered after the fact).
