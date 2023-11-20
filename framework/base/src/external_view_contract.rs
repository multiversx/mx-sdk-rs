use alloc::string::ToString;

use crate::{
    abi::{EndpointAbi, EndpointMutabilityAbi, EndpointTypeAbi, InputAbi, TypeAbi},
    api::{
        const_handles, use_raw_handle, CallValueApiImpl, ManagedBufferApiImpl, StorageWriteApiImpl,
        VMApi, EXTERNAL_VIEW_TARGET_ADRESS_KEY,
    },
    io::load_endpoint_args,
    types::ManagedType,
};

pub const EXTERNAL_VIEW_CONSTRUCTOR_FLAG: &str = "<external view init>";

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

/// The definition for the external view
pub fn external_view_contract_constructor_abi() -> EndpointAbi {
    let mut endpoint_abi = EndpointAbi::new(
        &[
            "The external view init prepares a contract that looks in another contract's storage.",
            "It takes a single argument, the other contract's address",
            "You won't find this constructors' definition in the contract, it gets injected automatically by the framework. See `multiversx_sc::external_view_contract`.",
            ],
        "init",
        EXTERNAL_VIEW_CONSTRUCTOR_FLAG,
        false,
        false,
        EndpointMutabilityAbi::Mutable,
        EndpointTypeAbi::Init,
        &[],
        &[],
        false
    );
    endpoint_abi.inputs.push(InputAbi {
        arg_name: "target_contract_address".to_string(),
        type_name: crate::types::heap::Address::type_name(),
        multi_arg: false,
    });
    endpoint_abi
}
