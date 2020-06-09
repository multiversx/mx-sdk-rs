# Introduction


## Welcome to the elrond-wasm tutorial!

This tutorial is intended for anyone interested in building smart contracts on the Elrond Network quickly and easily, using Rust.

## Arwen VM

Elrond currently has a single VM, called Arwen. Arwen runs WebAssembly (wasm) binaries and communicates with an elrond-go node. Under the hood it uses [Wasmer](https://github.com/wasmerio/wasmer). 

Also, very importantly, Arwen is **[fast!](https://medium.com/elrondnetwork/improving-the-performance-of-smart-contract-execution-5e62808679ac)**

## Why Rust?

Any language that compiles to WASM can in principle be used to write smart contracts for Arwen, even writing WASM by hand is possilbe. However, only 3 solutions currently have support: Rust, C and Solidity.

Rust is the one with the most features and most support out-of-the-box. The elrond-wasm framework does a lot of work for the programmer automatically, such as checking and preparing arguments, dealing with storage and handling asynchronous calls.

## Why not C?

It is currently possible to write smart contracts in C, but there is no layer on top of the base API provided by Arwen. The developer needs to manually prepare all API arguments and process results. There is also no functionality for working with the heap.

It can work for small contracts, but it becomes difficult as the contracts become larger.

## Why not Solidity?

In order to help developers migrate their projects from Ethereum to Elrond, we tried bringing a Solidity to WASM compiler to the stack. There are 2 main problems though:
* The [SOLL compiler](https://github.com/second-state/soll) is not yet fully featured.
* More importantly, Elrond's architecture is fundamentally different from Ethereum and many concepts do not translate well. Especially contracts calling one another function differently in a sharded architecture.

If you already have a smart contract on Ethereum that you want to bring to Elrond, it is probably easier to simply rewrite it from scratch in Rust. For instance, it took a developer just a few hours to translate BUSD from [this](https://github.com/paxosglobal/busd-contract) to [this](https://github.com/ElrondNetwork/sc-busd-rs).

If you are starting out a new project, even more so go directly for Rust. 
