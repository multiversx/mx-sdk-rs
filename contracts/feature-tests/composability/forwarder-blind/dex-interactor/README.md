# forwarder-blind DEX interactor

CLI interactor for the `forwarder-blind` contract that demonstrates calling into a DEX pair
using every available call type.

## Running

```bash
cargo run -- <COMMAND> [OPTIONS]
```

---

## Commands

### `deploy`

Deploy a new instance of the forwarder-blind contract.

```bash
cargo run -- deploy
```

---

### `wrap`

Wrap EGLD into WEGLD via the WEGLD swap contract.

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
| `direct` | Swap directly on the DEX pair |
| `sync` | Swap via forwarder-blind using `blind_sync` |
| `async1` | Swap via forwarder-blind using `blind_async_v1` |
| `async2` | Swap via forwarder-blind using `blind_async_v2` |
| `te` | Swap via forwarder-blind using `blind_transf_exec` |

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
| `direct` | Swap directly on the DEX pair |
| `sync` | Swap via forwarder-blind using `blind_sync` |
| `async1` | Swap via forwarder-blind using `blind_async_v1` |
| `async2` | Swap via forwarder-blind using `blind_async_v2` |
| `te` | Swap via forwarder-blind using `blind_transf_exec` |

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

Drain all WEGLD and USDC balances held by the deployed forwarder-blind contract back to the owner.
Useful to recover tokens left in the contract after transfer-execute swaps (which have no callback).

```bash
cargo run -- drain
```

---

## Configuration

The interactor reads `config.toml` from the current directory. Example:

```toml
chain_type = 'real'
gateway_uri = 'https://gateway.battleofnodes.com'
wegld_address  = 'erd1...'   # WEGLD swap contract
pair_address   = 'erd1...'   # WEGLD/USDC DEX pair contract
wegld_token_id = 'WEGLD-bd4d79'
usdc_token_id  = 'USDC-c76f1f'
```

| Field | Description |
|-------|-------------|
| `chain_type` | `real` for mainnet/testnet or `simulator` for the chain simulator |
| `gateway_uri` | Gateway endpoint URL |
| `wegld_address` | Address of the WEGLD swap contract |
| `pair_address` | Address of the WEGLD/USDC DEX pair contract |
| `wegld_token_id` | ESDT identifier for WEGLD |
| `usdc_token_id` | ESDT identifier for USDC |

The deployed contract address is persisted automatically in `state.toml` after a successful `deploy`.
