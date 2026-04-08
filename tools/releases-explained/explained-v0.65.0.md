# Release: SpaceCraft SDK v0.65.0

Date: 2026-02-26


## Short description:

SpaceCraft v0.65.0 focuses on making blackbox tests easier to write and maintain. The highlight is scen-blackbox, a new sc-meta standalone tool that automatically generates Rust blackbox tests from existing mandos scenario JSON files. The release also includes ABI and TypeAbi improvements that support the generator, payment API additions, codec fixes, and VM cleanups.


## Full description:

### Overview

The central theme of v0.65.0 is improving the experience of writing blackbox tests. The MultiversX framework supports two testing paradigms: the older mandos JSON scenario format and the modern Rust blackbox testing style. This release bridges the two, making it easy to produce and grow a suite of blackbox tests.

### VM improvements

Several internal cleanups were made to the Rust VM:

- DebugHandle now holds a weak pointer to TxContext instead of a strong one. This fixes LLDB pretty-printer behavior when inspecting values after a transaction completes, since the handle could have previously kept TxContext alive past its natural lifetime.
- Signal error messages that contain non-UTF-8 bytes are now displayed using lossy decoding instead of crashing.
- The VMAddress alias has been removed. Code should use Address directly.
- The Shareable trait and with_shared_mut_ref helper were removed. These were internal VM infrastructure that is no longer needed.

### Blackbox test improvements: scen-blackbox

The flagship feature of v0.65.0 is the scen-blackbox tool, added as a new sc-meta standalone command. Its purpose is to make it easier to build and grow a blackbox test suite, by automatically generating Rust blackbox tests from existing mandos .scen.json scenario files.

The command is invoked as:

    sc-meta convert-scenarios

It reads the contract's ABI and scenario files and produces strongly-typed Rust code that exercises the blackbox testing API. The following aspects of mandos scenarios are fully translated: deploy, call, and query steps; EGLD and ESDT payments including multi-transfers; scalar, variadic, OptionalValue::None, and IgnoreValue arguments; token and address constants extracted as named Rust declarations; H256 constants; epoch, round, nonce, timestamp, and duration block info; error expectations; set-state for account balances, ESDT token state, and storage; and external .steps.json step file references.

The tool was validated by regenerating tests for digital-cash, ping-pong-egld, payable-features, order-book, and crypto-zombies.

Note that manual or AI interventions are necessary after generating the code.

### ABI changes

Two additions were made to the ABI format, primarily to support the scen-blackbox generator:

- rustMethodName is now included in the JSON ABI for each endpoint, where it differs from the endpoint name. This allows the generator to emit correct Rust proxy method calls.
- A specificType field was added, used to distinguish TimestampSeconds, TimestampMillis, DurationSeconds, and DurationMillis from plain u64 values. Without it, the generator could not emit the right Rust type for time-related arguments.
- PaymentMultiValue ABI fix: a bug in its ABI description was corrected.

### TypeAbi changes

- TypeAbiUniversalInput was added, allowing developers to bypass proxy argument type restrictions when needed.
- The IgnoreValue blanket TypeAbiFrom implementation was removed, as it conflicted with TypeAbiUniversalInput.
- ScenarioValueRaw was introduced as an alias for TypeAbiUniversalInput over BytesValue. The scen-blackbox tool uses it as a placeholder when it cannot determine the appropriate typed value for a mandos argument.

### Testing improvements

Several smaller improvements make writing blackbox tests more convenient:

Transaction ids are now supported in tests. The unified syntax allows capturing or asserting the transaction hash or nonce produced by a call, which is useful when correlating a test call with on-chain results.

Address and H256 now have const constructors from hex string literals, replacing the hex-literal crate dependency. This makes it easy to declare typed address and hash constants in test setup code.

Payment API additions:
- .payment() calls can now be chained on a transaction, and are automatically merged into a multi-payment at compile time. .esdt() and .multi_esdt() are deprecated in favor of .payment().
- NonZeroBigUint gained one(), try_from for integer primitive types and NonZero variants, and comparison operators against BigUint.
- Payment::try_new now accepts a generic error type.
- MultiEgldOrEsdtPayment to PaymentVec conversion now includes a validity check. Scenario payments use PaymentVec internally instead of MultiEgldOrEsdtPayment.
- TestTokenIdentifier has been renamed to TestTokenId, matching the TokenId naming from v0.64.0. The old name is kept as a deprecated alias.

Deprecations:
- MultiEsdtPayment is deprecated, superseded by PaymentVec.
- TestEsdtTransfer is deprecated; Payment is used instead.
- .commit() in the blackbox set/check account API is deprecated; state is applied immediately without it.

### Codec fixes and improvements

- A bool encoding and decoding was simplified and optimized.
- A bug in num-bigint encoding was fixed.
- MultiValueVec received new constructors and unit tests.
- Inline annotations were adjusted for better performance and consistency.

### Dependency upgrades

- reqwest was upgraded and the TLS backend was switched to rustls, a pure-Rust implementation. This removes the OpenSSL dependency and improves portability.
- Other dependencies were upgraded to their latest versions.

### LLDB pretty-printer fix

The LLDB Python formatter (multiversx_sc_lldb_pretty_printers.py) was updated to handle the DebugHandle structural changes described above.

