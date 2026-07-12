#![no_std]

use multiversx_sc::imports::*;

pub mod storage_examples_proxy;

/// One endpoint pair per mapper type this recipe covers with working,
/// tested code. Method names and complexity claims below are taken
/// directly from the real `multiversx-sc` crate source doc comments
/// (`~/.cargo/registry/.../multiversx-sc-0.64.2/src/storage/mappers/*.rs`,
/// read while authoring this recipe) — several correct CLAUDE.md's own
/// summary table; see this recipe's README for exactly which claims and
/// why.
#[multiversx_sc::contract]
pub trait StorageMappers {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // ---- SingleValueMapper: one value, no parameters. Simplest mapper,
    //      baseline for storage cost (1 entry). ----
    #[endpoint(setCounter)]
    fn set_counter(&self, value: BigUint) {
        self.counter().set(value);
    }

    #[view(getCounter)]
    #[storage_mapper("counter")]
    fn counter(&self) -> SingleValueMapper<BigUint>;

    // ---- VecMapper: ordered, 1-indexed, allows duplicates, random
    //      access by index. ----
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

    // ---- SetMapper: ordered (insertion order) set. Real source confirms
    //      O(1) contains via an internal value->node_id lookup, CONTRA
    //      CLAUDE.md's table, which claims O(n) — see README. ----
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

    // ---- UnorderedSetMapper: no ordering guarantee, O(1) contains via
    //      VecMapper + a reverse index lookup (2N+1 entries total — see
    //      README for why this differs from CLAUDE.md's "N+1"). ----
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

    // ---- WhitelistMapper: membership-only, no iteration, most
    //      space-efficient of the set-shaped mappers. ----
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

    // ---- MapMapper: key-value with iteration, HashMap-like API. Uses a
    //      SetMapper internally for key tracking plus separate value
    //      storage — real source confirms this is where CLAUDE.md's
    //      "4N+1 entries" claim actually checks out. ----
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
