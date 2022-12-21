# elrond-wasm-output

Only contains the allocator and the panic handler, required for building the wasm output.

Import this crate in the crate that produces the wasm output;
do not import in any crate where you want debugging capabilities or unit tests.
