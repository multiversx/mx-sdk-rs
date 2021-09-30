use elrond_wasm::{api::VMApi, elrond_codec::TryStaticCast};

use crate::TxContext;

impl TryStaticCast for TxContext {}

impl VMApi for TxContext {}
