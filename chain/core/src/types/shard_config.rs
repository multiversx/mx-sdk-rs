use super::Address;

/// Identifies a shard by its numeric index.
///
/// Regular shards are numbered from `0` to `number_of_shards - 1`.
/// The special value [`METACHAIN_SHARD_ID`] (`u32::MAX`) identifies the metachain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardId(u32);

impl ShardId {
    /// Shard ID reserved for the metachain.
    pub const METACHAIN_ID: ShardId = ShardId(u32::MAX);

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for ShardId {
    fn from(value: u32) -> Self {
        ShardId(value)
    }
}

/// Precomputed configuration for shard assignment.
/// Mirrors the Go `multiShardCoordinator` struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShardConfig {
    /// Total number of regular shards (not counting the metachain).
    pub number_of_shards: u32,
    /// Bitmask using ceil(log2(number_of_shards)) bits.
    pub mask_high: u32,
    /// Bitmask using ceil(log2(number_of_shards)) - 1 bits (fallback).
    pub mask_low: u32,
}

impl ShardConfig {
    /// Pre-built configuration for the standard 3-shard setup used on MultiversX mainnet.
    pub const THREE_SHARDS: ShardConfig = ShardConfig::new(3).unwrap();

    /// Creates a new [`ShardConfig`] for the given number of regular shards.
    ///
    /// Returns `None` if `number_of_shards` is zero.
    pub const fn new(number_of_shards: u32) -> Option<Self> {
        if number_of_shards < 1 {
            return None;
        }
        let (mask_high, mask_low) = calculate_masks(number_of_shards);
        Some(ShardConfig {
            number_of_shards,
            mask_high,
            mask_low,
        })
    }

    /// Returns the number of trailing address bytes needed to cover all shard IDs.
    fn bytes_needed(&self) -> usize {
        if self.number_of_shards <= 256 {
            1
        } else if self.number_of_shards <= 65_536 {
            2
        } else if self.number_of_shards <= 16_777_216 {
            3
        } else {
            4
        }
    }

    /// Calculates the shard for a given address.
    /// Mirrors `ComputeIdFromBytes` in multiShardCoordinator.go.
    pub fn compute_id(&self, address: &Address) -> ShardId {
        if address.is_smart_contract_on_metachain() {
            return ShardId::METACHAIN_ID;
        }

        // address is always 32 bytes, which is larger than bytes_needed (max 4).
        let arr = address.as_array();
        let starting_index = arr.len() - self.bytes_needed();
        let buf_needed = &arr[starting_index..];

        // Interpret the trailing bytes as a big-endian u32.
        let mut val: u32 = 0;
        for &byte in buf_needed {
            val = (val << 8) + u32::from(byte);
        }

        // Try the high mask first; fall back to the low mask if out of range.
        let mut shard = val & self.mask_high;
        if shard > self.number_of_shards - 1 {
            shard = val & self.mask_low;
        }

        ShardId(shard)
    }

    /// Returns true if both addresses belong to the same shard.
    /// Mirrors `SameShard` in multiShardCoordinator.go.
    pub fn same_shard(&self, a: &Address, b: &Address) -> bool {
        if a.is_zero() || b.is_zero() {
            return true;
        }
        if a == b {
            return true;
        }
        self.compute_id(a) == self.compute_id(b)
    }
}

/// Computes the two bitmasks from the number of shards.
/// n = ceil(log2(number_of_shards))
/// mask_high = 2^n - 1
/// mask_low  = 2^(n-1) - 1
const fn calculate_masks(number_of_shards: u32) -> (u32, u32) {
    let n = u32::BITS - (number_of_shards - 1).leading_zeros();
    // Guard against n == 0 (number_of_shards == 1): n-1 would underflow.
    let mask_high = (1u32 << n).wrapping_sub(1);
    let mask_low = if n > 0 {
        (1u32 << (n - 1)).wrapping_sub(1)
    } else {
        0
    };
    (mask_high, mask_low)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors `getAddressFromUint32`: encodes `value` as big-endian u32
    /// in the last 4 bytes of a 32-byte address (left-padded with zeros).
    fn address_from_u32(value: u32) -> Address {
        let mut arr = [0u8; 32];
        arr[28..].copy_from_slice(&value.to_be_bytes());
        Address::new(arr)
    }

    fn address_from_hex(s: &str) -> Address {
        use alloc::vec::Vec;
        let bytes = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();
        Address::new(bytes.try_into().unwrap())
    }

    // --- constructor tests ---

    #[test]
    fn test_new_multi_shard_config() {
        let num_of_shards = 10u32;
        let sr = ShardConfig::new(num_of_shards).unwrap();
        assert_eq!(sr.number_of_shards, num_of_shards);
        let (expected_high, expected_low) = calculate_masks(num_of_shards);
        assert_eq!(sr.mask_high, expected_high);
        assert_eq!(sr.mask_low, expected_low);
    }

    #[test]
    fn test_new_invalid_number_of_shards() {
        let result = ShardConfig::new(0);
        assert!(result.is_none());
    }

    // --- compute_id tests ---

    #[test]
    fn test_compute_id_does_not_generate_invalid_shards() {
        let num_of_shards = 10u32;
        let sr = ShardConfig::new(num_of_shards).unwrap();
        for i in 0u32..200 {
            let addr = address_from_u32(i);
            let shard_id = sr.compute_id(&addr);
            assert!(
                shard_id.0 < sr.number_of_shards,
                "i={i}: shard {} >= {num_of_shards}",
                shard_id.0
            );
        }
    }

    #[test]
    fn test_compute_id_10_shards() {
        let sr = ShardConfig::new(10).unwrap();
        let dataset: &[(u32, u32)] = &[
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 2),
            (11, 3),
            (12, 4),
            (13, 5),
            (14, 6),
            (15, 7),
        ];
        for &(address, expected_shard) in dataset {
            let addr = address_from_u32(address);
            assert_eq!(
                sr.compute_id(&addr),
                ShardId(expected_shard),
                "address={address}"
            );
        }
    }

    #[test]
    fn test_compute_id_10_shards_big_numbers() {
        let sr = ShardConfig::new(10).unwrap();
        let dataset: &[(&str, u32)] = &[
            (
                "2ca2ed0a1c77b5ddeabf99e3f17074f1c77b5ddea37dbaacf501ef1752950c50",
                0,
            ),
            (
                "5f7e73b922883bf97b5ddeab9e3f103b8ddea37dbaaca24b5ddea37dbaac1061",
                1,
            ),
            (
                "65b9926097345bf7b5ddeab99e3f1cc7c71c01bfd3e1efacc1d0df1ba8f96172",
                2,
            ),
            (
                "c1c77b5ddea5c71c4160c861c01ba8f9617a65010d2baac7827a501bbf29aad3",
                3,
            ),
            (
                "22c2e1facc1d1c77b5d16160c861c01ba8f96170c86783b8deabaac782733b84",
                4,
            ),
            (
                "4cc88bdac668dc1878271e79a67b5ddeaddea37dbaacbbf99e3f1f4e5a4d7085",
                5,
            ),
            (
                "b533facc1daa3a617466f4160c861c01ba8f9617a65010d160c86783b82a5836",
                6,
            ),
            (
                "b1487283ad280316baa160c861c01ba8f9617c78270c8668dc18b5ddea37dba7",
                7,
            ),
            (
                "acfba138faed1c7b5d668dc1878b5ddea37dbaac271edeab7160c861c01ba8f8",
                8,
            ),
            (
                "cc3757647aebf9d160c86e9f9eb5dde0c86783b89160a37dbaac3f15c8f48999",
                9,
            ),
            (
                "1a30b33104a94a65010d2a5e87285eb0ea37dbaac60c86c3facc1d4d1ba8f96a",
                2,
            ),
            (
                "fcca8da9ba5160c86783b89160ea37dbaac60c86c86783b8c868be1ba8f9617b",
                3,
            ),
            (
                "8f9b094668dc1878271ed1b1b5ddea37dbaac60c86760f2e4c71c01bf6a913cc",
                4,
            ),
            (
                "a2d768be59a607d160c86eb5ddea37dbaafacc1dc9fa0e0c86783b8916092cbd",
                5,
            ),
            (
                "365865f21b2e0d668dc18ea37dbaac60c8678271e160c86e9160c86783b8fe6e",
                6,
            ),
            (
                "16cc745884a65ba160c861c01ba8f9617ac7827d160c86e9f010d2a592b3a52f",
                7,
            ),
        ];
        for &(hex, expected_shard) in dataset {
            let addr = address_from_hex(hex);
            assert_eq!(sr.compute_id(&addr), ShardId(expected_shard), "hex={hex}");
        }
    }

    #[test]
    fn test_compute_id_same_suffix_has_same_shard() {
        let sr = ShardConfig::new(2).unwrap();
        let dataset: &[(u32, u32)] = &[
            (0, 0),
            (1, 1),
            (2, 0),
            (3, 1),
            (4, 0),
            (5, 1),
            (6, 0),
            (7, 1),
            (8, 0),
            (9, 1),
        ];
        for &(address, expected_shard) in dataset {
            let addr = address_from_u32(address);
            assert_eq!(
                sr.compute_id(&addr),
                ShardId(expected_shard),
                "address={address}"
            );
        }
    }

    // --- same_shard tests ---

    #[test]
    fn test_same_shard_same_address() {
        let shard = ShardConfig::new(1).unwrap();
        let addr1 = address_from_u32(1);
        let addr2 = address_from_u32(1);
        assert!(shard.same_shard(&addr1, &addr2));
    }

    #[test]
    fn test_same_shard_same_address_multiple_shards() {
        let shard = ShardConfig::new(11).unwrap();
        let addr1 = address_from_u32(1);
        let addr2 = address_from_u32(1);
        assert!(shard.same_shard(&addr1, &addr2));
    }

    #[test]
    fn test_same_shard_different_address_one_shard() {
        let shard = ShardConfig::new(1).unwrap();
        let addr1 = address_from_u32(1);
        let addr2 = address_from_u32(2);
        assert!(shard.same_shard(&addr1, &addr2));
    }

    #[test]
    fn test_same_shard_different_address_multiple_shards() {
        let shard = ShardConfig::new(2).unwrap();
        let addr1 = address_from_u32(1);
        let addr2 = address_from_u32(2);
        assert!(!shard.same_shard(&addr1, &addr2));
    }

    #[test]
    fn test_same_shard_contract_deploy() {
        let shard = ShardConfig::new(2).unwrap();
        let addr1 = Address::zero(); // empty address
        let addr2 = Address::new([1u8; 32]); // non-empty address
        assert!(shard.same_shard(&addr1, &addr2));
    }
}
