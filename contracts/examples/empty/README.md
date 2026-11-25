# Empty Contract

A minimal MultiversX smart contract with no logic. Intended as a **starting template** when creating a new contract from scratch.

## Contract

The contract defines a single trait `EmptyContract` with:

- `#[init]` — empty initializer, called on deploy
- `#[upgrade]` — empty upgrade handler, called on contract upgrade

## Project structure

```
empty/
├── src/
│   └── empty.rs          # Contract source
├── tests/
│   ├── empty_scenario_go_test.rs   # Scenario tests (Go runner)
│   └── empty_scenario_rs_test.rs   # Scenario tests (Rust runner)
├── scenarios/
│   └── empty.scen.json   # Scenario definition
├── meta/                 # Build metadata
├── wasm/                 # Wasm build output
├── Cargo.toml
└── multiversx.json
```

## Building

```bash
sc-meta all build
```

## Testing

```bash
cargo test
```

## Using as a template

This contract is registered as an `sc-meta` template under the name `empty`. To scaffold a new project from it:

```bash
sc-meta new --template empty --name my-contract
```
