# MultiversX Smart Contract Framework

The crates in this folder form the MultiversX smart contract framework.

They are as follows:
    - `mx-sc` - the base crate for smart contract libraries, it is the only dependency the smart contract code sees.
    - `mx-sc-codec` / `mx-sc-codec-derive` - the standard serializer/deserializer for SC data
    - `mx-sc-derive` - procedural macros for friendlier SC code
    - `mx-sc-meta` - smart contract meta-programming: build system and other tools
    - `mx-sc-scenario` - the main testing tool, contracts are tested by via scenarios
    - `mx-sc-snippets` - base crate for tools that interact with the blockchain
    - `wasm-adapter` - the API that connects contracts to the WASM backend
