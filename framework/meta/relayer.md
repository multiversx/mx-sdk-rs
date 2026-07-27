# Relayed Transactions v3 — Implementation Plan

## Overview

Relayed transactions v3 add two optional fields to an existing transaction:
- `relayer` — bech32 address of the account that pays the gas fees
- `relayerSignature` — hex signature proving the relayer's agreement

No inner transaction wrapping is needed (unlike v1/v2). The sender signs first, then the relayer signs.

Gas: `gas_limit = base_cost + base_cost + cost_per_byte * len(data)`

Signing order:
1. Sender signs over the full transaction bytes (which already include the `relayer` field).
2. Relayer signs over the same transaction bytes (with `relayer` set but `relayerSignature` still empty/absent).

---

## Phase 1 — SDK: Extend the `Transaction` struct

**File:** `sdk/core/src/data/transaction/transaction_request.rs`

Add two new optional fields, skipped during serialization when absent:

```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub relayer: Option<Bech32Address>,

#[serde(skip_serializing_if = "Option::is_none")]
pub relayer_signature: Option<WalletSignature>,
```

The existing `Wallet::sign_tx` method already signs over the serialized transaction bytes via protobuf. The protobuf schema already supports `Relayer` and `RelayerSignature` fields (confirmed in `chain/vm`). No new signing method is needed — the same `sign_tx` is called by both sender and relayer (at different stages, with `relayer_signature` absent during both signing calls).

Verify that the protobuf serializer (`sdk/core/src/data/transaction/`) includes `relayer` in the bytes-to-sign and excludes `relayer_signature` from both signing payloads.

---

## Phase 2 — sc-meta CLI

### 2a. `RelayerArgs` struct

**File:** `framework/meta/src/cli/cli_args_sender.rs`  
*(mirrors the existing `SenderArgs`)*

```rust
/// Wallet / relayer arguments for commands that add a relayer signature.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct RelayerArgs {
    /// Path to a PEM wallet file for the relayer.
    #[arg(long = "relayer-pem", group = "relayer_source")]
    pub pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file for the relayer.
    #[arg(long = "relayer-keyfile", group = "relayer_source")]
    pub keyfile: Option<PathBuf>,

    /// Relayer keystore password (plain text). If omitted, prompted interactively.
    #[arg(long = "relayer-keystore-password")]
    pub keystore_password: Option<String>,
}
```

Add a companion `load_relayer_wallet(args: &RelayerArgs) -> Result<Option<Wallet>>` function alongside the existing `load_wallet`. Returns `None` when no relayer args are set (relayer wallet is optional in most commands).

### 2b. `--relayer` address field

**File:** `framework/meta/src/cli/cli_args_tx.rs`

Add to `TxArgs`:
```rust
/// Bech32 address of the relayer. If set, the transaction gas limit must
/// include the extra base cost for the relayed operation.
#[arg(long)]
pub relayer: Option<String>,
```

This field is used by `tx new`, `tx deploy`, `tx call`, `tx upgrade` to set `tx.relayer` before the sender signs.

### 2c. Extend existing commands to accept a relayer wallet

**Files:** `cli_args_tx.rs`, `tx_cli_new.rs`, `tx_cli_deploy.rs`, `tx_cli_call.rs`, `tx_cli_upgrade.rs`

For each of `DeployArgs`, `CallArgs`, `UpgradeArgs`, `NewArgs`:

```rust
#[command(flatten)]
pub relayer: RelayerArgs,
```

In their respective `tx_cli_*.rs` implementations:
1. If `args.tx.relayer` is set, write it to `tx.relayer` before the sender signs.
2. After the sender signs, call `load_relayer_wallet(&args.relayer)?`.
3. If a relayer wallet is present, validate its address matches `tx.relayer`, then call `wallet.sign_tx(&tx)` and write the result to `tx.relayer_signature`.

**File:** `framework/meta/src/cli/cli_args_tx.rs` — extend `SignArgs`:

```rust
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SignArgs {
    #[arg(long)]
    pub infile: PathBuf,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,        // existing

    #[command(flatten)]
    pub relayer: RelayerArgs,      // NEW

    // ... existing send/wait_result/outfile
}
```

In `tx_cli_sign.rs`:
- After the sender signs (existing behaviour), optionally load the relayer wallet and add `relayer_signature`.

### 2d. New `tx relay` subcommand

This is a dedicated command for the relayer party: they receive a pre-built, sender-signed transaction file and add their signature.

**`cli_args_tx.rs`** — add variant to `TxCliAction`:

```rust
#[command(about = "Adds the relayer signature to a previously signed transaction.")]
Relay(RelayArgs),
```

**New `RelayArgs` struct:**

```rust
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct RelayArgs {
    /// Path to the input transaction file (must already have sender signature and relayer field set).
    #[arg(long)]
    pub infile: PathBuf,

    #[command(flatten)]
    pub relayer: RelayerArgs,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    /// If set, the transaction is broadcast after signing.
    #[arg(long, default_value = "false")]
    pub send: bool,

    /// Wait for the transaction result. Requires --send.
    #[arg(long, default_value = "false", requires = "send")]
    pub wait_result: bool,

    /// Path to write the output to. Defaults to stdout.
    #[arg(long)]
    pub outfile: Option<PathBuf>,
}
```

**New file:** `framework/meta/src/cmd/tx/tx_cli_relay.rs`

Logic (`tx_relay_inner`):
1. Load `RelayerArgs` wallet — error if no wallet provided.
2. Load transaction from `--infile`.
3. Validate `tx.relayer` is set.
4. Validate relayer wallet address == `tx.relayer`.
5. Validate `tx.signature` is present (sender must have already signed).
6. Call `relayer_wallet.sign_tx(&tx)` and set `tx.relayer_signature`.
7. Save / broadcast via existing `save_output` / `broadcast_and_save` helpers.

**`tx.rs`** — wire up:

```rust
TxCliAction::Relay(relay_args) => tx_relay(relay_args).await,
```

---

## Phase 3 — Tx environment trait for relayer

**File:** `framework/base/src/types/interaction/tx_env.rs`

Add a new trait analogous to `TxEnvWithTxHash`, to carry the relayer address through the tx building pipeline:

```rust
pub trait TxEnvWithRelayer: TxEnv {
    fn set_relayer_address(&mut self, relayer: ManagedAddress<Self::Api>);

    /// Retrieves the relayer address, while resetting it in self.
    fn take_relayer_address(&mut self) -> Option<ManagedAddress<Self::Api>>;
}
```

This trait is implemented wherever `TxEnvWithTxHash` is already implemented:

- **`ScenarioTxEnvData`** (`framework/scenario/src/facade/world_tx/scenario_tx_env.rs`)  
  Add `relayer: Option<ManagedAddress<StaticApi>>` field; implement the two methods.

- **`ScenarioEnvExec`** (`framework/scenario/src/facade/world_tx/scenario_exec_call.rs`)  
  Forward to `self.data`.

- **`InteractorEnvExec`** (`framework/snippets/src/interactor/interactor_tx/interactor_exec_env.rs`)  
  Forward to `self.data`.

**`Tx` builder method** (`framework/base/src/types/interaction/tx.rs`)

Mirroring `.id()` and `.tx_hash()` (which require `Env: TxEnvWithTxHash`), add a `.relayer()` method that requires `Env: TxEnvWithRelayer`:

```rust
impl<Env, From, To, Payment, Gas, Data, RH> Tx<Env, From, To, Payment, Gas, Data, RH>
where
    Env: TxEnvWithRelayer,
    // ... other bounds
{
    /// Sets the relayer address for a relayed v3 transaction.
    ///
    /// The relayer pays the gas fees. After the sender signs the transaction,
    /// the relayer must also sign it (adding `relayerSignature`) before broadcasting.
    pub fn relayer<A>(mut self, relayer: A) -> Self
    where
        A: AnnotatedValue<Env, ManagedAddress<Env::Api>>,
    {
        let relayer_addr = relayer.into_value(&self.env);
        self.env.set_relayer_address(relayer_addr);
        self
    }
}
```

The relayer address stored in the env is read by the interactor execution pipeline (just before signing) to populate `tx.relayer` on the SDK `Transaction`.

---

## Phase 4 — Interactor Framework

### 4a. `Transaction` struct already extended (Phase 1)

No further struct changes needed here.

### 4b. Relayer registration — reuse `sender_map`

No new map or registration method is needed. The relayer is registered via the same `register_wallet` / `register_wallets` path as any other sender. The `InteractorBase` simply stores the chosen relayer address:

```rust
pub relayer_address: Option<Address>,   // added to InteractorBase
```

A dedicated setter:
```rust
pub fn set_relayer(&mut self, address: Address) {
    // address must already be registered in sender_map
    assert!(self.sender_map.contains_key(&address), "relayer wallet not registered");
    self.relayer_address = Some(address);
}
```

Users register the wallet normally and then call `set_relayer`:
```rust
let relayer_addr = interactor.register_wallet(relayer_wallet).await;
interactor.set_relayer(relayer_addr);
```

### 4c. `InteractorConfig` trait — no change

**File:** `framework/snippets/src/config/interactor_config.rs`

The `InteractorConfig` trait is **not** extended. There is no `relayer_wallet()` method. Contract-specific `Config` structs include a relayer wallet as a regular `WalletConfig` field (e.g. `pub relayer: WalletConfig`) and return it inside `register_wallets()`. The interactor setup code then calls `set_relayer` explicitly after loading the config.

Example in contract interactor setup code:
```rust
let relayer_addr = interactor.register_wallet(config.relayer.wallet().clone()).await;
interactor.set_relayer(relayer_addr);
```

### 4d. Transaction dispatch

**File:** `framework/snippets/src/interactor/interactor_sender.rs`

After the sender signs, if `relayer_address` is set on the interactor:
1. `tx.relayer` is populated from the env (via `take_relayer_address`, or from `interactor.relayer_address`).
2. `sign_tx_as_relayer` signs using the relayer's entry in `sender_map`:

```rust
pub(crate) fn sign_tx_as_relayer(&self, tx: &mut Transaction) {
    let Some(relayer_address) = &self.relayer_address else { return };
    let relayer = self.sender_map
        .get(relayer_address)
        .expect("relayer wallet not registered");
    let sig = relayer.wallet.sign_tx(tx);
    tx.relayer_signature = Some(sig);
}
```

---

## Summary of new files

| File | Purpose |
|---|---|
| `framework/meta/src/cmd/tx/tx_cli_relay.rs` | Implementation of `tx relay` command |

## Summary of modified files

| File | Change |
|---|---|
| `sdk/core/src/data/transaction/transaction_request.rs` | Add `relayer`, `relayer_signature` fields |
| `framework/base/src/types/interaction/tx_env.rs` | Add `TxEnvWithRelayer` trait |
| `framework/base/src/types/interaction/tx.rs` | Add `.relayer()` method on `Tx` (requires `Env: TxEnvWithRelayer`) |
| `framework/scenario/src/facade/world_tx/scenario_tx_env.rs` | Add `relayer` field to `ScenarioTxEnvData`; implement `TxEnvWithRelayer` |
| `framework/scenario/src/facade/world_tx/scenario_exec_call.rs` | Implement `TxEnvWithRelayer` for `ScenarioEnvExec` |
| `framework/snippets/src/interactor/interactor_tx/interactor_exec_env.rs` | Implement `TxEnvWithRelayer` for `InteractorEnvExec` |
| `framework/meta/src/cli/cli_args_sender.rs` | Add `RelayerArgs`, `load_relayer_wallet` |
| `framework/meta/src/cli/cli_args_tx.rs` | Add `--relayer` to `TxArgs`; add `RelayerArgs` flatten to `DeployArgs`/`CallArgs`/`UpgradeArgs`/`NewArgs`/`SignArgs`; add `RelayArgs` and `TxCliAction::Relay` |
| `framework/meta/src/cmd/tx/tx.rs` | Wire `TxCliAction::Relay` |
| `framework/meta/src/cmd/tx/tx_cli_new.rs` | Set `tx.relayer`, sign as relayer if wallet provided |
| `framework/meta/src/cmd/tx/tx_cli_deploy.rs` | Same |
| `framework/meta/src/cmd/tx/tx_cli_call.rs` | Same |
| `framework/meta/src/cmd/tx/tx_cli_upgrade.rs` | Same |
| `framework/meta/src/cmd/tx/tx_cli_sign.rs` | Optionally sign as relayer if `RelayerArgs` provided |
| `framework/snippets/src/interactor/interactor_base.rs` | Add `relayer_address: Option<Address>` field |
| `framework/snippets/src/interactor/interactor_sender.rs` | Add `set_relayer`, `sign_tx_as_relayer` |

---

## Key design decisions

1. **`RelayerArgs` mirrors `SenderArgs`** — same fields, same loading logic, different prefix (`--relayer-pem` etc.). This follows the Python CLI pattern exactly.

2. **Reuse `sender_map` for relayers** — no new map, no new registration path. A relayer is just another registered wallet identified by its address.

3. **No `relayer_wallet()` in `InteractorConfig`** — the trait is not extended. Relayer wallets are returned by `register_wallets()` alongside senders. The interactor setup code calls `set_relayer(address)` explicitly.

4. **`TxEnvWithRelayer` as a first-class trait** — mirrors the `TxEnvWithTxHash` pattern. The relayer address is carried in the environment, not as a separate parameter, so the tx builder `.relayer()` method integrates cleanly into the existing `Tx` chain.

5. **`--relayer` address vs `--relayer-pem`** — the address flag lets the sender declare the intended relayer without providing the key (two-step workflow). The wallet flags are only needed when both signatures happen in one command or via `tx relay`.

6. **Two-step vs one-step CLI workflow:**
   - *One-step:* `sc-meta tx new --relayer erd1... --relayer-pem relayer.pem ...` — both signatures in one go.
   - *Two-step:* `sc-meta tx new --relayer erd1... ...` (sender signs, saves to file) → `sc-meta tx relay --infile tx.json --relayer-pem relayer.pem` (relayer signs, broadcasts).

7. **Gas limit** — the caller is responsible for providing the correct gas limit including the extra base cost. No automatic adjustment is added at this stage; documentation/comments will explain the formula.

8. **Broadcast validation** — `broadcast_and_save` should additionally check that if `tx.relayer` is set, `tx.relayer_signature` is also present before broadcasting.
