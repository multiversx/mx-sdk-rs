# Very Large Storage Benchmark

Benchmark contract that measures gas usage for operations on large storage values.

The contract holds a single storage entry `x` of type `ManagedBuffer`.
The `append` endpoint grows this entry by a configurable number of bytes, with the payload cycling through `0x00..=0xff`.
This allows measuring how gas consumption scales as the stored value grows larger.

## Contract Endpoints

- `append(num_bytes: u64)` — Appends `num_bytes` bytes to storage entry `x`. Bytes cycle through `0x00..=0xff`.
- `getXLen` (view) — Returns the current byte length of storage entry `x`.
- `getX` (view) — Returns the full contents of storage entry `x`.

## Interactor

The interactor deploys the contract and calls `append`, reporting gas used and the resulting total storage size.

```bash
# Deploy
cargo run deploy

# Append bytes and print gas used + total storage size
cargo run append -n 10000
```
