#![no_std]

//! Benchmarks decoding a large `ManagedVec` of structs from storage.
//!
//! Reproduces the storage pattern from mx-exchange-sc `token-unstake`
//! (`SingleValueMapper<ManagedVec<UnstakePair>>`), and compares the default framework
//! decoder against prototype decoders that preload the encoded bytes into wasm memory.

pub mod large_vec_storage_proxy;
pub mod preloaded_input;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::codec::NestedDecodeInput;
use preloaded_input::{PreloadedNestedDecodeInput, preload_bytes};

pub const MAX_CLAIM: usize = 20;

#[type_abi]
#[derive(
    ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct UnstakePair<M: ManagedTypeApi> {
    pub unlock_epoch: u64,
    pub locked_tokens: EsdtTokenPayment<M>,
    pub unlocked_tokens: EsdtTokenPayment<M>,
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
        let locked_id = EsdtTokenIdentifier::from("LKMEX-123456");
        let unlocked_id = EsdtTokenIdentifier::from("MEX-123456");
        let mut buffer = self.raw_entries().get();
        for i in start..start + count {
            let amount = BigUint::from(1_000_000_000_000_000_000u64 + i as u64);
            let entry = UnstakePair {
                unlock_epoch: i as u64,
                locked_tokens: EsdtTokenPayment::new(
                    locked_id.clone(),
                    1000 + i as u64,
                    amount.clone(),
                ),
                unlocked_tokens: EsdtTokenPayment::new(unlocked_id.clone(), 0, amount / 2u32),
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

    /// Current framework decoder (one VM call per field).
    #[endpoint(decodeDefault)]
    fn decode_default(&self) -> usize {
        self.entries().get().len()
    }

    /// Mimics the pre-#1964 behaviour: fixed-size fields from wasm memory,
    /// nested managed buffers via `mBufferCopyByteSlice`.
    #[endpoint(decodePreloadedCopy)]
    fn decode_preloaded_copy(&self) -> usize {
        self.decode_preloaded::<false>().len()
    }

    /// Everything from wasm memory, nested managed buffers/big ints created from bytes.
    #[endpoint(decodePreloadedBytes)]
    fn decode_preloaded_bytes(&self) -> usize {
        self.decode_preloaded::<true>().len()
    }

    /// Cheap decode, then iterates once over the ManagedVec.
    /// Subtract `decodePreloadedBytes` to isolate iteration cost.
    #[endpoint(iterSum)]
    fn iter_sum(&self) -> BigUint {
        let entries = self.decode_preloaded::<true>();
        let mut sum = BigUint::zero();
        for entry in &entries {
            sum += &entry.unlocked_tokens.amount;
        }
        sum
    }

    /// Cheap decode, then top-encodes into a ManagedBuffer (no storage write).
    /// Subtract `decodePreloadedBytes` to isolate encoding cost.
    #[endpoint(encodeOnly)]
    fn encode_only(&self) -> usize {
        let entries = self.decode_preloaded::<true>();
        let mut buffer = ManagedBuffer::new();
        let _ = entries.top_encode(&mut buffer);
        buffer.len()
    }

    fn decode_preloaded<const FROM_BYTES: bool>(&self) -> ManagedVec<UnstakePair<Self::Api>> {
        let raw = self.raw_entries().get();
        let bytes = preload_bytes(&raw);
        let mut input = PreloadedNestedDecodeInput::<Self::Api, FROM_BYTES>::new(&raw, bytes);
        let mut result = ManagedVec::new();
        while input.remaining_len() > 0 {
            result.push(UnstakePair::dep_decode(&mut input).unwrap_or_else(|e| sc_panic!(e)));
        }
        result
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
