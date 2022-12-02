use elrond_wasm::types::{SCResult, StaticSCError};
use elrond_wasm_debug::*;

use alloc_features::macro_features_legacy::MacroFeaturesLegacy;

/// Likely to be removed soon.
#[test]
fn test_sc_error() {
    let _ = DebugApi::dummy();
    let bf = alloc_features::contract_obj::<DebugApi>();
    let result = bf.return_sc_error();
    assert_eq!(
        SCResult::Err(StaticSCError::from(&b"return_sc_error"[..])),
        result
    );
}
