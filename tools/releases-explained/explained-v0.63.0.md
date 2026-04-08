# Release: SpaceCraft SDK v0.63.0

Date: 2025-11-17

## Short description:

SpaceCraft v0.63.0 cleans up the block into API, by introducing second and millisecond timestamp and duration types.

It also makes the Rust compiler version used for building smart contracts predictable and configurable. There are several more improvements in the build system.

Finally, it adds support for fallible sync calls in the Rust VM, allowing these calls to be used in blackbox tests.


## Full description:

### Block API improvements

Supernova reduces block time from 6 to 0.6 seconds. The old block API only has precision of a second, so it would sometimes yield the same timestamp (in seconds) for consecutive blocks.

The solution is to have some API functions that work on millisecond level. The VM functionality required for that was added in the Barnard release, and has been available for some months. It was prior to this release that Mandos (the testing system) was enhanced with millisecond block time configurations, and we started to migrate contracts and tests to the new block API. We quickly discovered that the transition was tricky, because it was easy to lose track of which values were in seconds and which in milliseconds, leading to conversion bugs. It also became obvious that legacy contracts cannot, or do not want, or do not need the millisecond API.

The solution to this problem was to introduce separate timestamp types: `TimestampSeconds` or `TimestampMillis`, and make the block API return these, instead of just `u64`. Variables of these types do not mix, trying to add seconds to milliseconds leads to compilation errors.

We also added 2 separate duration types, since durations and timestamps represent different physical values: one identifies a moment in time, and the other a time interval. We implemented the most common arithmetic operators in a way that makes sense in the real world, e.g.:

```
TimestampMillis - TimestampMillis = DurationMillis
TimestampMillis + TimestampMillis = does not compile
TimestampSeconds + DurationSeconds = TimestampSeconds
DurationSeconds + DurationSeconds = DurationSeconds
```

To preserve backwards compatibility, the old API functions were kept, with intact functionality. They are, however, deprecated, and new functions now return the new types. The pattern is:
- `[no suffix]` becomes `*_seconds`. This is the oldest API, yields seconds as `u64`, the new functions return `TimestampSeconds`.
- `*_ms` becomes `*_millis`. Nothing wrong with the "ms" suffix, but we needed a new suffix to preserve backwards compatibility. These functions date from Barnard, and were returning `u64`, now return `TimestampMillis`. There is also a block round time function, that now returns `DurationMillis` instead of `u64`.

Specifically:
- `get_block_round_time_ms` -> `get_block_round_time_millis` (returns `DurationMillis`)
- `get_block_timestamp` -> `get_block_timestamp_seconds`
- `get_block_timestamp_ms` -> `get_block_timestamp_millis`
- `get_prev_block_timestamp` -> `get_prev_block_timestamp_seconds`
- `get_prev_block_timestamp_ms` -> `get_prev_block_timestamp_millis`
- `epoch_start_block_timestamp_ms` -> `epoch_start_block_timestamp_millis`

Note: there is no epoch start block time in seconds, since it was only introduced in Barnard.

Values of type `TimestampMillis` and `TimestampSeconds` can also be used to set up blackbox tests, e.g.:

```
    world
        .epoch_start_block()
        .block_timestamp_ms(TimestampMillis::new(123_000_000))
        .block_nonce(15_000)
        .block_round(17_000);
```


### Rust VM support for fallible sync calls

Barnard introduced fallible sync calls: with this method contracts can recover from a synchronous same-shard call, if the callee fails.

This mechanism was only implemented in the Rust VM now, so now it can be used in blackbox and whitebox tests.

Additional effort was to align the event logs from the Rust VM with the ones from the main, Go VM.


### Contract build improvements

#### A. Rust compiler version

In order to set up reproducible builds, one of the most important aspects is to always use the same compiler and tooling versions. The SpaceCraft framework is making steps toward integrating reproducible builds into its build system, directly. This release includes a few such steps.

First, the Rust version is now sent to the wasm build command via CLI, always. This overrides all other settings, giving the framework full control over the Rust version used.
	
This Rust version can be configured in `sc-config.toml`. This overrides all other settings. If missing, the current config will be detected and used explicitly. This config normally comes either from `rustup default`, or from `rust-toolchain.toml`. However, setting it explicitly in `sc-config.toml` is the most reliable way of getting it right.

#### B. `wasm-opt` version

The `wasm-opt` version can also be specified in `sc-config.toml`. While `sc-meta` cannot install or change this version, it will crash if there is a version mismatch, signaling problems with reproducible builds. If configured as such, it will halt the build process if `wasm-opt` is missing, instead of just issuing a warning.

The `wasm-opt` tool is used to reduce smart contract binary size, and its impact is significant. It is part of the Binaryen tool suite, and can be installed separately, or via `sc-meta install wasm-opt`.

#### C. Incompatibility detector

The `sc-meta` standalone tool signals version incompatibilities when building contracts. Most importantly it writes a warning to console if multiversx-sc version < `v0.58` and rustc ≥ `v1.87`. This combination is not supported because that version of Rust added LLVM 20, which produces bulk memory opcodes, currently not supported by our VM.

We noticed that this was a common problem with developers running on an older version of the framework, and wanted to address this. There is no way to go back in time and change the old versions of the framework, but the `sc-meta` tool works with all versions and can detect incompatibilities.

When calling `sc-meta all build`, the new mechanism works as follows:
1. `sc-meta` calls `all abi` first, this produces ABIs for all contracts, including the rustc versions and framework versions;
2. For each contract it checks for incompatibilities, most notably if multiversx-sc version < `v0.58` and rustc ≥ `v1.87`. It also checks for nightly Rust before multiversx-sc v0.50 and knows about minimum versions for the compiler, per release (although this is somewhat redundant, the compiler performs this check as well). Incompatibility warnings are issued (the process is not stopped).
3. The build process continues normally from here.

This detector requires full backward ABI compatibility, so this is ensured starting with this release, going back to the first version of the ABI. A suite of backwards compatibility tests are checking this as part of our CI.

We also added an LLVM version and the host to the build info in the ABI.

#### D. Deprecated VM hooks checker

Developers will not notice this, but the build system now runs a post-build check for deprecated VM hooks. There are many hooks that we don't use anymore, and never will again, but need to be kept for blockchain integrity. The framework no longer uses them. This is an extra check to warn, if they ever somehow accidentally get used again. We are going to go through all of them in the VM, mark the deprecated ones, and generate the list, but for now the mechanism exists, and a few sample such hooks.


### `TokenIdentifier` renamed to `EsdtTokenIdentifier`

The next release will bring a big revamp of the payment system. For now we just added this rename. `TokenIdentifier` is historical, before we mixed EGLD with the ESDTs, and so the name is misleading, since it (also historically) specifically excludes EGLD. `EsdtTokenIdentifier` is the appropriate name.

The old name is not deprecated yet, but might be in the future.


### Fixed proxy imports in snippets

Fixed an issue with proxy imports when generating snippets. The wrong contract name was sometimes being used.
