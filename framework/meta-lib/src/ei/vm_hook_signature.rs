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

    pub fn to_wat_func_type_declaration(&self, comment: &str) -> String {
        let mut sc_wat = String::new();
        sc_wat.push_str("(type");
        sc_wat.push_str(comment);
        sc_wat.push_str(" (func");
        if !self.params.is_empty() {
            sc_wat.push_str(" (param");
            for param in self.params {
                sc_wat.push(' ');
                sc_wat.push_str(val_type_to_wat(*param));
            }
            sc_wat.push(')');
        }
        if let Some(ret) = self.returns {
            sc_wat.push_str(" (result ");
            sc_wat.push_str(val_type_to_wat(ret));
            sc_wat.push(')');
        }
        sc_wat.push_str("))");
        sc_wat
    }
}

fn val_type_to_wat(val_type: ValType) -> &'static str {
    match val_type {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        _ => panic!("unsupported return type in vm hook signature"),
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
