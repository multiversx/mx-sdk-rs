/// Identifies a shard by its numeric index.
///
/// Regular shards are numbered from `0` to `number_of_shards - 1`.
/// The special value [`ShardId::METACHAIN_ID`] (`u32::MAX`) identifies the metachain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
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

impl core::fmt::Display for ShardId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if *self == ShardId::METACHAIN_ID {
            write!(f, "metachain")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
