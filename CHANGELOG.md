# Change Log

This file contains a centralizes a trace of all published crate versions, with their changes in short.

## Versioning the crates

The `mx-sdk-rs` repo contains many crates, grouped into several families. Crates in these families always have the same version with one another.

For brevity, the changelog will only mention a short version of their name.

They are:
- `multiversx-sc`, in short `sc`, the smart contract framework, 6 crates + 3 for contracts/modules:
	- `multiversx-sc`
    - `multiversx-sc-derive`
    - `multiversx-sc-meta`
    - `multiversx-sc-scenario`
    - `multiversx-sc-snippets`
    - `multiversx-sc-wasm-adapter`
    - `multiversx-sc-modules` - *standard contract modules*
	- `multiversx-price-aggregator-sc` - *core contract*
	- `multiversx-wegld-swap-sc` - *core contract*
- `multiversx-sc-codec`, in short `codec`, the serializer/deserializer, 2 crates:
	- `multiversx-sc-codec`
	- `multiversx-sc-codec-derive`
- `multiversx-chain-vm`, in short `vm`, a Rust VM implementation, 1 crate.
- `multiversx-chain-scenario-format`, in short `scenario-format`, scenario JSON serializer/deserializer, 1 crate.
- `multiversx-sdk`, in short `sdk`, allows communication with the chain(s), 1 crate.

## [sc 0.43.4] - 2023-09-18
- Bugfix in `sc-meta`: fixed `--locked argument` in `all` command.
- Template fix: added `multiversx.json` files.
- Testing framework: check NFT balances and attributes.

## [sc 0.43.3, vm 0.5.2] - 2023-09-08
- Added several new methods in the `SendWrapper`, which perform EGLD & ESDT transfers but don't do anything if the value is zero.
- Added the `DeleteUsername` builtin function to the VM.
- Minor fixes in API wrapper constructors.

## [sc 0.43.2] - 2023-08-18
- Template tool tag argument validation bugfix.

## [sc 0.43.1, vm 0.5.1] - 2023-08-18
- Template tool improvements:
	- Ability to specify for which framework version to download (based on git tag). The first allowed version is 0.43.0.
	- Ability to specify path where to create new contract.
	- Various bugfixes.
- VM implementation for `get_shard_of_address` VM hook.

## [sc 0.43.0, codec 0.18.1, vm 0.5.0] - 2023-08-16
- Fixed a rustc compatibility issue when building contracts. The meta crate looks at the rustc version when generating the wasm crate code:
	- pre-rustc-1.71;
	- between rustc-1.71 and rustc-1.73;
	- latest, after rustc-1.73. Also upgraded some dependencies, notably proc-macro2 "1.0.66" and ed25519-dalek "2.0.0".
- Initial version of the contract template tool in multiversx-sc-meta:
	- Ability to download and adapt template contracts, to kickstart contract development;
	- A template mechanism that is customizable on the framework side;
	- Available templates: adder, empty, crypto-zombies.
- The Rust debugger is now thread safe.
- Removed the `big-float` feature of multiversx-sc, because the functionality is already available on mainnet.
- Arguments `--target-dir-wasm`, `--target-dir-meta`, and `--target-dir-all` in the `multiversx-sc-meta` CLI.
- Fixed an issue with contract calls and ESDT transfers in the `StaticApi` environment.

## [sc 0.42.0, codec 0.18.0, vm 0.4.0, scenario-format 0.20.0, sdk 0.2.0] - 2023-07-15
- Multi-endpoints in multi-contracts:
	- It is now possible to have multiple versions of the same endpoint in different multi-contract variants.
	- We can also have multiple versions of the constructor.
- Major architectural redesign of the debugger:
	- The VM executor interface inserted between the smart contract API objects and the Rust VM. A new `VMHooksApi` is used to connect on the smart contract side. A `VMHooksDispatcher` object and `VMHooksHandler` interface provide the connection on the VM side.
	- The `VMHooksApi` comes in several flavors (backends):
		- The old `DebugApi` is now only used at runtime, on the VM context stack;
		- A new `StaticApi` provides support for managed types in a regular context, without needing to be initialized;
		- An additional `SingleTxApi` is useful for unit tests. Aside managed types, it also allows some basic context for tx inputs, results, storage and block info.
	- Removed almost all of the legacy functionality from the smart contract APIs.
- System SC mock.
	- It is now possible to issue tokens (fungible, SFT, NFT) in integration tests.
	- Setting roles is modelled.
	- It is, however, not fully mocked.
- Integration of blackbox and whitebox testing into one unified framework.
	- Whitebox testing was the modus operandi of the old testing framework.
	- Integration of whitebox functionality into the new testing framework allows easier migration in some specific cases.
	- Tested the new whitebox framework with the old tests by injecting it into the implementation of the old one.
- Interactors can now export a trace of their execution, thus producing integration tests.
	- Integrated tool for retrieving the initial states of the involved accounts from the blockchain.
	- Tight integration with the scenario testing infrastructure makes generating the trace straightforward;
	- The same format for the trace is used, as in the case of the integration tests.
- Interactors can now execute several steps (calls, deploys) in parallel.
- Redesigned the wrappers around the Rust and Go JSON scenario executors;
	- Also improved the  `sc-meta test-gen` tool for auto-generating these wrappers.
	- Using the `ScenarioRunner` interface to abstract away the various backends used to run tests.
- Redesigned syntax of both the testing and the interactor (snippets) frameworks.
	- While the codebases are separate (the latter is async Rust), the names and arguments of the methods are the same, and both use the scenario infrastructure.
	- Methods that allow chaining scenario steps, while also processing results;
	- Added several defaults in the syntax, for more concise code;
	- Deprecated the old testing framework;
	- Updated all contract interactors and blackbox tests with the new syntax;
	- Upgraded the snippets generator to produce new syntax.

## [sc 0.41.3, vm 0.3.3] - 2023-06-19
- Bugfix on `ManagedBufferCachedBuilder`, involving large inputs.
- Explicit enum ABI: `OperationCompletionStatus` is now properly described in the ABI as an enum that gets serialized by name instead of discriminant.

## [sc 0.41.2, codec 0.17.2, vm 0.3.2] - 2023-06-09
- Releasing a new version of the codec, without the dependency to `wee_alloc`.

## [sc 0.41.1, vm 0.3.1] - 2023-05-15
- Fixed an edge case for the token storage mappers (`FungibleTokenMapper`, `NonFungibleTokenMapper`).

## [sc 0.41.0, vm 0.3.0] - 2023-05-05
- Fixed compatibility with rustc v1.71.0.
- Allocator system:
	- Contracts can now choose their own allocator. This works in multi-contract contexts.
	- New allocators: `fail` (default), `static64k`, `leaking`.
	- Removed dependency to `wee_alloc`, but using it is still possible if the contract references it directly.
	- Contract call stack size is now configurable in `multicontract.toml`.
	- The 'panic with message' system now relies on managed buffers instead of on an allocator.
- Fixed BigUint bitwise operations in the debugger.
- When building contracts, an additional `.mxsc.json` file is created, which packs both the contract binary, the ABI, and some additional metadata.
- Refactor: reorganized the meta crate.
- Deprecated some legacy methods in the API wrappers.

## [sc 0.40.1, vm 0.2.1] - 2023-04-24
- Building contracts also triggers an EI check, which verifies compatibility with various VM versions. It currently only issues warnings.
- `ManagedVecItem` implementation for arrays.

## [sc 0.40.0, vm 0.2.0] - 2023-04-20
- Call value `egld_value` and `all_esdt_transfers` methods return `ManagedRef` instead of owned objects, because they are cached (to avoid accidental corruption of the underlying cache).

## [sc 0.39.8, vm 0.1.8] - 2023-03-29
- `multiversx-sc-meta` `test-gen` command: generates Rust integration tests based on scenarios present in the `scenarios` folder.
 - `UnorderedSetMapper` `swap_indexes` method.

## [sc 0.39.7, vm 0.1.7] - 2023-03-18
 - `TokenIdentifier` `ticker` method.
 - `ManagedBuffer` `concat` method.

## [sc 0.39.6, vm 0.1.6] - 2023-03-16
- `multiversx-sc-meta` improvements:
	- Bugfix: custom names in the main contract no longer crash the multi-contract build.
	- Bugfix: the `--mir` flag works correctly in `sc-meta all build`;
	- Multi-contract configs can now specify separate cargo features for individual contracts, for conditional compilation.

## [sc 0.39.5, vm 0.1.5] - 2023-02-06
- `multiversx-sc-meta` improvements:
	- Rust snippet generator fixes. The generator creates compilable code with appropriate argument types.
	- `local-deps` command: generates a report on the local depedencies of contract crates. Will explore indirect depdencies too.
	- Upgrade tool minor fix.

## [sc 0.39.4, vm 0.1.4] - 2023-01-26
- `multiversx-sc-meta` improvements:
	- `--locked` flag get passed to the build command, preserves dependencies in Cargo.lock.
	- `update` command updates Cargo.lock files without building the contracts.
- Backwards compatibility for running scenarios using the VM Go infrastructure.

## [sc 0.39.3, vm 0.1.3] - 2023-01-26
- `multiversx-sc-meta` improvements:
	- `upgrade` can handle crates as early as `0.28.0`;
	- `--ignore` flag for the `all` command: will ignore folders with given names, by default set to `target`;
	- `info` command, shows contracts and contract library crates with their respective framework versions;
	- `--mir` flag when building, also emits MIR files;
	- printing to console the build command.
- `BigUint` from `u128` conversion.

## [sc 0.39.2, vm 0.1.2] - 2023-01-19
- `multiversx-sc-meta` improvements:
	- `all` command that allows calling all contract meta crates in a folder;
	- `upgrade` also re-generates wasm crates after reaching 0.39.1.
- Cleaned up dependencies.

## [sc 0.39.1, codec 0.17.1, vm 0.1.1, scenario-format 0.19.1, sdk 0.1.1] - 2023-01-18
- `multiversx-sc-meta` can be installed as a standalone tool (`sc-meta`), and used to automatically upgrade contracts.
- Many depedencies updates across the repo.
- Updated readme files.

## [sc 0.39.0, codec 0.17.0, vm 0.1.0, scenario-format 0.19.0, sdk 0.1.0] - 2023-01-12
- All crates were renamed, in line with the MultiversX brand.
- New crate: `multiversx-chain-vm`, extracted from the old debug crate.
- New crate: `multiversx-sdk`, adapted from a solution proposed by the community.
- A `ScenarioWorld` facade, for contract tests.
- The meta crate supports `twiggy` post-processing, this is a tool to analyze contract size and investigate bloat in the binaries.
- Dropped crate: `elrond-wasm-output`. There is no equivalent crate, its job was passed to the individual `wasm` crates.
- `ManagedVec` supports sorting and deduplication.
- `migrateUserName` builtin function mock.

## [elrond-wasm 0.38.0, elrond-codec 0.16.0, mandos 0.18.0] - 2022-12-15
- `ContractCall` refactor. Building a contract call comes with harder compile-time constraints. This also reduces compiled code size.
- `ContractBase` supertrait can be now stated explicitly for contract and module traits.
- Debugger:
	- Callback payment is now set correctly.
	- Function names are represented internally as strings instead of bytes, which aids debugging.
- Removed the `ei-1-2` feature, which was guarding the newer VM functions. These functions are in the mainnet, so this feature is no longer needed.
- New utility functions: `self.send().esdt_local_burn_multi(...`, `self.blockchain().get_token_attributes(...)`.
- Updated all crates to Rust 2021.

## [elrond-wasm 0.37.0, elrond-codec 0.15.0] - 2022-12-09
- Multi-contract build system:
	- build system refactor;
	- `multicontract.toml` config system with labels,
	- eliminated monomorphization issue that was bloating some contracts;
	- build post-processing: `wasm2wat`, imports via `wasm-objdump`.
- Support for the new async call system (promises):
	- new APIs;
	- a new flavor of callbacks (`#[promises-callback]`);
	- callback optimizations.
- `elrond-codec` refactor: removed `TopEncodeNoErr`, `NestedEncodeNoErr` and `TypeInfo`
- System SC proxy: added support for `controlChanges` endpoint and transfer create role (from community).
- Module updates:
	- `MergedTokenInstances` module;
	- Governance module improvements;
	- `set_if_empty` for FungibleTokenMapper and NonFungibleTokenMapper.
- `IntoMultiValue` trait.
- Storage mapper improvements:
	- Storage mappers can read from another contract.
	- `BiDiMapper` improvements;
	- Fixed missing substitution rules for `FungibleTokenMapper`, `NonFungibleTokenMapper`, `UniqueIdMapper`, `BiDiMapper`, `WhitelistMapper`, `RandomnessSource`;
	- Added `take` and `replace` methods for `SingleValueMapper`;
	- Implemented `Extend` trait for `UnorderedSetMapper`.

## [elrond-wasm 0.36.1] - 2022-11-01
- Deprecated `ContractCall` `execute_on_dest_context_ignore_result` method, since it is currently redundant.

## [elrond-wasm 0.36.0, elrond-codec 0.14.0] - 2022-10-13
- `EsdtTokenPayment` legacy decode: objects encoded by older versions of the framework can now also be decoded, if flag `esdt-token-payment-legacy-decode` is active.
- Codec `NestedDecodeInput` new  `peek_into` method.
- `FungibleTokenMapper` caches the token identifier.

## [elrond-wasm 0.35.0, elrond-codec 0.13.0, mandos 0.17.0] - 2022-09-20
- Rust interactor snippet generator.
- Added some missing substitution rules in the contract preprocessor.
- Allow single zero byte when top-decoding Option::None.
- Ongoing operations module.
- Claim developer rewards module.
- `FromIterator` trait for `ManagedVec`.
- Mandos `"id"` accepted as synonym to `"txId"`.

## [elrond-wasm 0.34.1] - 2022-07-19
- `#[only_admin]` annotation
- Safer BigUint/BigInt conversions
- Added and published `price-aggregator` and `wegld-swap` core contracts.

## [elrond-wasm 0.34.0, elrond-codec 0.12.0, mandos 0.16.0, elrond-interact-snippets 0.1.0] - 2022-07-08
- Major refactor of the mandos-rs infrastructure.
	- High-level Mandos objects moved to elrond-wasm-debug;
	- The `mandos` crate no longer depends on `elrond-wasm-debug` (as originally intended and implemented);
	- Typed mandos contract call objects, for better call syntax.
	- More syntactic sugar for writing mandos calls.
- The first version of elrond-interact-snippets, which can be used to write short blockchain interactor programs.
	- The syntax relies on contract proxies to easily build calls.
	- Some of the infrastructure is shared with Mandos.
	- There is an example of such a interactor for the multisig contract.
- Refactor of managed type handles in all API traits. Eliminated undefined behavior when using the same handle in multiple contexts.
- Transfer role proxy module.
- NFT merge module.
- `#[only_user_account]` annotation. Only user accounts can call these endpoints.
- ABI - fixed missing event logs from modules.

## [elrond-wasm 0.33.1, mandos 0.15.1] - 2022-06-24
- CodecSelf for BigInt

## [elrond-wasm 0.33.0, mandos 0.15.0] - 2022-06-20
- Removed the data field for direct EGLD & ESDT transfers.
- Testing and debugging environment aligned with VM version 1.4.53.
- Call value and token data infrastructure additional cleanup.

## [elrond-wasm 0.32.0, mandos 0.14.0] - 2022-06-03
- VM new functionality added as part of the environment interface 1.2:
	- Fully managed functionality for elliptic curves (no allocator);
	- Fully managed cryptographic functions (no allocator);
	- More efficient printing of big ints and hex;
	- Functionality available by adding the `ei-1-2` flag to contracts.
- `BigFloat` functionality. Since the functionality is not yet deployed on mainnet, use flag `big-float` to use.
- Major refactoring of the call value mechanism:
	- `TokenIdentifier` now only refers to ESDT, for mixed EGLD+ESDT we have `EgldOrEsdtTokenIdentifier`.
	- `EsdtTokenPayment` now only refers to ESDT, for mixed EGLD+ESDT we have `EgldOrEsdtTokenPayment`.
	- Compact version for multi-transfer: `let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();`.
	- Explicit `single_esdt` vs. `single_fungible_esdt` vs. `egld_or_single_esdt` vs. `egld_or_single_fungible_esdt`.
	- Payment arguments are still supported, although discouraged. They always assume the EGLD+ESDT scenario.
- `ManagedOption` provides some minor optimization for specific use-cases. Mostly for use in the framework.
- Cleanup in the callback mechanism and in the `SendApi`.
- `SparseArray` implementation.
- `UniqueIdMapper` - efficient storage mapper for holding unique values.
- The ABI also contains events.
- New standard module: `StakingModule`.


## [elrond-wasm 0.31.1, mandos 0.13.1] - 2022-05-04
- Bugfix - formatter single char issue.

## [elrond-wasm 0.31.0, elrond-codec 0.11.0, mandos 0.13.0] - 2022-05-02
- Improved formatter. Strings can be formatted similarly to the standard Rust ones, but without allocator, using managed buffers. Macros `require!`, `sc_panic!`, `sc_format!`, `sc_print!` use it.
- Removed build flag `ei-1-1`, following mainnet updated and new VM endpoints being available. Among others, managed `sha256` and `keccak256` APIs can be used freely.
- `CodecFrom` and `CodecInto` traits to define equivalent encodings and conversions via codec.
- Generated smart contract proxies use the `CodecFrom`/`CodecInto` traits to accept a wider range of types.
- Mandos Rust testing framework v2, which uses contract proxies for composing calls and is capable of building and exporting mandos scenarios.
- Managed type handle management system in the contract, to reduce the number of API calls to the VM. General VM API refactor.
- Eliminated `#[var_args]` annotation. The framework can now distinguish between single-values and multi-values solely based on type.
- Contract cleans up return data after performing synchronous calls. Getting return data by range is no longer needed and the respective methods have been removed.
- Fixed behavior of blockchain API `get_esdt_token_data`.
- Git tag/commit info in ABI (fixed & reintroduced).

## [elrond-wasm 0.30.0, elrond-codec 0.10.0] - 2022-03-17
- Feature flags in `elrond-wasm`:
	- `alloc` allows contracts to use the heap allocator. It is not a hard restriction, there is still access to the implementations of the heap-allocated types, but they are not imported. Some methods are only available with this flag.
	- `ei-1-1` allows contracts to use VM endpoints that are not yet available on the mainnet.
- Fixes with async calls, smart contract deploy & upgrade.
- Refactoring regarding small number types in the API.
- Rust testing framework: Allow checking NFT balance without also checking attributes.
- View for `MapMapper`.

## [elrond-wasm 0.29.3] - 2022-03-03
- `ManagedVec` backwards compatible implementation for `set`.
- Implemented `ManagedVecItem` for `Option<T>`.

## [elrond-wasm 0.29.2] - 2022-03-01
- Disabled git tag/commit info in ABI due to issue in standard modules.

## [elrond-wasm 0.29.0] - 2022-03-01
- Cleaned up allocator from modules: `DnsModule`, `EsdtModule`, `FeaturesModule`, `PauseModule`, `UsersModule`.
- Crypto API managed wrapper over legacy VM endpoints.
- Managed multi-value types refactor and rename.
- `ManagedVec` - `remove`, `contains`, `find`.
- `ManagedVecItem` derive for simple enums.
- Feature `cb_closure_managed_deser` replaced by `cb_closure_unmanaged_deser`, managed implementation is now the default.
- Git tag/commit info in ABI.

## [elrond-wasm 0.28.0, elrond-codec 0.9.0, mandos 0.12.0] - 2022-02-22
- Major elrond-codec refactor:
	- Redesigned the error handling for single value encoding
	- Introduced multi-value encoding, which replaces the previous endpoint argument and result mechanisms
- Mandos improvements:
	- Multi-values: out, topics, ESDT uri
	- Logs "+" wildcard
- Builtin function mocks: `ESDTNFTUpdateAttributes`, `ESDTNFTAddURI`
- New storage mappers: `FungibleTokenMapper`, `NonFungibleTokenMapper`, `WhitelistMapper`
- Call value wrapper avoids using invalid token index in requests

## [elrond-wasm 0.27.4, elrond-codec 0.8.5] - 2022-02-02
- Backwards compatibility fix.

## [elrond-wasm 0.27.3] - 2022-01-31
- Backwards compatibility fix.
- Trailing commas are allowed in `sc_panic!`, `require!` and `sc_print!`.
- EsdtTokenData `decode_attributes_or_exit` for easier error handling.

## [elrond-wasm 0.27.2, elrond-codec 0.8.4] - 2022-01-27
- Added missing non-specialized decode implementations for managed types.

## [elrond-wasm 0.27.1] - 2022-01-27
- Deriving `PartialEq` now works on structs that contain managed types.

## [elrond-wasm 0.27.0] - 2022-01-25
- Fixed certain compilation error messages. The previous implementation of the macro preprocessor would have concealed the location of many issues.
- Changed implementation of `require!`:
	- `require!` no longer returns a `SCResult` type, when the condition is false it now stops the transaction immediately, via `signal_error`;
	- `require!` now accepts message formatting;
	- `require_old!` gives access to the old implementation.
- The Rust testing framework can now handle panics and async calls.
- ABI bugfix - an issue regarding nested types.
- `meta` crate build also attempts to call `wasm-opt` after building the contracts.
- Refactored `CodeMetadata` and added "payable by SC" field.
- Empty contract template.

## [elrond-wasm 0.26.0] - 2022-01-19
- Major VM API trait refactoring. All API methods can be accessed from a static context. Removed api instance variables from all objects.
- External view contracts
	- Annotating one or more endpoints with `#[external_view]` triggers the framework to create a second "external view" contract where all these endpoints are placed. This is primarily to reduce the main contract size.
	- General `meta` crate functionality refactor to allow multiple contract generation.
- `ManagedRef` type
	- Provided as a more efficient alternative to regular references to managed types
	- Has `Copy` semantics
	- `ManagedVec` iterators made safer by the proper use of lifetimes
	- `ManagedVec` `get_mut` offers a safe mutable reference, using lifetimes
	- Some initial optimizations in storage mappers
- First version of a message formatter based on `ManagedBuffer`s:
	- `sc_print!` macro
	- `sc_panic!` macro
- Random number generator wrapper over randomness source from the VM.

## [elrond-wasm 0.25.0] - 2021-12-14
- Rust testing framework - mandos generation fixes and some more getters
- Standard modules moved to `elrond-wasm-modules` crates

## [elrond-wasm 0.24.0] - 2021-12-07
- Rust testing framework
- Managed Crypto API - keccak256 and sha256
- New hook for ESDT local roles
- Only-owner module annotation

## [elrond-wasm 0.23.1, elrond-codec 0.8.3] - 2021-11-25
- `ArrayVec` serialization
- `ManagedAddress` additional conversions

## [elrond-wasm 0.23.0] - 2021-11-23
- Static access to API. Static thread-local context stack in the debugger.

## [elrond-wasm 0.22.11] - 2021-11-17
- Derive `ManagedVecItem` generics fix
- Constructor can reside in module

## [elrond-wasm 0.22.10] - 2021-11-12
- `ManagedMultiResultVec` push accepts multi result

## [elrond-wasm 0.22.9] - 2021-11-12
- `ManagedVarArgsEager` implementation
- `EsdtLocalRoleFlags`, no heap allocation in `get_esdt_local_roles`

## [elrond-wasm 0.22.8, elrond-codec 0.8.2] - 2021-11-12
- Optimized decode unsigned number from slice

## [elrond-wasm 0.22.7] - 2021-11-12
- Optimized decode unsigned number from slice
- Optimized blockchain API: managed get token nonce, get esdt balance
- `ManagedVecItem` for `ManagedByteArray`

## [elrond-wasm 0.22.6] - 2021-11-11
- Optimized decode u64 from `ManagedBuffer`
- `ManagedVecItem` in `derive_imports`

## [elrond-wasm 0.22.5] - 2021-11-11
- Implemented `ManagedVecItem` for `bool`.
- Substitution for `ManagedMultiResultVec::new()`.

## [elrond-wasm 0.22.4] - 2021-11-11
- Derive `ManagedVecItem`.
- Nested encode and decode from ManagedBuffers cached in a static singleton buffer.
- Implemented `ExactSizeIterator` for `ManagedVecIterator`.

## [elrond-wasm 0.22.3] - 2021-11-10
- Memory allocation optimisations.

## [elrond-wasm 0.22.2] - 2021-11-06
- Callback endpoint automatically created empty for contracts that have no callbacks. This is determined by the `meta` crate, based on the ABI of the contract and its modules.
- `UnorderedSetMapper`
- `IgnoreVarArgs` variadic argument type that ignores input

## [elrond-wasm 0.22.1] - 2021-11-04
- Made the generated code in `wasm/lib.rs` more compact with the use of macros.

## [elrond-wasm 0.22.0] - 2021-11-02
- Mechanism for generating contract endpoints based on ABI. Previously, all endpoints from all modules from a crate were automaticaly included, now they can be filtered based on what modules are used.
- Contract `meta` crates are now capable of building the respective contracts and the ABIs without relying on `erdpy`.
- Renamed feature `arwen-tests` to `mandos-go-tests`

## [elrond-wasm 0.21.2] - 2021-10-26
- Bugfix regarding contract upgrade args in `elrond-wasm-debug`

## [elrond-wasm 0.21.1, elrond-codec 0.8.1, mandos 0.11.1] - 2021-10-26
- Relative path improvements and fixes in `elrond-wasm-debug`:
	- mandos-rs `file:` syntax now actually loads files and correctly unifies equivalent paths
	- debugging now works seamlessly, without needing to temporarily change paths in the tests
- SC proxy - `register_meta_esdt`
- Debugger builtin function mocks check for ESDT roles
- ABI provides definitions for EsdtTokenPayment, EsdtTokenData, EsdtTokenType

## [elrond-wasm 0.21.0, elrond-codec 0.8.0, mandos 0.11.0] - 2021-10-22
- Mandos support for NFT syntax. Many more small improvements and some major refactoring.
- Major refactoring of the `elrond-wasm-debug` crate, which enables the debugger and the coverage tool. Many features added:
	- support for synchronous calls, also nested synchronous calls
	- support for NFT simple transfers
	- support for ESDT multitransfer (FT + NFT)
	- builtin functions mocked in the debugger: `ESDTLocalMint`, `ESDTLocalBurn`, `MultiESDTNFTTransfer`, `ESDTNFTTransfer`, `ESDTNFTCreate`, `ESDTNFTAddQuantity`, `ESDTNFTBurn`, `ESDTTransfer`, `ChangeOwnerAddress`, `SetUserName`
	- supports deploy/deploy from source/upgrade/upgrade from source from contracts
- `#[payment_multi]` annotation
- `ManagedRef` type, that allows easier handling of managed types
- ABI contains endpoint mutability flag (mutable/readonly)
- reverse iteration for `ManagedVec`

## [elrond-wasm 0.20.1] - 2021-10-05
- Added missing managed methods in blockchain API: `is_smart_contract`, `get_shard_of_address`, `get_balance`.
- Improved preprocessor substitutions: `ManagedAddress`, `TokenIdentifier`.

## [elrond-wasm 0.20.0, elrond-codec 0.7.0, mandos 0.10.0] - 2021-10-02
- Managed callback handling
- Managed async call result
- ManagedVec improvements, deserialization fix
- Better conversions between big numeric types
- Improved preprocessor substitutions: hidden generics for most managed types
- Build info in ABI - rustc version, framework version, crate version

## [elrond-wasm 0.19.1] - 2021-09-17
- Legacy Send API implementation fix

## [elrond-wasm 0.19.0, elrond-codec 0.6.0, mandos 0.9.0] - 2021-09-10
- Managed types used extensively. Because of this, the recommended Arwen minimum version is `v1.4.10`.
	- Redesigned parts of the elrond-codec, so as to allow custom type specializations. These specializations allow serializers and types to bypass the limitations of the codec traits to provide optimized implementations. Managed type serialization relies on this.
	- Redesigned existing managed types: `BigInt`, `BigUint`, `EllipticCurve`.
	- Added the `ManagedBuffer` type, which can be used to store anything on the VM side.
	- Support for complex operations using managed buffers, such as storing lists of elements in a managed buffer via the `ManagedVec` type.
	- There are `ManagedAddress`es now. They rely on another managed type, the `ManagedByteArray`, which is a fixed size managed structure.
	- `TokenIdentifier` is now a managed type.
	- Serializer based on a managed buffer.
	- Storage keys are now based on managed buffers.
	- All error messages generated by the framework are assembled using a managed buffer.
	- The blockchain API uses managed types for most interactions.
	- The contract call API uses managed types for most interactions.
	- The call value API supports multi transfer via managed `EsdtTokenPayment` objects.
	- Event logs are sent to the VM via managed types (`ManagedVec<ManagedBuffer>` for topics, `ManagedBuffer` for data).
	- Type conversion traits for managed types: `ManagedFrom` and `ManagedInto`.
	- There are now 2 types of `SCError`: `StaticSCError` for static messages and `ManagedSCError`, which is backed by a managed buffer.
	- Contract errors can now be triggered immediately, without the need to return them from an endpoint.
- Improved macro preprocessor: more complex patterns can now be substituted.
	- Generic API parameter needs not be specified every time.
	- Substitutions available for most managed types and storage mappers.
- Separated contract API into low-level VM API connectors and high-level utility objects to be used in the contracts.
- Mandos-rs improvements:
	- Self tests synchronized with mandos-go. Some missing features needed to be added to make them pass.
	- Support for ESDT tokens.
	- Support for ESDT multi-transfer.


## [elrond-wasm 0.18.2] - 2021-08-20
- Crypto API: `ripemd160` function, custom secp256k1 signature verification (`verify_custom_secp256k1`) and signature generation (`encode_secp256k1_der_signature`).

## [elrond-wasm 0.18.1] - 2021-08-05
- Added "safe" storage mappers, which serialize keys using nested encoding instead of top. The old respective mappers only kept for backwards compatibility, are now deprecated.

## [elrond-wasm 0.18.0, mandos 0.8.0] - 2021-07-28

- New math hooks exposed from Arwen:
	- `pow`, `log2`, `sqrt`
	- cryptography: elliptic curves
- `deploy_contract` now returns `Option<Address>`
- `deploy_from_source_contract` API
- Send API refactored for more consistency and ease of use.
- High level proxies can be used to deploy contracts.
- Mandos log syntax updated, to match Arwen.
- A better `#[only_owner]` annotation, which can be applied directly to endoint methods. This annotation also shows up in the ABI.
- `elrond-wasm-derive` now an optional dependency of `elrond-wasm`. Use `#[elrond_wasm::contract]` instead of `#[elrond_wasm_derive::contract]` now. Same for proxies and modules.

## [elrond-wasm 0.17.4] - 2021-06-30
- conversions from big ints to small int: `BigUint::to_u64`, `BigInt::to_i64`

## [elrond-wasm 0.17.3] - 2021-06-11
- `SingleValueMapper` `set_if_empty` method

## [elrond-wasm 0.17.2] - 2021-06-04
- callbacks can now declared in modules only (manual forwarding from the main contract no longer required)

## [elrond-wasm 0.17.1] - 2021-06-04
- `legacy-nft-transfer` feature for interacting with older versions of Arwen

## [elrond-wasm 0.17.0] - 2021-05-28
- Integration tests can now call Arwen-Mandos (mandos-go)
- Send API refactoring and cleanup
	- ESDT builtin function calls no longer require explicit gas
	- sync calls and transfer-execute no longer require explicit gas
- `#[payment_nonce]` endpoint argument annotation
- `#[payable]` annotation no longer allowed without argument

## [elrond-wasm 0.16.2, mandos 0.7.2] - 2021-05-20
- New implementation for the `Try` trait for `SCResult`, in accordance to feature `try_trait_v2`
- Published DNS module, which helps contracts register usernames for themselves
- `ESDTLocalRole` more expressive type ABI

## [elrond-wasm 0.16.1, mandos 0.7.1] - 2021-05-18
- Improvements in mandos-rs: username, contract owner, nested async calls

## [elrond-wasm 0.16.0, mandos 0.7.0, elrond-codec 0.5.3] - 2021-05-14
### Major redesign of important framework components:
- The arguments to contract/module/proxy annotations are gone. All items are generated in the same Rust module. Both submodule inclusion and contract calls are now Rust-module-aware.
- Submodule imports are now expressed as supertraits instead of the module getter annotated methods. Note: explicitly specifying the Rust module is required, in order for the framework to fetch generated types and functions from that module.
- Each contract now generates its own callable proxy to ease calling it. Caller contracts do no longer need to define a call interface, they can import it from the crate of the contract they want to call. Callable proxies contain the methods from the main contract, as well as from all the modules. Note: calling a contract requires the caller to specify the Rust module where it resides.
- We no longer have a separate syntax/parser/code generation for call proxies. They are just contracts with no implementations and annotated with `#[elrond_wasm_derive::proxy]` instead of `#[elrond_wasm_derive::contract]`.
- BigUint and BigInt are now associated types instead of generics in all API traits. Contracts need to specify them as `Self::BigUint` instead of just `BigUint`. Although more verbose, this might be more intuitive for the developer.
- `ContractCall`s, `AsyncCall`s and all other call & transfer result types now contain a reference to the Send API. This also means the `execute_on_dest_context` method no longer requires an api argument.
- `execute_on_dest_context` can now deserialize the call results automatically and provide them to the calling contract. There is a mechanism in place to deconstruct non-serialized types, e.g. `SCResult<T>` becomes `T` and `AsyncCall<Self::BigUint>` becomes `()`. 
- Callbacks and callback proxies needed to be adapted to the new system, but work similar to how they did in the past.
- Contracts can define proxy getter methods using the `#[proxy]` annotation.
- Callbacks can now have names, just like endpoints. This name gets saved in the callback closure in storage, but has no other impact on the contract. The reason I needed it was to help me with defining callback forwarders and avoiding some name collisions there. Callback forwarders are still needed for a little longer, until module callbacks are properly implemented.

### Mandos
- mandos-rs syntax synchronized with mandos-go (`sc:` syntax, new ESDT call value syntax, _no NFTs yet_).

## [elrond-wasm 0.15.1] - 2021-04-30
- Mitigating nested sync calls with Send API `execute_on_dest_context_raw_custom_result_range`

## [elrond-wasm 0.15.0, elrond-codec 0.5.2] - 2021-04-19
- ABI
	- Constructor representation
	- Simplified ABI syntax for tuples and fixed-size arrays
- Final cleanup for the contract APIs: split off blockchain and crypto APIs
- Small fixes in the send API
- `TokenIdentifier` validation
- Minor refactoring in the elrond-codec 

## [elrond-wasm 0.14.2] - 2021-03-29
- Fixed contract call/callback logs in mandos-rs

## [elrond-wasm 0.14.1] - 2021-03-25
- Unified variadic arguments with respective variadic results

## [elrond-wasm 0.14.0, mandos 0.6.0, elrond-codec 0.5.1] - 2021-03-22
- ESDT functionality:
	- ESDT system smart contract proxy, though which it is possible to mint, burn, issue, freeze, pause, etc.
	- Endpoints to handle NFTs. Also added NFT management in the  ESDT system smart contract proxy
	- Get balance, get token data, local mint/burn
- Contract calls:
	- Low-level and high-level support for synchronous calls via `execute_on_dest_context`.
	- Callback bug fix
- Improvements in storage mappers:
	- VecMapper length is now lazy
	- UserMapper more functionality
- Mandos
	- `scQuery` step
	- fixed defaults: unspecified fields now check the default value instead of being ignored
	- check logs
	- `nested:` and `biguint:` syntax
- `elrond-codec-derive` dix - `TopDecodeOrDefault` works with generics
- Upgraded to Rust2021.

## [elrond-wasm 0.13.0] - 2021-03-04
### Main feature
- Events revamped:
	- any event name of any length is accepted. The event name is now expressed as ASCII instead of hex
	- topics can have any length
	- topics and data are serialized using the elrond-codec instead of the old macro-based solution
	- old events are still allowed for now via the `#[legacy_event("0x...")]` syntax; might be removed in the future
### Refactoring 
- Major refactoring of elrond-wasm-derive. This doesn't change much of the functionality, though.
### Minor features
- SingleValueMapper redesigned for easier use. It no longer keeps the storage value cached.

## [elrond-wasm 0.12.0] - 2021-02-25
- Reorganized ESDT and EGLD direct send api.
- New async call syntax
	- redesigned contract proxies
	- contract calls are communicated via objects returned from endpoint methods
	- callbacks now specified programmatically
	- got rid of the `#[callback_arg]` annotation

## [elrond-wasm 0.11.0, elrond-codec 0.5.0, mandos 0.5.0] - 2021-02-05
### Refactor
- Major refactoring of the contract API: split into smaller traits
### Added
- Storage mappers:
	- LinkedListMapper
	- SetMapper
	- MapMapper
- SendApi
	- created SendApi, which groups all functionality related to sending tokens and interactions with other contracts
    - integrated the new TransferESDT hook from Arwen
    - added an unsafe buffer for handling values before transfer
    - mandos-rs fixes
    - contracts now use the new API + more mandos tests
- Call Value API refactor and `#[payable]` updates:
	- Main features:
    	- `#[payable]` annotation more versatile: `#[payable("EGLD")]` `#[payable("TOKEN-ID")]` `#[payable("*")]`
    	- `#[payable]` still accepted but throws a warning, will become unsupported in the future.
    	- `#[payment]` argument attribute now also provides ESDT payment where applicable
    	- a new TokenIdentifier type that encodes the EGLD special token and any ESDT token
    	- a new `#[token_identifier]` argument attribute provides the token id. Similar to `#[payment]` it is a fake argument, not exported.
    	- ABI updated ("payableInTokens" is no longer restricted to "EGLD")
    	- all new features covered by mandos tests
    	- async proxies still only accept `#[payable("EGLD")]`, but that is for future updates
	- Less visible changes:
    	- all call value hooks now grouped in a new CallValueApi
    	- for low-level access, developers now need to write self.call_value().egld_value(), etc.
    	- some optimizations in the handling of call value hooks
	- Refactoring:
    	- parse_attr mod was split into a proper folder with many files, since it had grown too large
    	- an extensive refactoring of elrond-wasm-derive not yet performed, will come soon
### Minor features
- ABI enum discriminants generation
### Fixed
- Crypto API fixes:
	- `keccak256:` prefix also supported in mandos
    - reorganized crypto mandos tests in basic-features
    - mandos-rs was accidentally providing keccak256 instead of sha256


## [elrond-wasm 0.10.5] - 2021-01-27
- Temporary fix: callbacks allow error message argument to be missing

## [elrond-wasm 0.10.4, mandos 0.4.2] - 2021-01-13
- Codec derive with defaults
- Storage mapper infrastructure

## [elrond-wasm 0.10.3] - 2020-12-29
- ABI generation of endpoint output names

## [elrond-wasm 0.10.2, elrond-codec 0.4.2] - 2020-12-28
- Codec type hygene

## [elrond-wasm 0.10.1, elrond-codec 0.4.1, mandos 0.4.1] - 2020-12-23
- Minor fixes, support for strings

## [elrond-wasm 0.10.0, elrond-codec 0.4.0] - 2020-12-21
- Codec derive
- ABI generation framework
- New example contracts

## [elrond-wasm 0.9.8, elrond-codec 0.3.2, mandos 0.3.1] - 2020-11-23
- SC deploy API

## [elrond-wasm 0.9.7, elrond-codec 0.3.1, mandos 0.3.0] - 2020-11-11
- Monomorphization via codec trait instead of TypeInfo for arguments and storage
- Reorganized all contracts in the `contracts` folder

## [elrond-wasm 0.9.6] - 2020-11-09
- H256 & BoxedBytes fixes

## [elrond-wasm 0.9.5] - 2020-11-09
- H256 is_zero, minor fixes

## [elrond-wasm 0.9.4] - 2020-11-09
- BoxedBytes
	- optimized allocation, used in hooks
	- used for error messages

## [elrond-wasm 0.9.3] - 2020-11-08
- Optimized Address/H256 hooks

## [elrond-wasm 0.9.2] - 2020-11-06
- Allow slices as arguments 
- `storage_is_empty` annotation

## [elrond-wasm 0.9.1] - 2020-11-05
- BigUint serialization bugfix

## [elrond-wasm 0.9.0, elrond-codec 0.3.0, mandos 0.2.0] - 2020-11-04
- Serialization completely refactored to use "fast exit" methods
- Storage/argument/result traits completely redesigned, simplified and optimized
- Completely ditched the approach from elrond-wasm 0.8.0.

## [elrond-wasm 0.8.0, elrond-codec 0.2.0] - 2020-11-02
- Was the first version to split Encode/Decode into TopEncode/NestedEncode/TopDecode/NestedDecode
- Attempted to optimize the serializer to use "fast exit" closures. It worked, but the resulting bytecode size was not satisfactory. Even though it was completely replaced and never got to be used, it historically remains the solution of this release.
- Some of the storage/argument/result trait refactorings, survived.

## [elrond-wasm 0.7.2] - 2020-10-16
- small int EI
- minor refactors, serialization fixes

## [elrond-wasm 0.7.1] - 2020-10-07
- Avoid function selector infinite loop
- Crowdfunding contract initial commit

## [elrond-wasm 0.7.0, mandos 0.1.0] - 2020-10-06
- Code coverage now possible
- Mandos in Rust
- Modules properly integrated in the build process

## [elrond-wasm 0.6.2] - 2020-09-16
- NonZeroUsize iterator and utils

## [elrond-wasm 0.6.1, elrond-codec 0.1.3] - 2020-09-15
- Integrated NonZeroUsize into the framework
- Specialized small int top encoding/decoding
- `only_owner!` macro

## [elrond-wasm 0.6.0, elrond-codec 0.1.2] - 2020-08-25
- Redesigned the entire build process with wasm crates
- Standard modules
- Moved all example contracts from sc-examples-rs to the framework

## [elrond-wasm 0.5.5] - 2020-07-27
- H256 now boxed
- SCResult is_ok, is_err

## [elrond-wasm 0.5.4, elrond-codec 0.1.1] - 2020-07-18
- MultiResultVec - new, from_iter
- EncodeError type

## [elrond-wasm 0.5.3, elrond-codec 0.1.0] - 2020-07-10
- Extracted elrond-codec to separate crate
- Fixed non_snake_case endpoint handling

## [elrond-wasm 0.5.2] - 2020-07-09
- Queue type

## [elrond-wasm 0.5.1] - 2020-07-02
- `#[view]` attribute, same as `#[endpoint]`
- `#[init]` attribute
- `storage get mut` annotation + BorrowedMutStorage
- Encode for references
- Array serialization/deserialization
- Option serialization fix
- Arg name in error message
- Async call arguments based on traits

## [elrond-wasm 0.5.0] - 2020-06-29
- EndpointResult trait, arg serialization trait, arg loader
- Variadic args/results: OptionalArg, OptionalResult, MultiResultX

## [elrond-wasm 0.4.6] - 2020-06-21
- MultiResultVec implementation
- Callback varargs

## [elrond-wasm 0.4.5] - 2020-06-09
- `storage_set` allows slices
- H256 to_vec
- async call and callback argument fixes
- eliminate bloat when no callback
- the new elrond lightweight serializer (would later become elrond-codec)
- imports macro
- OtherContractHandle implementation

## [elrond-wasm 0.4.4] - 2020-05-19
- Serialization fixes for small ints
- `get_cumulated_validator_rewards` hook

## [elrond-wasm 0.4.3] - 2020-05-11
- Allow any (macro-based) serializable argument in async call
- `#[var_args]`
- Call data serialization refactoring

## [elrond-wasm 0.4.2] - 2020-05-07
- Tutorial setup (later abandoned)

## [elrond-wasm 0.4.1] - 2020-05-06
- Direct storage conversion for simple types
- Block info hooks

## [elrond-wasm 0.4.0] - 2020-05-06
- Serde-based serializer (later abandoned)
- Major storage improvements:
	- Generate storage getters & setters
	- Variable length storage keys

## [elrond-wasm 0.3.2] - 2020-04-13
- Fixes in the macro-based argument handling

## [elrond-wasm 0.3.0] - 2020-04-03
- Raw callback support
- `storage_load_len` hook
- Multi args
- Multi args in async calls

## [elrond-wasm 0.2.0] - 2020-03-18
- BigUint trait created, added operators (including bitwise)
- BigUint used for balances

## [elrond-wasm 0.1.1] - 2020-02-27
- Async call contract proxy infrastructure

## [elrond-wasm 0.1.0] - 2020-02-05 
- Initial relase of the framework
- Main features at this time:
	- contract main macro
	- handling of arguments and results automagically using macros
	- BigInt generic type, hooked directly to the Arwen big int heap
	- `#[private]` attribute

## [Initial commit] - 2020-01-04
- Early framework moved here from sc-examples
- 4 crates:
	- elrond-wasm
	- elrond-wasm-derive for macros
	- elrond-wasm-node for wasm
	- elrond-wasm-debug for debugging and early tests
