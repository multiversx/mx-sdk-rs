# managed-mem-bench

A benchmarking tool for detecting memory leaks in the `ManagedBuffer` implementation when used with `StaticApi`.

## What it does

The tool installs a custom global allocator that tracks every heap allocation and deallocation, then walks through three phases:

| Phase | Action | What it tells you |
|-------|--------|-------------------|
| 1 | Create `NUM_BUFFERS` `ManagedBuffer` instances, each `BUFFER_SIZE` bytes | How much heap the VM allocates per buffer |
| 2 | Drop the Rust-side handles | How much data is retained inside the `ManagedTypeContainer` after the Rust objects are gone (expected: most of it, because the VM owns the storage) |
| 3 | Call `StaticApi::reset()` | Whether the VM properly frees all buffer storage — residual should be 0 |

### Key insight

A `ManagedBuffer<StaticApi>` is just a thin Rust struct holding an integer handle. The actual bytes live inside `ManagedTypeContainer::managed_buffer_map` (a `HashMap<RawHandle, Vec<u8>>`) on the VM side. Dropping the Rust handle calls `drop_managed_buffer` into the VM to remove the entry. `StaticApi::reset()` discards the entire container.

A non-zero residual after phase 3 indicates a real leak — handles whose backing storage was not freed.

## Configuration

Edit the constants at the top of `src/main.rs`:

```rust
const NUM_BUFFERS: usize = 100_000; // number of ManagedBuffer instances to create
const BUFFER_SIZE: usize = 100;     // payload size of each buffer in bytes
```

## Running

```bash
cargo run -p managed-mem-bench
# or from inside the tool directory:
cargo run
```

## Example output

```
Baseline allocated bytes:    716
After creating 100000 x 100-byte ManagedBuffers: 14727352 bytes
  Net increase:              14726636 bytes
After dropping Rust handles: 2108 bytes
  Net change from baseline:  1392 bytes
After StaticApi::reset():    1904 bytes
  Net change from baseline:  1188 bytes
Result: LEAK — 1188 bytes were not released after reset.
```
