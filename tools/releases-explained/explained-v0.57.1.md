# Release: SpaceCraft SDK v0.57.1

Date: 2025-04-04


## Short description:

Hotfix release v0.57.1 gives access to token properties, and fixes several issues with running integration tests.


## Full description:

Access to properties is given, via the method `get_token_properties`. Better support for this is expected with the Barnard release.

Bugfixes:
- System Sc proxy methods `esdt_metadata_recreate` and `esdt_metadata_update` were functioning properly.
- `sc-meta test --chain-simulator` was encountering some issues.
- Result handler `ReturnsTxHash` was not behaving properly.
