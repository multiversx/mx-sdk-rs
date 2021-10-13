use elrond_wasm::{api::VMApi, elrond_codec::TryStaticCast};

use crate::DebugApi;

impl TryStaticCast for DebugApi {}

impl VMApi for DebugApi {}
