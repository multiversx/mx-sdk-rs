use elrond_wasm::{api::VMApi, elrond_codec::TryStaticCast};

/// The reference to the API implementation based on Arwen hooks.
/// It continas no data, can be embedded at no cost.
/// Cloning it is a no-op.
pub struct ArwenApiImpl {}

impl TryStaticCast for ArwenApiImpl {}

impl VMApi for ArwenApiImpl {}

/// Should be no-op. The API implementation is zero-sized.
impl Clone for ArwenApiImpl {
    #[inline]
    fn clone(&self) -> Self {
        ArwenApiImpl {}
    }
}
