# Smart contract scenario runner system

*Currently in process of being extracted into a separate crate.*

This large module handles high-level scenario running functionality.

It consists of:
- the high-level scenario runner `model`,
- the mandos `executor`, which runs all debugger sessions and tests,
- convenience methods for easily setting up contract tests.

It does **not** contain:
- the low-level serde model,
- file read/write logic,
- the value interpreter.

These features are the scope of the `multiversx-chain-scenario-format` crate.
