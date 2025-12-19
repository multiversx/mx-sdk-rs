# Release: SpaceCraft SDK v0.63.2

Date: 2025-12-08

## Short description:

SpaceCraft v0.63.3 reverts a backwards-incompatible ABI change.


## Full description:

`TokenIdentifier` was ernamed to `EsdtTokenIdentifier`, and the ABI was changed with it. Unfortunately, this breaks existing tooling so it needed to be reverted.
