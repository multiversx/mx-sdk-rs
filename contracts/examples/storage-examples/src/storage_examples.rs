#![no_std]

use multiversx_sc::imports::*;

pub mod proxy;

/// Demonstrates the six most commonly used storage mapper types in
/// `multiversx-sc`, one endpoint group per mapper.  Each group exposes a
/// minimal but realistic API so the companion blackbox tests can verify the
/// behavioural guarantees documented in the mapper source.
///
/// | Mapper               | Iteration  | Duplicates | `contains` |  Storage entries  |
/// |----------------------|------------|------------|------------|-------------------|
/// | `SingleValueMapper`  | —          | one slot   | —          | 1                 |
/// | `VecMapper`          | sequential | allowed    | O(n)       | N + 1             |
/// | `SetMapper`          | ordered    | rejected   | O(1)       | ~3N + 1           |
/// | `UnorderedSetMapper` | unordered  | rejected   | O(1)       | 2N + 1            |
/// | `WhitelistMapper`    | —          | rejected   | O(1)       | N                 |
/// | `MapMapper`          | keys only  | —          | O(1)       | 4N + 1            |
#[multiversx_sc::contract]
pub trait StorageExamples {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // ---- SingleValueMapper -----------------------------------------------
    // Stores exactly one value under a fixed key.  The simplest mapper:
    // one storage entry, no iteration, O(1) read/write.
    // `set` overwrites any previous value; `get` returns the default if
    // the slot has never been written.
    #[endpoint(setCounter)]
    fn set_counter(&self, value: BigUint) {
        self.counter().set(value);
    }

    #[view(getCounter)]
    #[storage_mapper("counter")]
    fn counter(&self) -> SingleValueMapper<BigUint>;

    // ---- VecMapper ---------------------------------------------------------
    // An ordered, 1-indexed sequence that allows duplicate values.
    // `push` appends to the end and returns the new item's 1-based index.
    // Random access by index is O(1); `contains` is O(n) (linear scan).
    #[endpoint(pushItem)]
    fn push_item(&self, item: ManagedBuffer) -> usize {
        self.items().push(&item)
    }

    #[view(getItem)]
    fn get_item(&self, index: usize) -> ManagedBuffer {
        self.items().get(index)
    }

    #[view(itemCount)]
    fn item_count(&self) -> usize {
        self.items().len()
    }

    #[storage_mapper("items")]
    fn items(&self) -> VecMapper<ManagedBuffer>;

    // ---- SetMapper --------------------------------------------------------
    // An ordered set (insertion order) that rejects duplicates.
    // Internally maintains a doubly-linked list of nodes plus a
    // value→node-id index, giving O(1) `contains`, `insert`, and `remove`.
    // `insert` returns `true` when the value was newly added, `false` if
    // it was already present.  Uses ~3N + 1 storage entries for N elements.
    #[endpoint(addToOrderedSet)]
    fn add_to_ordered_set(&self, value: u64) -> bool {
        self.ordered_set().insert(value)
    }

    #[view(orderedSetContains)]
    fn ordered_set_contains(&self, value: u64) -> bool {
        self.ordered_set().contains(&value)
    }

    #[view(orderedSetLen)]
    fn ordered_set_len(&self) -> usize {
        self.ordered_set().len()
    }

    #[storage_mapper("ordered_set")]
    fn ordered_set(&self) -> SetMapper<u64>;

    // ---- UnorderedSetMapper -----------------------------------------------
    // A set with no ordering guarantee.  Backed by a `VecMapper` plus a
    // reverse value→index map, giving O(1) `contains`, `insert`, and
    // `remove` (swap-remove strategy) at the cost of ~2N + 1 storage
    // entries.  Cheaper than `SetMapper` when iteration order does not
    // matter.  `insert` returns `true` for a new value, `false` for a
    // duplicate.
    #[endpoint(addToUnorderedSet)]
    fn add_to_unordered_set(&self, value: u64) -> bool {
        self.unordered_set().insert(value)
    }

    #[view(unorderedSetContains)]
    fn unordered_set_contains(&self, value: u64) -> bool {
        self.unordered_set().contains(&value)
    }

    #[view(unorderedSetLen)]
    fn unordered_set_len(&self) -> usize {
        self.unordered_set().len()
    }

    #[storage_mapper("unordered_set")]
    fn unordered_set(&self) -> UnorderedSetMapper<u64>;

    // ---- WhitelistMapper --------------------------------------------------
    // Membership-only set: supports `add`, `remove`, and `contains` but
    // provides no iteration.  Each member costs exactly one storage entry
    // (a boolean flag), making it the most space-efficient option when
    // you only need to answer "is this address a member?".
    #[endpoint(addToWhitelist)]
    fn add_to_whitelist(&self, address: ManagedAddress) {
        self.whitelist().add(&address);
    }

    #[view(isWhitelisted)]
    fn is_whitelisted(&self, address: ManagedAddress) -> bool {
        self.whitelist().contains(&address)
    }

    #[storage_mapper("whitelist")]
    fn whitelist(&self) -> WhitelistMapper<ManagedAddress>;

    // ---- MapMapper --------------------------------------------------------
    // Key-value store with iterable keys.  Uses a `SetMapper` internally
    // to track the key set plus one extra slot per value, totalling ~4N + 1
    // storage entries for N pairs.  `insert` and `get` are O(1); iterating
    // all keys is O(N).  `get` returns `Option<V>` — use `unwrap_or_default`
    // when a missing key should be treated as a zero/empty value.
    #[endpoint(setBalance)]
    fn set_balance(&self, address: ManagedAddress, amount: BigUint) {
        self.balances().insert(address, amount);
    }

    #[view(getBalance)]
    fn get_balance(&self, address: ManagedAddress) -> BigUint {
        self.balances().get(&address).unwrap_or_default()
    }

    #[view(hasBalanceEntry)]
    fn has_balance_entry(&self, address: ManagedAddress) -> bool {
        self.balances().contains_key(&address)
    }

    #[storage_mapper("balances")]
    fn balances(&self) -> MapMapper<ManagedAddress, BigUint>;
}
