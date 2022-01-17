use crate::{
    api::{
        CallValueApiImpl, EndpointArgumentApiImpl, ManagedBufferApi, StorageWriteApiImpl, VMApi,
        EXTERNAL_VIEW_TARGET_ADRESS_KEY,
    },
    load_single_arg,
    types::ManagedType,
    ArgId,
};

/// Implementation of external view contract constructors.
/// They take 1 Address argument and save it to storage under key `external-view-target-address`.
pub fn external_view_contract_constructor<A>()
where
    A: VMApi,
{
    A::call_value_api_impl().check_not_payable();
    A::argument_api_impl().check_num_arguments(1);
    let target_contract_address = load_single_arg::<A, crate::types::ManagedAddress<A>>(
        0i32,
        ArgId::from(&b"target_contract_address"[..]),
    );
    let key_handle = A::managed_type_impl().mb_new_from_bytes(EXTERNAL_VIEW_TARGET_ADRESS_KEY);
    A::storage_write_api_impl()
        .storage_store_managed_buffer_raw(key_handle, target_contract_address.get_raw_handle());
}
