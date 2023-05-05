# MultiversX Smart Contract Framework

The crates in this folder form the MultiversX smart contract framework.

They are as follows:
    - `multiversx-sc` - the base crate for smart contract libraries, it is the only dependency the smart contract code sees.
    - `multiversx-sc-derive` - procedural macros for friendlier SC code
    - `multiversx-sc-meta` - smart contract meta-programming: build system and other tools
    - `multiversx-sc-scenario` - the main testing tool, contracts are tested by via scenarios
    - `multiversx-sc-snippets` - base crate for tools that interact with the blockchain
    - `multiversx-sc-wasm-adapter` - the API that connects contracts to the WASM backend
