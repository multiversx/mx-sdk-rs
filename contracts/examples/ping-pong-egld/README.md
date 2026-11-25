# Ping-Pong EGLD Contract

A MultiversX smart contract that accepts a fixed EGLD amount from users ("ping"), locks it for a configurable duration, then allows users to reclaim their funds ("pong") after the deadline passes.

Key design points:
- Each address can ping **exactly once**.
- Only the exact `ping_amount` set at deploy is accepted — no more, no less.
- An optional funding cap prevents the contract from accepting more than `max_funds` total.
- An optional `activation_timestamp` delays when pinging opens.
- `pongAll` sends funds back to all registered users in one call, with gas-interrupt-and-resume support for large user sets.

It is also available as an `sc-meta` template (see "Using as a template" below).

## Endpoints and views

| Name | Type | Description |
|---|---|---|
| `init(ping_amount, duration, opt_activation_timestamp, max_funds)` | `#[init]` | Configure the contract on deploy |
| `upgrade(...)` | `#[upgrade]` | Re-configure with the same parameters as `init` |
| `ping` | `#[payable("EGLD")] #[endpoint]` | Lock the exact `ping_amount`; callable once per address while the contract is active |
| `pong` | `#[endpoint]` | Reclaim your `ping_amount` after the deadline |
| `pongAll` | `#[endpoint]` | Send back funds to every registered user; returns `completed` or `interrupted` if gas runs low (resumable) |
| `getUserAddresses` | `#[view]` | List all addresses that pinged, in order |
| `getContractState` | `#[view]` | Snapshot of all configuration and progress fields |
| `getPingAmount` | `#[view]` | The required ping payment |
| `getDeadline` | `#[view]` | Timestamp after which pong is allowed |
| `getActivationTimestamp` | `#[view]` | Timestamp from which ping is allowed |
| `getMaxFunds` | `#[view]` | Optional total funding cap |
| `getUserStatus(user_id)` | `#[view]` | Per-user state: `New` / `Registered` / `Withdrawn` |
| `pongAllLastUser` | `#[view]` | Last user index processed by `pongAll` (0 = idle or complete) |

## Project structure

```
ping-pong-egld/
├── src/
│   ├── ping_pong.rs    # Contract logic
│   ├── proxy.rs        # Auto-generated proxy (sc-meta all proxy)
│   └── types.rs        # ContractState, UserStatus types
├── tests/
│   ├── ping_pong_egld_blackbox_from_scenarios.rs
│   ├── ping_pong_egld_scenario_go_test.rs
│   └── ping_pong_egld_scenario_rs_test.rs
├── scenarios/          # JSON scenario files
├── interactor/         # Interactor (live network) tool
├── snippets/           # Shell snippets
├── meta/               # Build metadata
├── wasm/               # Wasm build output
├── sc-config.toml      # Proxy output path
├── Cargo.toml
└── multiversx.json
```

## Building

```bash
sc-meta all build
```

## Regenerating the proxy

```bash
sc-meta all proxy
```

## Testing

```bash
cargo test
```

## Using as a template

```bash
sc-meta new --template ping-pong-egld --name my-contract
```
