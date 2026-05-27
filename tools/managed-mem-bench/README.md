# managed-mem-bench

A benchmarking and testing tool for `StaticApi` managed types.  It ships two
binaries that exercise different aspects of the implementation.

---

## `bench-leak` — heap-memory lifecycle benchmark

Installs a custom global allocator that tracks every heap allocation and
deallocation, then measures three numbers for each managed type (and for
`ManagedVec` of each element type):

| Column | What it tells you |
|--------|-------------------|
| `create` | Net bytes allocated after creating `NUM_ITEMS` instances |
| `hold` | Net bytes still live after dropping all Rust-side handles (data stays in `ManagedTypeContainer` until reset) |
| `residual` | Net bytes still live after `StaticApi::reset()` — should be near zero |

### Key insight

A `ManagedBuffer<StaticApi>` is just a thin Rust struct holding an integer
handle.  The actual bytes live inside the `ManagedTypeContainer` on the VM
side.  `StaticApi::reset()` discards the entire container.  A non-zero
`residual` indicates that some backing storage was not freed.

> **Note:** a small residual after reset is normal — thread-locals, internal
> caches, and Rust runtime structures may retain a modest amount of memory
> that is unrelated to managed-type data.  Use the numbers to track *relative*
> changes over time, not to assert an exact zero.

### Configuration

Edit the constants at the top of `src/bench_leak.rs`:

```rust
const NUM_ITEMS: usize = 100_000; // number of managed-type instances to create
const BUFFER_SIZE: usize = 100;   // payload size used for buffer-like types
```

### Running

```bash
cargo run -p managed-mem-bench --bin bench-leak
# or from inside the tool directory:
cargo run --bin bench-leak
```

### Example output

```
=== Individual managed types (100000 instances each, 100-byte payloads where applicable) ===

  type                                          create (bytes)    hold (bytes)    residual
  -----------------------------------------------------------------------------------------------
  ManagedBuffer                                    14 726 636        1 392           1 188
  BigUint                                           3 200 840          ...             ...
  ...
```

---

## `bench-threading` — multi-thread correctness tests

Verifies that `StaticApi` behaves correctly in a multi-threaded environment.
`StaticApi` stores its `ManagedTypeContainer` in thread-local storage, so
every OS thread owns a fully independent handle space.  Five properties are
checked:

1. **Thread isolation** – same handle number on two threads holds independent data.
2. **Reset isolation** – `StaticApi::reset()` on thread A does not affect thread B.
3. **Concurrent construction safety** – many threads create managed types in parallel without panics or data corruption.
4. **Handle identity is thread-local** – `ManagedBuffer<StaticApi>` is `!Send`; the compiler prevents moving managed values across threads.
5. **Correct cross-thread data transfer** – the safe pattern (materialise → send plain Rust type → reconstruct) is verified end-to-end.

### Running

```bash
cargo run -p managed-mem-bench --bin bench-threading
# or from inside the tool directory:
cargo run --bin bench-threading
```

### Example output

```
=== StaticApi multi-thread tests ===

[PASS] test_thread_isolation
[PASS] test_reset_isolation
[PASS] test_concurrent_construction
[PASS] test_handle_identity_is_thread_local  (handle #-200: thread A had "hello from A", main thread has "hello from main")
[PASS] test_cross_thread_data_transfer

All tests passed.
```
