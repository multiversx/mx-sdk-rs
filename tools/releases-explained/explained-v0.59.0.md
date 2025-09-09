# Release: SpaceCraft SDK v0.59.0

Date: 2025-07-03


## Short description:

SpaceCraft SDK v0.59.0 is live.
‍
This release brings full support for the Barnard protocol upgrade, introducing expanded APIs, fallible transaction mechanisms, and improved contract tooling for MultiversX smart contract development.

It’s a major step forward for developers building advanced logic, tools, and governance flows on MultiversX.


## Full description:

SpaceCraft SDK v0.59.0 is live.
‍
This release brings full support for the Barnard protocol upgrade, introducing expanded APIs, fallible transaction mechanisms, and improved contract tooling for MultiversX smart contract development.

### Highlights:
‍
- Barnard-level APIs are now available, including precise block timestamps, code hash retrieval, and protocol-supplied ESDT token type info. These additions enable more accurate onchain logic and deeper contract introspection.
- Fallible synchronous calls and fallible transfer-execute are now fully supported and integrated into the unified syntax, streamlining complex execution flows and error handling.
- Back-transfers now support multi-transfer scenarios (including EGLD), resolving long-standing edge cases in payment logic. The system is redesigned for clarity, flexibility, and proper VM behavior.
- New proxies for the governance and delegation smart contracts are included, allowing seamless interaction with staking, delegation, and voting processes.
- Experimental support for building contracts with Rust’s standard library (std) is also included. While bulk memory operations are not yet supported, this lays the groundwork for future enhancements in contract ergonomics.
‍
As always, we’ve spent considerable time testing each component. We look forward to seeing what you build with it.