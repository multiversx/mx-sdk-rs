#![no_std]

//! Benchmarks decoding a large `ManagedVec` of structs from storage.
//!
//! Reproduces the storage pattern from mx-exchange-sc `token-unstake`
//! (`SingleValueMapper<ManagedVec<UnstakePair>>`).
//! Built twice (see `sc-config.toml`), with and without the `managed-buffer-preload` feature.

pub mod large_vec_storage_proxy;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MAX_CLAIM: usize = 20;

#[type_abi]
#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct UnstakePair<M: ManagedTypeApi> {
    pub unlock_epoch: u64,
    pub locked_tokens: Payment<M>,
    pub unlocked_tokens: Payment<M>,
}

#[multiversx_sc::contract]
pub trait LargeVecStorageBenchmark {
    #[init]
    fn init(&self) {}

    /// Baseline: endpoint call overhead, nothing else.
    #[endpoint]
    fn noop(&self) {}

    /// Appends `count` entries to storage, with indices starting at `start`.
    ///
    /// Encodes entries one by one and appends to the raw buffer,
    /// to avoid the (quadratic) cost of re-encoding a whole `ManagedVec`.
    /// Can be called repeatedly to build up large storage in chunks.
    #[endpoint]
    fn fill(&self, start: usize, count: usize) {
        let locked_id = TokenId::from("LKMEX-123456");
        let unlocked_id = TokenId::from("MEX-123456");
        let mut buffer = self.raw_entries().get();
        for i in start..start + count {
            let amount = BigUint::from(1_000_000_000_000_000_000u64 + i as u64);
            let half_amount = NonZeroBigUint::new_or_panic(&amount / 2u32);
            let entry = UnstakePair {
                unlock_epoch: i as u64,
                locked_tokens: Payment::new(
                    locked_id.clone(),
                    1000 + i as u64,
                    NonZeroBigUint::new_or_panic(amount),
                ),
                unlocked_tokens: Payment::new(unlocked_id.clone(), 0, half_amount),
            };
            entry
                .dep_encode(&mut buffer)
                .unwrap_or_else(|e| sc_panic!(e));
        }
        self.raw_entries().set(buffer);
    }

    #[endpoint]
    fn clear(&self) {
        self.raw_entries().clear();
    }

    /// Raw storage load only, no decoding.
    #[endpoint(loadRaw)]
    fn load_raw(&self) -> usize {
        self.raw_entries().get().len()
    }

    /// Framework decoder. Its cost depends on whether the contract is built with `managed-buffer-preload`.
    #[endpoint(decodeDefault)]
    fn decode_default(&self) -> usize {
        self.entries().get().len()
    }

    /// Decodes, then iterates once over the ManagedVec.
    /// Subtract `decodeDefault` to isolate iteration cost.
    #[endpoint(iterSum)]
    fn iter_sum(&self) -> BigUint {
        let entries = self.entries().get();
        let mut sum = BigUint::zero();
        for entry in &entries {
            sum += entry.unlocked_tokens.amount.as_big_uint();
        }
        sum
    }

    /// Decodes, then top-encodes into a ManagedBuffer (no storage write).
    /// Subtract `decodeDefault` to isolate encoding cost.
    #[endpoint(encodeOnly)]
    fn encode_only(&self) -> usize {
        let entries = self.entries().get();
        let mut buffer = ManagedBuffer::new();
        let _ = entries.top_encode(&mut buffer);
        buffer.len()
    }

    /// Mirrors `claimUnlockedTokens` storage access: decode, remove up to 20 from front, re-encode.
    #[endpoint(claimDefault)]
    fn claim_default(&self) -> usize {
        let mut count = 0;
        self.entries().update(|entries| {
            while !entries.is_empty() && count < MAX_CLAIM {
                entries.remove(0);
                count += 1;
            }
        });
        count
    }

    #[storage_mapper("entries")]
    fn entries(&self) -> SingleValueMapper<ManagedVec<UnstakePair<Self::Api>>>;

    #[storage_mapper("entries")]
    fn raw_entries(&self) -> SingleValueMapper<ManagedBuffer>;
}
