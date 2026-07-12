# Greeter Contract

A minimal MultiversX smart contract that stores and retrieves a per-caller greeting message. Built on top of the `empty` template by adding one endpoint, one view, and one storage mapper — the smallest meaningful step beyond a bare scaffold.

It is also available as an `sc-meta` template (see "Using as a template" below).

## Contract

The `Greeter` trait exposes:

| Member | Type | Description |
|---|---|---|
| `init` | `#[init]` | Empty initializer, called on deploy |
| `upgrade` | `#[upgrade]` | Empty upgrade handler |
| `setGreeting(message)` | `#[endpoint]` | Stores a greeting for the calling address |
| `getGreeting(address)` | `#[view]` | Returns the greeting stored for `address`, or an empty buffer if none |

Storage is keyed per address via a parameterized `SingleValueMapper<ManagedBuffer>`, so each caller has an independent greeting.

## Project structure

```
greeter/
├── src/
│   ├── greeter.rs          # Contract source
│   └── greeter_proxy.rs    # Auto-generated proxy (sc-meta all proxy)
├── tests/
│   └── greeter_blackbox_test.rs
├── meta/                   # Build metadata
├── wasm/                   # Wasm build output
├── sc-config.toml          # Proxy output path
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
sc-meta new --template greeter --name my-contract
```
