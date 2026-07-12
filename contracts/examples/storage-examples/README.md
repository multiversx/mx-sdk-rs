# Storage Examples Contract

A working MultiversX smart contract demonstrating the six most commonly used storage mapper types side by side. Each mapper gets one endpoint group so the companion blackbox tests can verify the behavioral guarantees in the decision table below.

It is also available as an `sc-meta` template (see "Using as a template" below).

## Storage mapper decision table

| Mapper | Ordering | `contains` | Storage entries (N items) | Iterable | Pick it when |
|---|---|---|---|---|---|
| `SingleValueMapper<T>` | n/a | n/a | 1 | n/a | Exactly one value: a counter, a config flag, a total. |
| `VecMapper<T>` | Insertion order | O(n) | N + 1 | Yes (1-based) | Ordered, indexable, append-friendly; no fast membership check. **Indexes start at 1, not 0.** |
| `SetMapper<T>` | Insertion order | O(1) | ~3N + 1 | Yes, in insertion order | Ordered iteration AND fast membership checks; higher per-element cost than `UnorderedSetMapper`. |
| `UnorderedSetMapper<T>` | None | O(1) | ~2N + 1 | Yes, arbitrary order | Fast membership checks without caring about order — deduplication, processed-ID sets. |
| `WhitelistMapper<T>` | n/a | O(1) | N | **No** | Permission gates where you only ever ask "is X allowed?" and never enumerate members. |
| `MapMapper<K,V>` | Insertion order of keys | O(1) via `contains_key()` | ~4N + 1 | Yes — `iter()`, `keys()`, `values()` | Key→value store with iteration — balances, per-user settings. Most expensive per element. |

**Storage entries = unique storage keys, not bytes.** Actual byte cost also depends on the encoded size of `T`.

Note: `SetMapper` and `UnorderedSetMapper` both provide O(1) `contains()` — the deciding factor between them is ordering and storage cost, not lookup speed. Prefer `UnorderedSetMapper` when insertion order does not matter.

## Contract endpoints

| Mapper | Endpoints / Views |
|---|---|
| `SingleValueMapper` | `setCounter`, `getCounter` |
| `VecMapper` | `pushItem`, `getItem`, `itemCount` |
| `SetMapper` | `addToOrderedSet`, `orderedSetContains`, `orderedSetLen` |
| `UnorderedSetMapper` | `addToUnorderedSet`, `unorderedSetContains`, `unorderedSetLen` |
| `WhitelistMapper` | `addToWhitelist`, `isWhitelisted` |
| `MapMapper` | `setBalance`, `getBalance`, `hasBalanceEntry` |

## Project structure

```
storage-examples/
├── src/
│   ├── storage_examples.rs          # Contract source
│   └── storage_examples_proxy.rs    # Auto-generated proxy (sc-meta all proxy)
├── tests/
│   └── storage_examples_blackbox_test.rs
├── meta/                            # Build metadata
├── wasm/                            # Wasm build output
├── sc-config.toml                   # Proxy output path
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
sc-meta new --template storage-examples --name my-contract
```
