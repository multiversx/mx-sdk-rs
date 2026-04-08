use wasmparser::ValType;

use super::VmHookSignature;

/// Signatures for the parametric test API.
/// 
/// Unlike the other signatures, not auto-generated.
/// 
/// Adapt as needed, but make sure to keep in sync with the actual implementation in `framework/wasm-adapter/src/api/test_api_node.rs`.
#[rustfmt::skip]
pub const TEST_API_HOOK_SIGNATURES: &[VmHookSignature] = &[
    VmHookSignature::new("createAccount", &[ValType::I32, ValType::I64, ValType::I32], None),
    VmHookSignature::new("registerNewAddress", &[ValType::I32, ValType::I64, ValType::I32], None),
    VmHookSignature::new("deployContract", &[ValType::I32, ValType::I64, ValType::I32, ValType::I32, ValType::I32, ValType::I32], None),
    VmHookSignature::new("setStorage", &[ValType::I32, ValType::I32, ValType::I32], None),
    VmHookSignature::new("getStorage", &[ValType::I32, ValType::I32, ValType::I32], None),
    VmHookSignature::new("assumeBool", &[ValType::I32], None),
    VmHookSignature::new("assertBool", &[ValType::I32], None),
    VmHookSignature::new("startPrank", &[ValType::I32], None),
    VmHookSignature::new("stopPrank", &[], None),
    VmHookSignature::new("setBlockTimestamp", &[ValType::I64], None),
    VmHookSignature::new("setExternalBalance", &[ValType::I32, ValType::I32], None),
    VmHookSignature::new("setESDTExternalBalance", &[ValType::I32, ValType::I32, ValType::I32], None),
];
