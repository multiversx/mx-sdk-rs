# Smart contract testing and debugging

The crate helps writing smart contract tests. Debugging can be performed by running these tests in debug mode.

It provides mocks for the entire blockchain infrastructure, so no call to the actual VM is necessary.

For convenience, the debug crate is not `#[no-std]`.
