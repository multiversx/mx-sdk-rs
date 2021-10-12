use elrond_wasm::{api::VMApi, elrond_codec::TryStaticCast};

use crate::tx_mock::TxContext;

impl TryStaticCast for TxContext {}

impl VMApi for TxContext {}
