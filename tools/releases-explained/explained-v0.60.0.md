# Release: SpaceCraft SDK v0.60.0

Date: 2025-08-08

## Short description:

SpaceCraft v0.60.0 makes all the new Barnard functionality available by default for all smart contracts.


## Full description:

### Highlights:

SpaceCraft v0.60.0 removes the "Barnard" feature flag, which had been introduced in v0.59.0 to prevent developers from accidentally deploying contracts to mainnet with features that are not active yet.

After the Barnard activation this is no longer a concern.

To make the transition smoother, certain behavior changes for transfer-execute introduced in v0.59.0 have been reverted.

We initially attempted to pass all transfer-execute calls through the new fallible VM hook, but this had the potential to break some existing flows, hence the revert.

The Rust VM received support for the new block info hooks: `getBlockTimestampMs`, `getPrevBlockTimestampMs`, `getBlockRoundTimeMs`, `epochStartBlockTimestampMs`, `epochStartBlockNonce`, `epochStartBlockRound`.

This makes them usable in Rust integration tests. we also added some new syntax to easily configure block info in SC tests.

QoL improvement: mx-scenario-go now retries installs to avoid CI disruptions from weak connections.
