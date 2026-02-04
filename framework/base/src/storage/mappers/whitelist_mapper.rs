use core::marker::PhantomData;

use multiversx_sc_codec::{TopDecode, TopEncode};

use super::{
    SingleValueMapper, StorageMapper, StorageMapperFromAddress,
    source::{CurrentStorage, StorageAddress},
};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    codec::NestedEncode,
    storage::StorageKey,
    types::ManagedAddress,
};

type FlagMapper<SA, A> = SingleValueMapper<SA, bool, A>;

const ITEM_NOT_WHITELISTED_ERR_MSG: &str = "Item not whitelisted";

/// A non-iterable whitelist mapper optimized for fast membership testing.
///
/// # Storage Layout
///
/// The `WhitelistMapper` uses a simple direct key approach with boolean flags:
///
/// - `base_key + encoded_item` → `true` (if whitelisted) or empty (if not whitelisted)
///
/// Each whitelisted item requires exactly one storage key, making this extremely space-efficient.
/// Uses `SingleValueMapper<bool>` internally for each item.
///
/// # Main Operations
///
/// - **Add**: `add(item)` - Adds item to whitelist. O(1) with one storage write.
/// - **Remove**: `remove(item)` - Removes item from whitelist. O(1) with storage clear.
/// - **Check**: `contains(item)` - Tests membership. O(1) with one storage read.
/// - **Require**: `require_whitelisted(item)` - Checks membership or errors. O(1).
///
/// # Key Characteristics
///
/// - **Non-iterable**: Cannot iterate over whitelisted items (use `SetMapper` if iteration needed)
/// - **Space-efficient**: One storage key per item, minimal overhead
/// - **Fast lookups**: Direct key-based membership testing
/// - **Boolean logic**: Uses presence/absence, since true = 1, false = empty
///
/// # Trade-offs
///
/// - **Pros**: Extremely efficient for membership testing; minimal storage overhead; simple and fast.
/// - **Cons**: No iteration capability; no built-in counting; cannot list all whitelisted items;
///   no bulk operations.
///
/// # Comparison with SetMapper/UnorderedSetMapper
///
/// - **WhitelistMapper**: Non-iterable; most space-efficient; fastest lookups
/// - **SetMapper**: Iterable; maintains insertion order; higher overhead
/// - **UnorderedSetMapper**: Iterable; no order guarantees; moderate overhead
///
/// # Use Cases
///
/// - Simple whitelists where iteration is not needed
/// - Permission systems (allowed addresses, tokens, etc.)
/// - Feature flags or capability checking
/// - Any membership testing where space efficiency is critical
/// - Large whitelists where iteration would be prohibitively expensive
///
/// # Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, WhitelistMapper};
/// # use multiversx_sc::types::ManagedAddress;
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>(
/// #     admin: ManagedAddress<SA>,
/// #     user1: ManagedAddress<SA>,
/// #     user2: ManagedAddress<SA>
/// # ) {
/// # let whitelist = WhitelistMapper::<SA, ManagedAddress<SA>>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"allowed_users"[..])
/// # );
/// // Add addresses to whitelist
/// whitelist.add(&admin);
/// whitelist.add(&user1);
///
/// // Check membership
/// assert!(whitelist.contains(&admin));
/// assert!(whitelist.contains(&user1));
/// assert!(!whitelist.contains(&user2));
///
/// // Require membership (errors if not whitelisted)
/// whitelist.require_whitelisted(&admin);  // OK
/// // whitelist.require_whitelisted(&user2);  // Would error: "Item not whitelisted"
///
/// // Remove from whitelist
/// whitelist.remove(&user1);
/// assert!(!whitelist.contains(&user1));
///
/// // Use in access control
/// fn admin_only_function<SA: multiversx_sc::api::StorageMapperApi>(
///     whitelist: &WhitelistMapper<SA, ManagedAddress<SA>>,
///     caller: &ManagedAddress<SA>
/// ) {
///     whitelist.require_whitelisted(caller);
///     // Function logic here...
/// }
/// # }
/// ```
///
/// # Token Whitelist Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, WhitelistMapper};
/// # use multiversx_sc::types::TokenIdentifier;
/// # fn token_example<SA: multiversx_sc::api::StorageMapperApi>(
/// #     token1: TokenIdentifier<SA>,
/// #     token2: TokenIdentifier<SA>
/// # ) {
/// # let allowed_tokens = WhitelistMapper::<SA, TokenIdentifier<SA>>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"allowed_tokens"[..])
/// # );
/// // Whitelist specific tokens
/// allowed_tokens.add(&token1);
///
/// // Validate token in payment
/// if allowed_tokens.contains(&token1) {
///     // Process payment
/// } else {
///     // Reject payment
/// }
/// # }
/// ```
pub struct WhitelistMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: NestedEncode + 'static,
{
    address: A,
    base_key: StorageKey<SA>,
    _phantom: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for WhitelistMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            address: CurrentStorage,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> StorageMapperFromAddress<SA> for WhitelistMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        Self {
            address,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> WhitelistMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopDecode + TopEncode + NestedEncode + 'static,
{
    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG.as_bytes());
        }
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA, ManagedAddress<SA>> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA, ManagedAddress<SA>>::new_from_address(self.address.clone(), key)
    }
}

impl<SA, T> WhitelistMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopDecode + TopEncode + NestedEncode + 'static,
{
    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG.as_bytes());
        }
    }

    pub fn add(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.set(true);
    }

    pub fn remove(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.clear();
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA, CurrentStorage> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA, CurrentStorage>::new(key)
    }
}
