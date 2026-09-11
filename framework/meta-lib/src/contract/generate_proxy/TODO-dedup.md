# `AbiSourceGenerator` — reuse the macro's proxy codegen for `format = "abi-raw"`

## `write_raw`/`write_raw_method` duplicate `proxy_gen.rs`

`AbiSourceGenerator::write_raw`/`write_raw_method` (`abi_source_generator.rs`) hand-format, as
strings, the exact same shape that `generate_abi_proxy`/`generate_proxy_method` in
`data/abi-derive-common/src/contract/proxy_gen.rs` build with `quote!`: the `AbiProxyTrait` impl,
the `Methods` struct, one impl block per method, the `IntoDeploy`/`IntoUpgrade`/`IntoCall`
dispatch, and the `ProxyArg`/`apply_argument` chain. Anyone changing that shape (e.g. the
payable-generic pattern) has to remember to update both places, or they drift.

**Why they aren't already shared:** `proxy_gen.rs` consumes a `syn`-AST `Method`/`ContractTrait`
parsed from live Rust source, with *unprojected* `syn::Type`s — it emits `<T as TypeAbi>::Abi` as
source and lets the compiler resolve it later, during the same compilation. `abi_source_generator.rs`
instead starts from an already-executed `ContractAbi`'s string-based `TypeNames` (no AST, already
fully resolved via `pure_rust`, and possibly produced by a contract built with a different
compiler/framework version than the one running `sc-meta`) — there is no `syn::Type` to hand it.

**Fix:** parse the resolved type-name strings (`abi_type()`'s output) back into `syn::Type` via
`syn::parse_str`, synthesize a `Method`/`ContractTrait` from `EndpointAbi`/`ContractAbi`, and feed
that into `generate_abi_proxy` instead of hand-formatting. The resulting `proc_macro2::TokenStream`
needs pretty-printing before it's written to the file (`.to_string()` on tokens is compact/unreadable)
— pull in `prettyplease` for that. Requires `multiversx-sc-meta-lib` to add a regular (non-proc-macro)
dependency on `multiversx-sc-abi-derive-common`, which is already a plain lib crate (`proc-macro2`/
`quote`/`syn` deps only, no `proc-macro = true`) so this is a normal dependency edge, not a proc-macro
one.

Not done yet because it's a real refactor (AST bridging + pretty-printing), not a drop-in reuse.
