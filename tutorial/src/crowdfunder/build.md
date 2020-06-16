# Building the project

There are 2 ways to build a smart contract project:

## As a WASM binary

The WASM mode is the main one, and the point of the framework. The result is a WASM binary that can be deployed on the blockchain.

To build this way, call from the project directory:
```
erdpy build .
```

## Debug mode

It is currently not possible to debug step by step on the WASM binary. Until a solution for that exists, we have decided to allow developers to also build the contract as a standalone Rust application with the entire blockchain interaction mocked. The project is set up in such a way that when building and running the `debug` project, the developer can debug the contract.

The debugger is not yet fully featured, but it works for simple usecases. Specifically for the way the current project is configured, it should work in VSCode by simply pressing `F5`.