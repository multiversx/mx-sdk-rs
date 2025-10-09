# Release: SpaceCraft SDK v0.56.0

Date: 2025-01-23

## Short description:

Release v0.56.0 adds a new map type (`ManagedMapEncoded`), as well as providing several bugfixes.


## Full description:

### ManagedMapEncoded

A new map type was added. The old type, ManagedMap only maps from ManagedBuffer to ManagedBuffer. The new type can work with any encodable key type, and with any encodable and decodable value type.


### Bugfixes & improvements:

- Fixed a bug regarding the ESDT roles VM hook;
- Pretty representation for ManagedBuffer and other string-like types when debugging;
- API fix of an issue that was preventing set state in chain simulator;
- Snippets generator fixes involving the crate path and the upgrade result handler.

