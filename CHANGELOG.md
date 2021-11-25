# Change Log

There are several crates in this repo, this changelog will keep track of all of them.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

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
