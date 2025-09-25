pub struct DeprecatedVMHook {
    pub name: &'static str,
    pub note: &'static str,
}

impl DeprecatedVMHook {
    pub const fn new(name: &'static str, note: &'static str) -> Self {
        Self { name, note }
    }
}

pub const DEPRECATED_VM_HOOKS_1_5: &[DeprecatedVMHook] = &[
    DeprecatedVMHook::new(
        "getBlockTimestamp",
        "Performs poorly in Supernova due to sub-second blocks. Use the millisecond version.",
    ),
    DeprecatedVMHook::new(
        "getPrevBlockTimestamp",
        "Performs poorly in Supernova due to sub-second blocks. Use the millisecond version.",
    ),
];

pub(super) fn deprecated_vm_hooks_1_5(name: &str) -> Option<&'static DeprecatedVMHook> {
    DEPRECATED_VM_HOOKS_1_5
        .iter()
        .find(|hook| hook.name == name)
}
