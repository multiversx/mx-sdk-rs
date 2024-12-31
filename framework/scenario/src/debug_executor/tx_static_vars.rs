use multiversx_sc::api::{const_handles, RawHandle, StaticVarApiFlags};

#[derive(Debug)]
pub struct TxStaticVars {
    pub external_view_target_address_handle: RawHandle,
    pub next_handle: RawHandle,
    pub num_arguments: i32,
    pub flags: StaticVarApiFlags,

    /// Vec of true/false, true if bit from handle = scaling_start + index is not empty
    pub scaling_factor_init: [bool; const_handles::SCALING_FACTOR_LENGTH],
}

impl Default for TxStaticVars {
    fn default() -> Self {
        TxStaticVars {
            external_view_target_address_handle: 0,
            next_handle: const_handles::NEW_HANDLE_START_FROM,
            num_arguments: -1,
            scaling_factor_init: [false; const_handles::SCALING_FACTOR_LENGTH],
            flags: StaticVarApiFlags::NONE,
        }
    }
}
