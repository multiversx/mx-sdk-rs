use elrond_wasm::{
    api::{CallTypeApi, VMApi},
    elrond_codec::TryStaticCast,
};

use crate::DebugApi;

impl TryStaticCast for DebugApi {}

impl CallTypeApi for DebugApi {}

impl VMApi for DebugApi {}
