# Recipe: Storage mappers — which to pick, when

A working, tested contract exercising six storage mappers side by side,
plus a decision table built from the real `multiversx-sc` crate source
doc comments (`~/.cargo/registry/.../multiversx-sc-0.64.2/src/storage/mappers/*.rs`,
read directly while authoring this recipe) — not just CLAUDE.md's
summary. **Two of CLAUDE.md's own table entries turned out to be wrong**
against that source; see "Corrections to CLAUDE.md's table" below.

## Prerequisites

- Rust via `rustup`, with the `wasm32v1-none` target installed.
- `sc-meta` (`cargo install multiversx-sc-meta`).
- Familiarity with [New contract from `sc-meta new --template empty`](/smart-contracts-rust/new-contract-from-template/) —
  this recipe assumes that scaffolding workflow and jumps straight to the
  mapper comparison.

## Install

```bash
git clone https://github.com/multiversx/cookbook.git
cd cookbook/recipes/storage-mapper-decision-table
sc-meta all build
cargo test
```

## The decision table

| Mapper | Ordering | Membership check | Storage entries for N items | Iterable? | Pick it when |
| --- | --- | --- | --- | --- | --- |
| `SingleValueMapper<T>` | n/a (one value) | n/a | 1 | n/a | You need exactly one value — a counter, a config flag, a total. |
| `VecMapper<T>` | Insertion order | Linear scan only (no built-in `contains`) | N + 1 (`.len` + one `.item{i}` per element) | Yes, 1 to `len()` | You need ordered, indexable, append-friendly storage and don't need fast membership checks — a log, a history, a queue you only ever read front-to-back. **Indexes start at 1, not 0.** |
| `SetMapper<T>` | Insertion order (doubly-linked internally) | **O(1)** — confirmed from source, contra CLAUDE.md (see below) | ~3N + 1 (info + node_links×N + value×N + node_id-lookup×N) | Yes, in insertion order, plus `next()`/`previous()` navigation | You need both ordered iteration AND fast membership checks, and can afford the higher per-element storage cost — an audit trail of unique events, a whitelist where insertion order itself is meaningful. |
| `UnorderedSetMapper<T>` | None | O(1) | 2N + 1 (`.len` + `.item{i}`×N + `.index{value}`×N) — CLAUDE.md says N+1, undercounting the reverse-lookup keys needed for O(1) `contains` (see below) | Yes, arbitrary order | You need fast membership checks and don't care about order — deduping, a processed-IDs set, a role list you only ever check `contains()` against. |
| `WhitelistMapper<T>` | n/a | O(1), most storage-efficient of the set-shaped mappers | N (one boolean-flag key per item, nothing else) | **No** — cannot enumerate members at all | You only ever need "is X allowed?" and never "list everyone allowed" — permission gates, feature flags, admin checks. |
| `MapMapper<K,V>` | Insertion order of keys (built on `SetMapper` internally) | O(1) via `contains_key()` | ~4N + 1 (a full `SetMapper` for keys, ~3N+1, plus one `.mapped{key}` value entry per item) | Yes — `.iter()`, `.keys()`, `.values()` | You need a real key→value store with iteration — balances, per-user settings, anything you'd reach for a `HashMap` for off-chain. Confirmed as the one CLAUDE.md entry that checks out exactly ("4N+1 entries (expensive!)"). |
| `LinkedListMapper<T>` | Insertion order, efficient front/back ops | Not built in | ~2N + 1 (info + one node key per element) | Yes | You need efficient push/pop from BOTH ends — a FIFO/LIFO combined structure. Not exercised with working code in this recipe (see "What this recipe didn't test" below); described from CLAUDE.md plus the source module's struct layout only. |

**Storage-cost column is expressed in unique storage keys, not bytes** —
each entry's actual byte cost also depends on the encoded size of `T`.
The point of this column is relative ordering between mapper choices for
the same logical data, not an absolute gas number.

## Corrections to CLAUDE.md's table

Read directly from `multiversx-sc-0.64.2`'s source doc comments while
authoring this recipe — CLAUDE.md's own "Available Storage Mappers"
table has two claims that don't match:

1. **`SetMapper<T>`'s `contains()` is O(1), not O(n).** CLAUDE.md lists
   it as `"O(n) contains"`. The real source's doc comment states
   plainly: `"Contains: contains(value) - Checks membership. O(1) with
   one storage read."`, and lists `"O(1) insert, remove, and contains"`
   as a Pro. `SetMapper` maintains a separate value→node-ID lookup
   specifically to make `contains()` O(1) — that's the entire reason its
   storage layout is more complex than a plain ordered list. This
   recipe's `set_mapper_contains_and_ordering` test exercises `contains()`
   for both a present and an absent value; functional correctness is
   confirmed, though this recipe doesn't independently benchmark
   complexity — the O(1) claim itself is the crate's own documented
   design intent, not a claim this recipe re-derived from scratch.

2. **`UnorderedSetMapper<T>` costs `2N + 1` storage entries, not `N + 1`.**
   CLAUDE.md's table lists `"N+1 entries"`. The real source's storage
   layout section lists two distinct key groups: value storage
   (`.len` + `.item{index}` per element — this is the `N + 1` CLAUDE.md
   counted) AND a separate `.index{encoded_value}` reverse-lookup key
   per element, which is what actually delivers the O(1) `contains()`
   both CLAUDE.md and this recipe agree the mapper provides. You cannot
   get O(1) membership testing from `N + 1` keys with no reverse
   index — the extra `N` keys are the cost of that guarantee, not an
   optional extra.

Everything else in CLAUDE.md's table (that this recipe independently
checked against source) held up, including `MapMapper`'s "4N+1 entries
(expensive!)" — genuinely the most expensive mapper here per element,
confirmed by reading how it's built on top of `SetMapper` internally
plus its own value storage.

## Files

`src/storage_mappers.rs` — one endpoint pair per mapper.
Every method name (`.insert()`, `.contains()`, `.contains_key()`,
`.add()`, `.push()`, `.get()`) is copied from the real crate source's own
doc-comment examples, not guessed — worth calling out since, for
instance, `SetMapper`/`UnorderedSetMapper` both use `.contains()` while
`MapMapper` uses `.contains_key()` instead — an easy name to get wrong by
assuming symmetry across mapper types.

`tests/storage_mappers_blackbox_test.rs` — six tests, one
per mapper, each proving the specific behavioral claim in the table above
(not just "it compiles"): `VecMapper`'s first push really does land at
index 1; `SetMapper`/`UnorderedSetMapper`'s `contains()` really does
return `true`/`false` correctly for present/absent values;
`WhitelistMapper` really has no enumeration method (only checked
implicitly — the endpoint surface here is `contains()`-only by design,
matching the mapper's own "non-iterable" nature); `MapMapper`'s
`get_balance()` on a key that was never inserted returns `0`
(`unwrap_or_default()`), which is indistinguishable from a real zero
balance unless you also check `contains_key()` — a real design trap
worth testing explicitly, not just describing.

## Pitfalls

1. **`VecMapper` is 1-indexed. Index 0 is invalid and panics.** Confirmed
   directly: `vec_mapper_is_one_indexed` pushes two items and reads back
   index 1 as the first one pushed.

2. **`MapMapper.get()` returning a default-via-`unwrap_or_default()` value
   looks identical to a real stored zero.** If "never set" and "set to
   zero" need to be distinguishable in your contract, check
   `contains_key()` explicitly rather than trusting a default-valued read
   — `map_mapper_insert_get_and_contains_key` tests exactly this gap.

3. **`SetMapper` and `UnorderedSetMapper` both offer `contains()` at
   O(1), so the deciding factor between them is ordering and storage
   cost, not lookup speed.** Pick `UnorderedSetMapper` unless you
   specifically need insertion-order iteration or `next()`/`previous()`
   navigation — it's cheaper per element.

4. **`WhitelistMapper` cannot list its members at all**, not even
   inefficiently. If you might ever need to enumerate — even rarely, even
   just for an admin dashboard — use `SetMapper` or `UnorderedSetMapper`
   instead from the start; there's no way to add enumeration to a
   `WhitelistMapper` later without migrating storage.

5. **Two different mapper types can use different names for what looks
   like the same operation.** `SetMapper`/`UnorderedSetMapper.contains()`
   vs `MapMapper.contains_key()` — check the exact mapper's own method
   names rather than assuming consistency across the family (the same
   general caution CLAUDE.md's own pitfall #14 raises for sdk-core's
   Controller/Factory casing, here inside sdk-rs instead).

## What this recipe didn't test

`LinkedListMapper`, `QueueMapper`, `UserMapper`, `UniqueIdMapper`, and
`BiDiMapper` are in CLAUDE.md's table and are real, exported mapper
types (confirmed present in the crate's module listing), but this recipe
doesn't ship working code exercising them — six mappers with real,
tested endpoints was already substantial scope for one recipe, and the
first six cover the large majority of real contract storage needs per
this Cookbook's own reading of the example contracts in `mx-sdk-rs`.
`FungibleTokenMapper`/`NonFungibleTokenMapper`/`TokenAttributesMapper`
are deliberately out of scope here too — they need real ESDT system
contract interaction to demonstrate meaningfully, which belongs in a
token-issuance-from-a-contract recipe (Wave 3 per V1-RECIPE-LIST.md's
"What v1 explicitly defers"), not a storage-mapper comparison.

## See also

- [New contract from `sc-meta new --template empty`](/smart-contracts-rust/new-contract-from-template/) —
  the scaffolding this recipe's contract builds on, including the same
  `sc-config.toml` / proxy-generation ordering.
- CLAUDE.md §"Storage Mapper Efficiency" and §"Available Storage
  Mappers" — the summary this recipe verifies against real source and
  partially corrects.

## Validation

`cargo check` passed on the first attempt for every mapper's endpoint
code — every method name used was taken directly from the real crate
source's own doc-comment examples rather than guessed first and fixed
later. `sc-meta all build` produces a real WASM + ABI + `.mxsc.json`
(contract size 5,836 bytes). `cargo test` runs 8/8 passing: the 6
blackbox tests described above, plus the 2 scaffold-provided scenario
tests (unchanged from `sc-meta new --template empty`'s output). No
`npm`/`tsc`/`eslint` gate applies — this is Rust, verified with
`sc-meta`/`cargo` per this Cookbook's TypeScript-vs-Rust verification
split (see PROTOTYPE-NOTES.md's FINAL SUMMARY). `target/`/`output/` are
gitignored (regenerable); `Cargo.lock`/`wasm/Cargo.lock` are kept.
