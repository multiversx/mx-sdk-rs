# Smart contract IDE debugging helper

A collection of scripts that help the IDE display relevant data, especially from managed types.

## LLDB Pretty-Printer Script for Smart Contracts

The LLDB pretty-printer script allows you to view managed types such as `BigUint` and `ManagedBuffer` in a readable format during debugging sessions.

### Prerequisites

- [**Visual Studio Code**](https://code.visualstudio.com/)
- [**CodeLLDB**](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (LLDB Debugger extension for Visual Studio Code)


### Installation

First, download the [**MultiversX LLDB Pretty-Printer Script**](https://github.com/multiversx/mx-sdk-rs/blob/master/tools/rust-debugger/pretty-printers/multiversx_sc_lldb_pretty_printers.py) to a known directory on your system.

Then, adjust the `lldb.launch.initCommands` entry in your Visual Studio Code settings to import the pretty-printer script:


```json
{
    "lldb.launch.initCommands": [
        "command script import /path/to/pretty-printers/multiversx_sc_lldb_pretty_printers.py"
    ]
}
```

Once you have completed the steps above, you can start debugging your smart contract in Visual Studio Code. The pretty-printer script will automatically format managed types for you.
