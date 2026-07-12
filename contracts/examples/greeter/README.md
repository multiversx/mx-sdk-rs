# Recipe: New contract from `sc-meta new --template empty`

The real, unedited output of `sc-meta new --template empty`, then the
smallest real customization on top (one storage mapper, one endpoint,
one view) — because a genuinely empty contract doesn't prove the
workflow actually works end to end. Every command below was run for
real in this environment (`rustc 1.93.0`, `cargo 1.93.0`,
`multiversx-sc-meta 0.64.1`) while authoring this recipe — see
"Validation" for exactly what that means and what it doesn't.

## Prerequisites

- Rust via `rustup`, with the `wasm32v1-none` target installed (see
  "A version note" below for why not `wasm32-unknown-unknown`).
- `sc-meta` (`cargo install multiversx-sc-meta`).
- Optional: `wasm-opt`, for size-optimized release builds (this recipe
  was verified without it — see Pitfall 3).

## Install

This recipe's own directory ships the already-generated-and-customized
project directly at its root (`Cargo.toml`, `src/`, `meta/`, etc. — the
same convention every other recipe in this Cookbook uses: clone and go,
no extra scaffolding step required):

```bash
git clone https://github.com/multiversx/cookbook.git
cd cookbook/recipes/new-contract-from-template
sc-meta all build
cargo test
```

To reproduce the scaffold from scratch instead (what Step 1 below
walks through): `sc-meta new --template empty --name greeter` creates a
**new subdirectory** named after `--name` — run it in an empty directory
of your own and treat the resulting `greeter/` as your project root
(this recipe's own root is exactly that directory's contents, moved up
one level for consistency with this Cookbook's other recipes).

## Step 1 — the bare scaffold

`sc-meta new --template empty --name greeter` does more than copy files:
it renames the trait, the crate names, the module paths, and the
scenario references throughout, all in one pass. This is the real,
captured output of running it standalone (the `greeter/` directory the
command itself creates — this recipe's own root, one level up, IS this
directory's contents, per "Install" above):

```
greeter/
├── Cargo.toml
├── multiversx.json
├── meta/
│   ├── Cargo.toml
│   └── src/main.rs
├── src/
│   └── greeter.rs
├── scenarios/
│   └── greeter.scen.json
└── tests/
    ├── greeter_scenario_go_test.rs
    └── greeter_scenario_rs_test.rs
```

`src/greeter.rs`, unmodified from the template:

```rust
#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Greeter {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
```

Two things worth noting immediately, both confirmed by actually running
the command rather than reading about it:

- **There is no `sc-config.toml`, no `wasm/`, and no `output/` yet.**
  CLAUDE.md's "Smart Contract Project Anatomy" shows these as if every
  contract project always has them; a bare `sc-meta new` output doesn't.
  `wasm/` and `output/` appear the first time you build (Step 3);
  `sc-config.toml` only exists once you need it (Step 2).
- **The scenario test files' function names don't get renamed.** Look at
  `tests/greeter_scenario_go_test.rs`: the *file* was renamed from
  `empty_scenario_go_test.rs`, and the scenario path inside was updated
  to `scenarios/greeter.scen.json` — but the test function itself is
  still `fn empty_go()`, not `fn greeter_go()`. The rename pass covers
  file names, crate names, and module/trait identifiers; it does not
  rewrite every string inside test bodies. Not a bug — just something to
  expect rather than be surprised by when you diff a freshly generated
  project.

## Step 2 — add a storage mapper, an endpoint, and a proxy

`src/greeter.rs` in this recipe adds one parameterized `SingleValueMapper`
(a per-caller greeting) and two members exposing it — the smallest real
next step past the bare scaffold, and the natural bridge into
[Storage mappers: which to pick, when](/smart-contracts-rust/storage-mapper-decision-table/):

```rust
#![no_std]

use multiversx_sc::imports::*;

pub mod greeter_proxy;

#[multiversx_sc::contract]
pub trait Greeter {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(setGreeting)]
    fn set_greeting(&self, message: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        self.greeting(&caller).set(message);
    }

    #[view(getGreeting)]
    #[storage_mapper("greeting")]
    fn greeting(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;
}
```

**Generating the proxy has a real chicken-and-egg order to it.** Adding
`pub mod greeter_proxy;` before the file exists fails the build
(`error[E0583]: file not found for module`) — confirmed by hitting this
error directly while authoring this recipe. The actual working order:

1. Add `sc-config.toml`:
   ```toml
   [[proxy]]
   path = "src/greeter_proxy.rs"
   ```
2. Write the contract logic WITHOUT the `pub mod greeter_proxy;` line yet.
3. Run `sc-meta all proxy` — this compiles the contract (which doesn't
   yet reference the proxy module, so nothing is missing) and generates
   `src/greeter_proxy.rs` from the resulting ABI.
4. Add `pub mod greeter_proxy;` now that the file exists.

## Step 3 — build

```bash
sc-meta all build
```

Real output, captured while authoring this recipe:

```
Building greeter.wasm in .../greeter/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo +1.93-aarch64-apple-darwin build --target=wasm32v1-none --release ...
    Compiling greeter v0.0.0 (...)
    Compiling greeter-wasm v0.0.0 (...)
    Finished `release` profile [optimized] target(s) in 6.78s
Copying .../target/wasm32v1-none/release/greeter_wasm.wasm to ../output/greeter.wasm ...
Warning: wasm-opt not installed.
Extracting imports to ../output/greeter.imports.json ...
Checking EI version: 1.5 ... OK
Packing ../output/greeter.mxsc.json ...
Contract size: 996 bytes.
```

`sc-meta all build` also regenerates `wasm/src/lib.rs`'s endpoint list on
every run — confirmed by diffing it before and after adding
`setGreeting`/`getGreeting`: it picked both up automatically without any
manual edit:

```rust
// Code generated by the multiversx-sc build system. DO NOT EDIT.
multiversx_sc_wasm_adapter::endpoints! {
    greeter
    (
        init => init
        upgrade => upgrade
        setGreeting => set_greeting
        getGreeting => greeting
    )
}
```

## Step 4 — test

This recipe adds `tests/greeter_blackbox_test.rs` — the scaffold's own
`greeter_scenario_*_test.rs` files only exercise deploy, not the added
logic. Two blackbox tests, following CLAUDE.md §"Blackbox Tests
(RECOMMENDED)" exactly: one confirms `setGreeting` → `getGreeting`
round-trips per caller, the other confirms two different callers get
independent storage (proving the parameterized mapper's key really does
include the address, not just in theory).

```bash
cargo test
```

Real output:

```
running 2 tests
test greeting_is_keyed_per_caller ... ok
test set_and_get_greeting ... ok
test result: ok. 2 passed; 0 failed; ...

running 1 test
test empty_go ... ok

running 1 test
test empty_rs ... ok
```

All 4 tests pass: the 2 new blackbox tests plus the 2 scaffold-provided
scenario tests (whose function names are still `empty_go`/`empty_rs` —
see Step 1).

## A version note

Three different version numbers showed up while authoring this recipe,
and they disagree:

| Source | `multiversx-sc` version |
| --- | --- |
| CLAUDE.md, "Current Version" | 0.65.0 |
| `mx-sdk-rs` GitHub `master` branch, `contracts/examples/empty/Cargo.toml` | 0.66.2 |
| This environment's installed `sc-meta` (0.64.1), and what it actually generated | 0.64.1 |

`sc-meta new` bundles its own template, versioned to match whatever
`multiversx-sc-meta` release you have installed — it does not fetch the
latest framework version from anywhere. If your installed `sc-meta` is
older than the framework's current release (as this environment's is),
your freshly scaffolded contract pins an older `multiversx-sc` than
CLAUDE.md calls "current." Run `sc-meta upgrade` after scaffolding if you
want the latest framework version, or pass `sc-meta new --tag <version>`
to pin one explicitly at generation time.

Also worth noting: the actual build command targets `wasm32v1-none`, not
`wasm32-unknown-unknown` as CLAUDE.md's "Build System" section states —
confirmed directly from the real `sc-meta all build` invocation logged
above. Install the right target via `rustup target add wasm32v1-none` if
`sc-meta install all` hasn't already done it for you.

## Pitfalls

1. **Adding `pub mod X_proxy;` before generating the proxy file breaks
   the build.** See Step 2's ordering — generate first, declare the
   module second.

2. **A bare `sc-meta new` output has no `sc-config.toml`.** CLAUDE.md's
   project anatomy diagram shows one unconditionally; you only need it
   once you want proxy generation or a multi-contract build.

3. **`wasm-opt` not being installed doesn't fail the build** — it's a
   warning, and `sc-meta all build` still produces a valid, deployable
   `.wasm`/`.mxsc.json`, just without size optimization. Verified
   directly: this recipe's build ran with no `wasm-opt` on `PATH` and
   still produced a working, tested contract. Install it before shipping
   to mainnet if contract size matters to you (bytecode storage has a
   real, per-byte on-chain cost).

4. **Scenario test function names surviving a rename are not a sign
   something went wrong.** See Step 1's second bullet.

5. **`sc-meta new`'s bundled template version may lag the framework's
   documented "current" version.** See "A version note" above — check
   what actually got generated (`cat Cargo.toml`) rather than assuming it
   matches the newest release.

## See also

- [Storage mappers: which to pick, when](/smart-contracts-rust/storage-mapper-decision-table/) —
  the natural next recipe: this one introduces exactly one mapper
  (`SingleValueMapper`, parameterized); that one covers the rest and when
  to reach for each.
- [Sign and send a transaction](/start-here/sign-and-send/) — the dApp
  side that would eventually call an endpoint like `setGreeting` from a
  connected wallet.

## Validation

Every command in this README was run for real in this environment:
`sc-meta new --template empty --name greeter` (captured file listing and
diff above), `sc-meta all proxy` (real generated
`src/greeter_proxy.rs`, shown in full in the source), `sc-meta all build`
(real WASM + ABI + `.mxsc.json` output, contract size 996 bytes), and
`cargo test` (4/4 tests passing: 2 new blackbox tests plus the 2
scaffold-provided scenario tests). This recipe has no `npm install`/
`tsc`/`eslint` gate — it's Rust, verified with `sc-meta`/`cargo` instead,
per this Cookbook's TypeScript-vs-Rust verification split (see
PROTOTYPE-NOTES.md's FINAL SUMMARY). `target/` and `output/` are
gitignored (regenerable build artifacts, the same convention this
Cookbook's TypeScript recipes use for `node_modules/`/`dist/`); `Cargo.lock`
and `wasm/Cargo.lock` are kept, mirroring `package-lock.json`.
