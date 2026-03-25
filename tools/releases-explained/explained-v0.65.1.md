# Release: SpaceCraft SDK v0.65.1

Date: 2026-03-25


## Short description:

SpaceCraft v0.65.1 is a minor release focused on tooling improvements, SDK fixes, and base framework additions. The highlights are the new shard-aware wallet generation in sc-meta, a new ShardId type in chain-core, Display implementations for big number and payment types, and several SDK bugfixes.


## Full description:

### Overview

v0.65.1 is a focused patch release. It does not introduce new language features or testing paradigms, but refines several existing areas: the sc-meta tooling becomes more ergonomic for multi-shard workflows, the SDK becomes more robust, and the base framework gains quality-of-life additions for types used in everyday contract development.


### Chain core: ShardId and ShardConfig

A new ShardId type has been introduced in the chain-core crate to represent a shard identifier. It carries special values for the metachain and provides utilities for determining whether an address belongs to a system smart contract.

Alongside ShardId, a new ShardConfig struct encodes the shard topology of the network, including the number of shards and how addresses are distributed across them.

These types lay the groundwork for tooling and testing code that needs to reason about which shard a wallet or contract belongs to.


### sc-meta improvements

Three sc-meta subcommands received improvements in this release.

The wallet new command gains a --shard flag. Supplying a shard identifier causes the tool to generate a new wallet whose address falls in that specific shard. This is convenient when setting up test environments that require contracts or accounts in particular shards.

The all proxy command gains a --verbose flag. When passed, the command prints the path of each proxy file as it is generated, which helps when running the command over a large workspace and wanting to verify coverage.

The test-gen command now emits an insert_ghost_accounts() call in the generated test setup. This is used internally by tests that reference accounts not otherwise declared.

The test wallets module has also been extended: a for_shard function is now available, which returns the pre-defined test wallet for the given ShardId. This makes it straightforward to write tests that target specific shards.


### Base framework additions

ManagedArgBuffer now implements ManagedVecItem, making it possible to store argument buffers inside a ManagedVec and use them as generic payload types wherever ManagedVecItem is required.

The Display trait has been implemented for BigInt, BigUint, NonZeroBigUint, Payment, and EsdtTokenPayment. This means these types can now be used directly in format strings and with the standard printing utilities, which simplifies logging and debugging in tests and interactors.


### SDK fixes

Two bugs in the SDK were addressed:

A transaction decode issue in sdk-core has been fixed. Certain transaction responses were not being parsed correctly.

The SDK HTTP and dapp gateway proxies no longer require the gateway URL to be provided without a trailing slash. Trailing slashes are now accepted and normalized automatically, avoiding a common source of misconfiguration.
