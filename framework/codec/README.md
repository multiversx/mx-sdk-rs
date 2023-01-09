# Smart contract standard codec

Lightweight binary serializer/deserializer, written especially for MultiversX smart contracts.

Designed to:
- produce minimal WASM bytecode
- be fast
- avoid data copy as much as possible

Largely inspired by the Parity SCALE codec, but a completely different format and implementation.

For more info about the serialization format, see [the developer reference](https://docs.multiversx.com/developers/developer-reference/elrond-serialization-format/).

# no-std

Being designed for MultiversX smart contracts, it needs to be able to run in a no-std environment.

It is also safe to run in a regular std environment.

The types provided by default all work without an allocator. To use an allocator, pass feature flag "alloc".
