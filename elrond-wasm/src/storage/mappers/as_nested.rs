use elrond_codec::{TopDecode, TopEncode};

use crate::api::{ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};

use super::SingleValueMapper;

pub trait AsNested<SA, T> {
    type Nested;
}

impl<SA, T> AsNested<SA, T> for T
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    type Nested = SingleValueMapper<SA, T>;
}
