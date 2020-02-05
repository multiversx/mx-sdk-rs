# elrond-wasm-rs

Rust smart contract library designed for Elrond's Arwen VM. Also provides a debugging mode with mocks.

# Examples

For examples on how to use the Elrond WASM framework, see https://github.com/ElrondNetwork/sc-examples-rs

# IDE

The framework is designed to be easiest to use with the Elrond IDE VSCode extension: https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide

# Manual build

To build a smart contract without the IDE, run the following command in the contract crate:
```
cargo build --bin wasm --target=wasm32-unknown-unknown --release
```

The resulting .wasm file will be in directory target/wasm32-unknown-unknown/release/wasm.wasm

# Debugging

Step-by-step debugging of smart contracts is possible in VSCode. To do this, it is required to have a separate debug crate and to have tasks.json and launch.json in .vscode properly configured. See https://github.com/ElrondNetwork/sc-examples-rs for examples on how to set this up. 

# Advanced

To debug macros:
```
cargo +nightly rustc --bin wasm -- -Z unstable-options --pretty=expanded > demacroed.rs
```

To check wasm size:
```
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm.wasm
```