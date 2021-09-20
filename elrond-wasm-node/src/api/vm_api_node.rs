use crate::ArwenApiImpl;
use elrond_wasm::{api::VMApi, elrond_codec::TryStaticCast};

impl TryStaticCast for ArwenApiImpl {}

impl VMApi for ArwenApiImpl {}
