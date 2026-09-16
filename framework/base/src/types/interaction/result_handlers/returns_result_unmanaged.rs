use multiversx_sc_codec::TopDecodeMulti;

use crate::types::{RHListItem, TxEnv};

use super::HasUnmanaged;

/// Indicates that the unmanaged version of the result will be returned.
pub struct ReturnsResultUnmanaged;

impl<Env, Original> RHListItem<Env, Original> for ReturnsResultUnmanaged
where
    Env: TxEnv,
    Original: HasUnmanaged,
    Original::Unmanaged: TopDecodeMulti,
{
    type Returns = Original::Unmanaged;
}
