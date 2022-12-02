use crate::{
    api::{
        const_handles, use_raw_handle, CallValueApiImpl, ManagedBufferApi, StorageWriteApiImpl,
        VMApi, EXTERNAL_VIEW_TARGET_ADRESS_KEY,
    },
    io::load_endpoint_args,
    types::ManagedType,
};

/// Implementation of external view contract constructors.
/// They take 1 Address argument and save it to storage under key `external-view-target-address`.
pub fn external_view_contract_constructor<A>()
where
    A: VMApi,
{
    A::call_value_api_impl().check_not_payable();
    let (target_contract_address, ()) = load_endpoint_args::<
        A,
        (crate::types::ManagedAddress<A>, ()),
    >(("target_contract_address", ()));
    let key_handle: A::ManagedBufferHandle = use_raw_handle(const_handles::MBUF_TEMPORARY_1);
    A::managed_type_impl().mb_overwrite(key_handle.clone(), EXTERNAL_VIEW_TARGET_ADRESS_KEY);
    A::storage_write_api_impl()
        .storage_store_managed_buffer_raw(key_handle, target_contract_address.get_handle());
}
