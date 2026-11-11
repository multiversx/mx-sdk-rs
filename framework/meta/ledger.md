# Ledger Hardware Wallet Integration Plan

## Overview

This document outlines the plan for implementing Ledger hardware wallet support, translated from the Python SDK (`mx-sdk-py` + `mx-sdk-py-cli`).

The goal is:
1. A `LedgerApp` struct in `sdk/core` (feature-gated) that handles raw APDU communication.
2. A `Signer` enum embedded in `Wallet`, replacing the `private_key` field, that holds either a `PrivateKey` or a Ledger signer.
3. A `--ledger` flag on `SenderArgs` (and a `ledger` field in `WalletConfig`) so any signing path can use the Ledger device.
4. A `sc-meta ledger` CLI subcommand to list addresses and query app version.

Guardians and relayers are out of scope.

---

## Python Reference

### `LedgerApp` (`multiversx_sdk/ledger/ledger_app.py`)

The core class. Wraps raw HID communication via the `ledgercomm` Python library.

**APDU constants** (`CLA = 0xED`):

| Instruction              | INS  | Description                         |
|--------------------------|------|-------------------------------------|
| `GET_APP_CONFIG`         | 0x02 | App version, data activated, indexes|
| `GET_ADDRESS`            | 0x03 | Get bech32 address for an index     |
| `SET_ADDRESS`            | 0x05 | Set active address index on device  |
| `SIGN_MESSAGE`           | 0x06 | Sign arbitrary message              |
| `SIGN_HASH_TX`           | 0x07 | Sign transaction (hash-based)       |
| `PROVIDE_ESDT_INFO`      | 0x08 | (Out of scope)                      |
| `GET_ADDRESS_AUTH_TOKEN` | 0x09 | (Out of scope)                      |

**Chunked signing protocol:**
- Maximum chunk size: 150 bytes
- First chunk: `p1 = 0x00`
- Subsequent chunks: `p1 = 0x80`
- `p2 = 0x00` always
- Response: 65 bytes — byte 0 is `0x40` (length prefix), bytes 1..=64 are the ed25519 signature

**Address derivation path:** `m/44'/508'/account'/0'/address_index'`
- `account` is always `0` for MultiversX
- `address_index` selects the address

**Error codes:**

| Code   | Meaning                        |
|--------|--------------------------------|
| 0x9000 | Success                        |
| 0x6985 | User denied                    |
| 0x6D00 | Unknown instruction            |
| 0x6E00 | Wrong CLA                      |
| 0x6E10 | Signature failed               |
| 0x6E01 | Invalid arguments              |
| 0x6E02 | Invalid message                |
| 0x6E03 | Invalid p1                     |
| 0x6E04 | Message too long               |
| 0x6E07 | Contract data disabled         |
| 0x6E09 | Wrong tx version               |
| 0x6E0F | Wrong tx options               |
| 0x6E11 | Regular signing is deprecated  |

### `LedgerAccount` (`multiversx_sdk/accounts/ledger_account.py`)

Wraps `LedgerApp` and exposes `sign_transaction` / `sign_message`.

- `sign_transaction` requires hash-signing options set: `tx.version >= 2` and `tx.options & 1 == 1`.
- Message signing prepends a 4-byte big-endian length prefix to the message data.

### CLI (`multiversx_sdk_cli/cli_ledger.py`)

Two subcommands:
- `ledger addresses [--num-addresses N]` — prints `account_index | address_index | address` for N addresses.
- `ledger version` — prints the app version string (`MAJOR.MINOR.PATCH`).

---

## Rust Crates

### Option A: `ledger-transport-hid` + `ledger-apdu` (Recommended)

- **`ledger-apdu`** (crates.io, Zondax): `APDUCommand` and `APDUAnswer` types with proper framing.
- **`ledger-transport-hid`** (crates.io, Zondax): HID transport over `hidapi`. Handles Ledger's 64-byte HID packet framing automatically (non-trivial to implement from scratch).

These are maintained by Zondax — the same company that maintains the official [MultiversX Ledger app](https://github.com/LedgerHQ/app-multiversx).

```toml
[dependencies]
ledger-transport-hid = "0.10"
ledger-apdu = "0.10"
```

### Option B: `hidapi` directly

Use the raw `hidapi` crate. Requires manually implementing the Ledger HID framing protocol (64-byte packets with channel/sequence headers). Not recommended — `ledger-transport-hid` does this correctly.

### Feature flag

The Ledger crates link against `libhidapi` (a C library; on Linux it also requires `libudev`). This must not be forced on all users of `multiversx-sdk` or `multiversx-sc-snippets`. Gate behind a `ledger` feature in each relevant crate:

| Crate                    | Feature   | Default |
|--------------------------|-----------|---------|
| `multiversx-sdk`         | `ledger`  | no      |
| `multiversx-sc-snippets` | `ledger`  | no      |
| `multiversx-sc-meta`     | `ledger`  | **yes** |

`sc-meta` enables it by default because it is a standalone developer tool — Ledger support is expected to be available out of the box when running `sc-meta tx sign --ledger` or `sc-meta ledger addresses`.

CI pipelines running `sc-meta` builds may need `libhidapi-dev` (and `libudev-dev` on Linux) installed.

---

## Testing Without a Physical Device

### Speculos (Official Ledger Emulator)

- **Repo:** https://github.com/LedgerHQ/speculos
- Emulates Ledger Nano S/X/S+ using QEMU + ARM cross-compilation.
- Exposes an APDU endpoint on TCP port 9999.
- The `ledger-transport-tcp` crate (Zondax) connects to it.
- **Requires:** the MultiversX Ledger app `.elf` binary (from https://github.com/LedgerHQ/app-multiversx), Python, QEMU.
- Suitable for integration tests in CI if the app binary is bundled.

### Mock Transport (Unit Tests)

Abstract the transport behind a trait so unit tests can inject mock responses:

```rust
pub trait LedgerTransport {
    fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer>;
}

pub struct LedgerApp<T: LedgerTransport> { transport: T }
```

This allows testing APDU encoding/decoding, chunking logic, error mapping, and response parsing without any device or emulator.

### Integration Test Strategy

| Scenario                      | Approach                                   |
|-------------------------------|--------------------------------------------|
| APDU encoding/chunking        | Unit tests with mock transport             |
| Response parsing / error codes| Unit tests with mock transport             |
| Full sign flow                | Integration test against Speculos (CI opt-in, guarded by feature/env var) |
| Physical device smoke test    | Manual test only                           |

---

## Implementation Plan

### 1. `Signer` enum and `Wallet` restructure (in `sdk/core`)

Replace the `private_key: PrivateKey` field in `Wallet` with a `signer: Signer` field.

**`Signer` enum** — always compiled, no `#[cfg]` on the type or its variants:

```rust
pub enum Signer {
    PrivateKey(PrivateKey),
    Ledger { address_index: u32 },
}
```

The `Ledger` variant holds only a `u32`, so it has no compile-time dependency on `libhidapi`. The enum is always available, which means `--ledger` flags in configs and CLIs work unconditionally. Runtime signing with `Signer::Ledger` will return an error if the `ledger` feature is absent (see §4).

**Updated `Wallet` struct:**

```rust
pub struct Wallet {
    pub signer: Signer,   // replaces private_key: PrivateKey
    pub address: Address,
    pub source: WalletSource,
}
```

`Wallet::new(private_key, source)` remains for the `PrivateKey` path (address is still derived from the key). A new `Wallet::new_ledger(address, address_index)` constructor is added (always available — it just stores the address and `Signer::Ledger { address_index }`).

**Impact on existing `Wallet` methods:**

| Method              | Change                                                                 |
|---------------------|------------------------------------------------------------------------|
| `private_key_hex()` | Returns `Option<String>` — `None` for Ledger                          |
| `public_key()`      | Returns `Option<PublicKey>` — `None` for Ledger                       |
| `public_key_hex()`  | Returns `Option<String>` — `None` for Ledger                          |
| `to_pem()`          | Returns `Result<WalletPem>` — `Err` for Ledger                        |
| `sign_tx()`         | Dispatches on `Signer`; see §4                                         |
| `sign_bytes()`      | Dispatches on `Signer`; see §4                                         |

### 2. `WalletSource::Ledger` (in `sdk/core`)

Add a new variant to `WalletSource` (always compiled, no `#[cfg]`):

```rust
pub enum WalletSource {
    Mnemonic,
    PrivateKey,
    PemFile(Bech32Hrp),
    TestWallet(&'static str),
    Keystore(Bech32Hrp),
    Ledger { address_index: u32, hrp: Bech32Hrp },   // NEW
}
```

`WalletSource` serves as display / bech32-HRP metadata, consistent with `PemFile` and `Keystore`. `Wallet::to_bech32()` reads `hrp` from the `Ledger` variant just like the other two. The `hrp` is populated from the bech32 address returned by the device when constructing the wallet.

### 3. `LedgerApp` (in `sdk/core`, feature-gated)

New module: `sdk/core/src/wallet/ledger/`

```
sdk/core/src/wallet/ledger/
    mod.rs           — re-exports; entire module gated with #[cfg(feature = "ledger")]
    ledger_app.rs    — LedgerApp struct, APDU communication
    ledger_error.rs  — LedgerError enum, error code mapping
    ledger_config.rs — LedgerAppConfiguration struct
```

**`LedgerApp` API** (only exists when `feature = "ledger"` is active):

```rust
pub struct LedgerApp { /* HID transport */ }

impl LedgerApp {
    pub fn new() -> Result<Self, LedgerError>;
    pub fn get_app_configuration(&self) -> Result<LedgerAppConfiguration, LedgerError>;
    pub fn get_version(&self) -> Result<String, LedgerError>;
    pub fn get_address(&self, address_index: u32) -> Result<String, LedgerError>;
    pub fn set_address(&self, address_index: u32) -> Result<(), LedgerError>;
    pub fn sign_transaction(&self, tx_bytes: &[u8]) -> Result<[u8; 64], LedgerError>;
    pub fn sign_message(&self, message_bytes: &[u8]) -> Result<[u8; 64], LedgerError>;
}
```

Internally `sign_transaction` and `sign_message` share a `do_sign(data, ins)` helper that:
1. Splits `data` into chunks of at most 150 bytes.
2. Sends APDUs: `p1=0x00` for first chunk, `p1=0x80` for subsequent.
3. Parses the 65-byte response: skips byte 0 (`0x40`), returns bytes 1..=64.

### 4. Signing dispatch in `Wallet::sign_tx` and `sign_bytes`

`Wallet::sign_tx` dispatches on `self.signer`:

```rust
pub fn sign_tx(&self, tx: &Transaction) -> Result<WalletSignature> {
    match &self.signer {
        Signer::PrivateKey(pk) => {
            // existing logic: serialize → optionally keccak256 → sign
        }
        Signer::Ledger { address_index } => {
            #[cfg(feature = "ledger")]
            {
                // enforce hash-signing options, open LedgerApp, sign
            }
            #[cfg(not(feature = "ledger"))]
            {
                anyhow::bail!(
                    "Ledger signing is not available; \
                     recompile with the `ledger` feature enabled"
                )
            }
        }
    }
}
```

The same pattern applies to `sign_bytes`.

**Transaction signing requirements:** The MultiversX Ledger app requires hash-based signing:
- `tx.version >= 2` and `tx.options & 1 == 1`
- The raw JSON bytes (not the hash) are sent to the device — the device hashes internally.
- If the options are not set, `sign_tx` returns an `Err` (matching Python behaviour).

### 5. `SenderArgs` extension (in `framework/meta`)

The new fields are **always** present — no `#[cfg]`:

```rust
pub struct SenderArgs {
    #[arg(long, group = "wallet_source")]
    pub pem: Option<PathBuf>,

    #[arg(long, group = "wallet_source")]
    pub keyfile: Option<PathBuf>,

    #[arg(long = "keystore-password")]
    pub keystore_password: Option<String>,

    /// Use Ledger hardware wallet for signing.
    #[arg(long, group = "wallet_source")]
    pub ledger: bool,

    /// Address index to use on the Ledger device (default: 0).
    #[arg(long = "ledger-address-index", default_value = "0")]
    pub ledger_address_index: u32,
}
```

`load_wallet` constructs a `Wallet` with `Signer::Ledger` when `sender.ledger` is true. The address is fetched from the device at that point (requires the `ledger` feature; returns `Err` otherwise). Signing then goes through `Wallet::sign_tx` as normal, which also checks the feature at the point of signing.

### 6. `WalletConfig` extension (in `framework/snippets`)

The new fields are **always** present — no `#[cfg]`:

```rust
pub struct WalletConfig {
    pub test_wallet: Option<String>,
    pub pem: Option<PathBuf>,
    pub keyfile: Option<PathBuf>,
    pub keystore_password: Option<String>,
    pub ledger: bool,                  // NEW
    pub ledger_address_index: u32,     // NEW (default 0)
    cache: OnceLock<Wallet>,
}
```

`WalletConfig::load_wallet` adds a branch for `ledger == true` that constructs a `Wallet::new_ledger(...)`. Attempting to use this branch without `feature = "ledger"` produces the same runtime error as §4.

### 7. `sc-meta ledger` CLI subcommand

New variant in `StandaloneCliAction`:

```rust
#[command(name = "ledger", about = "Interact with a Ledger hardware wallet.")]
Ledger(LedgerArgs),
```

New file: `framework/meta/src/cli/cli_args_ledger.rs`

```rust
#[derive(Args)]
pub struct LedgerArgs {
    #[command(subcommand)]
    pub command: LedgerAction,
}

#[derive(Subcommand)]
pub enum LedgerAction {
    #[command(about = "List addresses stored on the Ledger device.")]
    Addresses(LedgerAddressesArgs),

    #[command(about = "Print the version of the MultiversX Ledger app.")]
    Version,
}

#[derive(Args)]
pub struct LedgerAddressesArgs {
    /// Number of addresses to retrieve (default: 10).
    #[arg(long = "num-addresses", default_value = "10")]
    pub num_addresses: u32,
}
```

The `sc-meta ledger` subcommand handlers live in `framework/meta/src/cmd/ledger_cmd.rs` and are **always compiled** — no `#[cfg]` on the functions or the CLI types. Only the inner calls to `LedgerApp::new()` are guarded, returning a clear error when the `ledger` feature is absent (same pattern as `Wallet::sign_tx` in §4). Since `sc-meta` enables the `ledger` feature by default, the guard is never hit in normal usage.

Output format (matching Python):
```
account index = 0 | address index = 0 | address: erd1...
account index = 0 | address index = 1 | address: erd1...
```

---

## File Structure Summary

```
sdk/core/src/wallet/
    signer.rs                               — NEW: Signer enum (PrivateKey | Ledger)
    wallet_source.rs                        — add Ledger { address_index: u32 } variant
    wallet_impl.rs                          — replace private_key field; update sign_tx/sign_bytes
    ledger/
        mod.rs                              — NEW: #[cfg(feature = "ledger")]
        ledger_app.rs                       — NEW: LedgerApp, APDU protocol, chunked signing
        ledger_config.rs                    — NEW: LedgerAppConfiguration
        ledger_error.rs                     — NEW: LedgerError, error code mapping
sdk/core/Cargo.toml                         — add ledger-transport-hid, ledger-apdu as optional deps

framework/snippets/src/config/wallet_config.rs — add ledger / ledger_address_index fields
framework/snippets/Cargo.toml               — add ledger feature forwarding to multiversx-sdk

framework/meta/src/cli/cli_args_ledger.rs   — NEW: LedgerArgs, LedgerAction, LedgerAddressesArgs
framework/meta/src/cli/cli_args_sender.rs   — add --ledger / --ledger-address-index flags
framework/meta/src/cli/cli_args_standalone.rs — add Ledger subcommand
framework/meta/src/cmd/ledger_cmd.rs        — NEW: CLI command handlers
framework/meta/Cargo.toml                   — ledger feature default; forward to multiversx-sdk
```

---

## Cargo.toml Changes

**`sdk/core/Cargo.toml`:**

```toml
[features]
ledger = ["dep:ledger-transport-hid", "dep:ledger-apdu"]

[dependencies]
ledger-transport-hid = { version = "0.10", optional = true }
ledger-apdu          = { version = "0.10", optional = true }
```

**`framework/snippets/Cargo.toml`:**

```toml
[features]
ledger = ["multiversx-sdk/ledger"]
```

**`framework/meta/Cargo.toml`:**

```toml
[features]
default = ["ledger"]
ledger  = ["multiversx-sdk/ledger"]
```

> **Note on `libhidapi`:** `ledger-transport-hid` links against `hidapi` (a C library). On Linux it also requires `libudev`. CI pipelines building `sc-meta` (which enables `ledger` by default) may need `libhidapi-dev` / `libudev-dev` installed.


