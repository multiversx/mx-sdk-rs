# elrond-wasm-debug mandos system

This large module handles high-level mandos functionality.

It consists of:
- the high-level mandos `model`,
- the mandos `executor`, which runs all debugger sessions and tests,
- convenience methods for easily setting up contract tests.

It does **not** contain:
- the low-level serde model,
- file read/write logic,
- the value interpreter.

These features are the scope of the `mandos` crate.
