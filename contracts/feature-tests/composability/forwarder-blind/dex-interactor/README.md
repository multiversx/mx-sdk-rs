# forwarder-blind DEX interactor

CLI interactor for the `forwarder-blind` contract that demonstrates calling into a DEX pair
using every available call type.

## Running

```bash
cargo run -- <COMMAND> [OPTIONS]
```

---

## Typical workflow

1. Fill in `config.toml` with the gateway, token IDs, contract addresses, and wallet PEM paths.
2. **Deploy** — creates one forwarder-blind contract per wallet and records the addresses in `deploy.toml`:
   ```bash
   cargo run -- deploy
   ```
3. **Copy addresses** — open `deploy.toml` and paste the `contract_addresses` list into `config.toml`.
4. **Wrap** — give each wallet WEGLD to spend:
   ```bash
   cargo run -- wrap -a <AMOUNT>
   ```
5. **Swap** — run any combination of swap commands to exercise the different call types.
6. **Drain** (optional) — recover tokens left in the contracts after `te` swaps:
   ```bash
   cargo run -- drain
   ```

---

## Commands

### `deploy`

Deploy one forwarder-blind contract instance per configured wallet.

```bash
cargo run -- deploy
```

Each wallet in `wallet_pem_paths` deploys its own contract instance. The resulting addresses
are written to `deploy.toml`.

> **After deploying**, copy the addresses from `deploy.toml` into the `contract_addresses`
> list in `config.toml` so that subsequent swap and drain commands can target them.

---

### `wrap`

Wrap EGLD into WEGLD via the WEGLD swap contract. Runs once per wallet in parallel.

```bash
cargo run -- wrap -a <AMOUNT>
```

| Flag | Description |
|------|-------------|
| `-a`, `--amount` | Amount of EGLD to wrap, in denomination (1 EGLD = 10^18) |

---

### `swap1` — Swap WEGLD → USDC

```bash
cargo run -- swap1 <METHOD> [OPTIONS]
```

#### Methods

| Method | Description |
|--------|-------------|
| `direct` | Each wallet swaps directly on the DEX pair |
| `sync` | Each wallet × each contract calls `blind_sync` (same-shard only) |
| `async1` | Each wallet × each contract calls `blind_async_v1` |
| `async2` | Each wallet × each contract calls `blind_async_v2` |
| `te` | Each wallet × each contract calls `blind_transf_exec` |

#### Options (all methods)

| Flag | Description | Default |
|------|-------------|---------|
| `-a`, `--wegld-amount` | Amount of WEGLD to sell | *(required)* |
| `-m`, `--usdc-amount-min` | Minimum USDC to accept (slippage guard) | `1` |

#### Examples

```bash
# Direct swap
cargo run -- swap1 direct -a 1000000000000000000

# Via blind_sync
cargo run -- swap1 sync -a 1000000000000000000 -m 1000

# Via blind_async_v1
cargo run -- swap1 async1 -a 1000000000000000000

# Via blind_async_v2
cargo run -- swap1 async2 -a 1000000000000000000

# Via blind_transf_exec
cargo run -- swap1 te -a 1000000000000000000
```

---

### `swap2` — Swap USDC → WEGLD

```bash
cargo run -- swap2 <METHOD> [OPTIONS]
```

#### Methods

| Method | Description |
|--------|-------------|
| `direct` | Each wallet swaps directly on the DEX pair |
| `sync` | Each wallet × each contract calls `blind_sync` (same-shard only) |
| `async1` | Each wallet × each contract calls `blind_async_v1` |
| `async2` | Each wallet × each contract calls `blind_async_v2` |
| `te` | Each wallet × each contract calls `blind_transf_exec` |

#### Options (all methods)

| Flag | Description | Default |
|------|-------------|---------|
| `-a`, `--usdc-amount` | Amount of USDC to sell | *(required)* |
| `-m`, `--wegld-amount-min` | Minimum WEGLD to accept (slippage guard) | `1` |

#### Examples

```bash
# Direct swap
cargo run -- swap2 direct -a 40000

# Via blind_sync
cargo run -- swap2 sync -a 40000 -m 1

# Via blind_async_v1
cargo run -- swap2 async1 -a 40000

# Via blind_async_v2
cargo run -- swap2 async2 -a 40000

# Via blind_transf_exec
cargo run -- swap2 te -a 40000
```

---

### `get-rate`

Get the approximate WEGLD → USDC conversion rate.

```bash
cargo run -- get-rate [-a <WEGLD_AMOUNT>]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-a`, `--wegld-amount` | Amount of WEGLD to price | `1000000000000000000` (1 EGLD) |

---

### `get-liquidity`

Show the liquidity reserves in the WEGLD/USDC pair.

```bash
cargo run -- get-liquidity
```

---

### `drain`

Drain all WEGLD and USDC balances held by the forwarder-blind contracts back to their owners.
Useful to recover tokens left in the contracts after transfer-execute swaps (which have no callback).

```bash
cargo run -- drain
```

For each contract address in `config.toml` (`contract_addresses`), the interactor looks up the
on-chain owner. If that owner is one of the registered wallets, it sends a drain transaction.
Contracts whose owner is not a registered wallet are skipped.

---

## Configuration

The interactor reads `config.toml` from the current directory:

```toml
chain_type = 'real'
gateway_uri = 'https://gateway.battleofnodes.com'
wegld_address  = 'erd1...'   # WEGLD swap contract
pair_address   = 'erd1...'   # WEGLD/USDC DEX pair contract
wegld_token_id = 'WEGLD-bd4d79'
usdc_token_id  = 'USDC-c76f1f'

wallet_pem_paths = [
    'path/to/wallet1.pem',
    'path/to/wallet2.pem',
]

contract_addresses = [
    'erd1...',
    'erd1...',
]
```

| Field | Description |
|-------|-------------|
| `chain_type` | `real` for mainnet/testnet, or `simulator` for the chain simulator |
| `gateway_uri` | Gateway endpoint URL |
| `wegld_address` | Address of the WEGLD swap contract |
| `pair_address` | Address of the WEGLD/USDC DEX pair contract |
| `wegld_token_id` | ESDT identifier for WEGLD |
| `usdc_token_id` | ESDT identifier for USDC |
| `wallet_pem_paths` | List of PEM file paths, one per wallet. Paths are relative to the workspace root. If absent or empty, all operations are skipped with a warning. |
| `contract_addresses` | List of forwarder-blind contract addresses to target for swap-via-forwarder commands. |

---

## Deploy output

`deploy.toml` is written automatically after every `deploy` run. Example contents:

```toml
# These are the last deployed addresses. Copy them to config.toml contract_addresses to use them.
contract_addresses = [
    "erd1...",
    "erd1...",
]
```

This file is **output-only** — no command reads it back. All commands (swap, drain, etc.) read
contract addresses exclusively from `contract_addresses` in `config.toml`.

> **After deploying**, copy the addresses from `deploy.toml` into `config.toml`
> (`contract_addresses`) before running any other command.

---

## Multiple wallets and contracts

The interactor is designed to exercise the same call paths from many wallets simultaneously,
each going through its own contract instance. The relationship between wallets and contracts
is many-to-many.

### `deploy`
One deploy transaction is submitted per wallet. The N resulting contract addresses are stored in
`deploy.toml` in the same order as the wallets in `wallet_pem_paths`. Copy those addresses into
`config.toml` (`contract_addresses`) before running any other command.

### `wrap`
One wrap transaction is submitted per wallet. All transactions are batched and sent in parallel.

### `swap1` / `swap2` — `direct`
One swap transaction is submitted per wallet, sent directly to the DEX pair.

### `swap1` / `swap2` — `sync`, `async1`, `async2`, `te`
One transaction is submitted for every **(wallet, contract)** pair — the full Cartesian product.
For example, 3 wallets × 3 contracts = 9 transactions per command, all sent in a single batch.

**Shard constraint for `sync`:** `blind_sync` uses a synchronous call, which requires the
forwarder contract and the DEX pair to be on the same shard. Any (wallet, contract) pair where
the contract's shard differs from the DEX pair's shard is skipped with a warning.

### `drain`
Reads contract addresses from `config.toml` (`contract_addresses`), exactly like swap commands.
For each contract it fetches the on-chain owner and checks whether that address corresponds to
one of the registered wallets. Only contracts whose owner is a registered wallet receive a drain
transaction, so it is safe to run even when `config.toml` contains contracts deployed by other
parties.
