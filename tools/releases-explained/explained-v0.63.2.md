# Release: SpaceCraft SDK v0.63.2

Date: 2025-12-03

## Short description:

SpaceCraft v0.63.2 cleans up some APIs related to block timestamps in the legacy whitebox testing framework.


## Full description:

The legacy testing framework can now properly specify block timestamps either in seconds, or in milliseconds, using the new timestamp types. Some functions were renamed, for consistency with the rest of the smart contract and testing frameworks.

This is important for contracts that still maintain older tests built in this style, particularly the xExchange contracts.
