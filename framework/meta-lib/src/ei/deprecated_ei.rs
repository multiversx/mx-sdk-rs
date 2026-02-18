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
        "getArgument",
        "Arguments are now processed via `mBufferGetArgument`",
    ),
    DeprecatedVMHook::new(
        "getCallValue",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTValue",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTValueByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenName",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenNameByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenNonce",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenNonceByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenType",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getESDTTokenTypeByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getNumESDTTransfers",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getCallValueTokenName",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "getCallValueTokenNameByIndex",
        "Call value processing is now done via `managedGetAllTransfersCallValue`",
    ),
    DeprecatedVMHook::new(
        "writeEventLog",
        "Events are now logged via `managedWriteLog`",
    ),
    DeprecatedVMHook::new(
        "mBufferFromSmallIntSigned",
        "This method has a bug that converts negative numbers to their absolute values. Do not use until the bug is fixed on mainnet. It will be un-deprecated once the VM bug is resolved.",
    ),
];

pub(super) fn deprecated_vm_hooks_1_5(name: &str) -> Option<&'static DeprecatedVMHook> {
    DEPRECATED_VM_HOOKS_1_5
        .iter()
        .find(|hook| hook.name == name)
}
