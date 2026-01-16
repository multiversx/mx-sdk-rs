# Release: SpaceCraft SDK v0.55.0

Date: 2025-01-08

## Short description:

Release v0.55.0 integrates the features of the Spica release into the framework. These mainly revolve around EGLD+ESDT multi-transfers.

It also improves chain simulator testing, and adds some optimizations, especially around ManagedVec iteration.


## Full description:

### Spica release integration

EGLD+ESDT multi-transfers are now possible.

This release attempts to make EGLD behave similarly to an ESDT. The purpose in the long run is to unite them in the protocol as well, but for now it suffices to have them dealt with the same way in the SC framework.

When receiving payments, a new method is provided, which deals with all newly possible cases: `self.call_value().all_transfers()`. Reasonable backwards compatibility is also provided.

When sending transactions, `EgldOrEsdtTokenPayment` is the new preferred type, and is properly handled everywhere.

Spica also adds new built-in functions that need framework support: `ESDTSystemSCProxy`: `ESDTModifyRoyalties`, `SDTSetNewURIs`, `ESDTModifyCreator`, `ESDTMetaDataRecreate`, `ESDTMetaDataUpdate`.

All the new Spica features also had to be added to the Rust VM for consistent test behavior.


### Syntax cleanup

Simplified the syntax in 2 aspects:
- `#[payable]` now allowed instead of `#[payable("*")]`;
- `register_promise` allows callback, without calling a function on destination.


### Interactor support for "set state" on the chain simulator

It is now possible to set up the initial state in the chain simulator, when running chain simulator tests.


### Safety and optimizations:
- Fixed ownership for ManagedVec iterators, specifically reference iterators only produce references to the items.
- Performance improvements in ManagedVec iterators.
- Simplified the callback selector.
