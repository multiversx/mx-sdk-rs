use multiversx_sc::{derive_imports::*, types::Box};

const ARRAY_SIZE: usize = 512;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LargeBoxedByteArray(Box<[u8; ARRAY_SIZE]>);
