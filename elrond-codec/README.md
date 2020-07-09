# elrond-codec

Lightweight binary serializer/deserializer, written especially for Elrond smart contracts.

Designed to:
- produce minimal WASM bytecode
- be fast
- avoid data copy as much as possible

Largely inspired by the Parity SCALE codec, but a completely different format and implementation.

# no-std

Being designed for elrond-wasm smart contracts, it needs to be able to run in a no-std environment.
