use multiversx_sc::api::{const_handles, RawHandle};

#[derive(Debug)]
pub struct TxStaticVars {
    pub external_view_target_address_handle: RawHandle,
    pub next_handle: RawHandle,
    pub num_arguments: i32,
    pub call_value_egld_handle: RawHandle,
    pub call_value_multi_esdt_handle: RawHandle,
    //vec of true/false, true if bit from handle = scaling_start + index is not empty
    pub scaling_factor_init: [bool; const_handles::SCALING_FACTOR_LENGTH],
}

impl Default for TxStaticVars {
    fn default() -> Self {
        TxStaticVars {
            external_view_target_address_handle: 0,
            next_handle: const_handles::NEW_HANDLE_START_FROM,
            num_arguments: -1,
            call_value_egld_handle: const_handles::UNINITIALIZED_HANDLE,
            call_value_multi_esdt_handle: const_handles::UNINITIALIZED_HANDLE,
            scaling_factor_init: [false; const_handles::SCALING_FACTOR_LENGTH],
        }
    }
}
