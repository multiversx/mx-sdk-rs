use std::collections::HashMap;

use wasmparser::ValType;

use super::VM_HOOK_SIGNATURES;

#[derive(Debug, Clone)]
pub struct VmHookSignature {
    pub name: &'static str,
    pub params: &'static [ValType],
    pub returns: Option<ValType>,
}

impl VmHookSignature {
    pub const fn new(
        name: &'static str,
        params: &'static [ValType],
        returns: Option<ValType>,
    ) -> Self {
        Self {
            name,
            params,
            returns,
        }
    }
}

pub fn vm_hook_signature_map() -> HashMap<&'static str, VmHookSignature> {
    let mut map = HashMap::new();
    for vm_hook in VM_HOOK_SIGNATURES {
        map.insert(vm_hook.name, vm_hook.clone());
    }
    map
}

pub fn check_vm_hook_signatures(
    func_name: &str,
    params: &[ValType],
    returns: &[ValType],
    signature_map: &HashMap<&'static str, VmHookSignature>,
) {
    let signature = signature_map
        .get(func_name)
        .unwrap_or_else(|| panic!("unknown vm hook function: {func_name}"));

    assert_eq!(
        signature.params, params,
        "vm hook function {func_name} has invalid parameters"
    );
    match signature.returns {
        Some(ret) => {
            assert_eq!(
                returns.len(),
                1,
                "vm hook function {func_name} must have exactly one return value"
            );
            assert_eq!(
                returns[0], ret,
                "vm hook function {func_name} has invalid return type"
            );
        }
        None => {
            assert!(
                returns.is_empty(),
                "vm hook function {func_name} must not have return values"
            );
        }
    }
}
