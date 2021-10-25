use crate::model::{AutoImpl, ContractTrait, Method, MethodImpl};

use super::{
    auto_impl_event::{generate_event_impl, generate_legacy_event_impl},
    auto_impl_proxy::generate_proxy_getter_impl,
    auto_impl_storage::{
        generate_clear_impl, generate_getter_impl, generate_is_empty_impl, generate_mapper_impl,
        generate_setter_impl,
    },
};

/// Implementations for methods that get auto-generated implementations: events, getters, setters
pub fn generate_auto_impls(contract: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract
        .methods
        .iter()
        .filter_map(|m| match &m.implementation {
            MethodImpl::Explicit(_) => None,
            MethodImpl::Generated(auto_impl) => Some(generate_auto_impl(m, auto_impl)),
            MethodImpl::NoImplementation => {
                panic!(
                    "method `{}` needs either an auto-implementation or a default implementation",
                    m.name
                )
            },
        })
        .collect()
}

fn generate_auto_impl(m: &Method, auto_impl: &AutoImpl) -> proc_macro2::TokenStream {
    match auto_impl {
        AutoImpl::LegacyEvent { identifier } => {
            generate_legacy_event_impl(m, identifier.as_slice())
        },
        AutoImpl::Event { identifier } => generate_event_impl(m, identifier),
        AutoImpl::StorageGetter { identifier } => generate_getter_impl(m, identifier),
        AutoImpl::StorageSetter { identifier } => generate_setter_impl(m, identifier),
        AutoImpl::StorageMapper { identifier } => generate_mapper_impl(m, identifier),
        AutoImpl::StorageIsEmpty { identifier } => generate_is_empty_impl(m, identifier),
        AutoImpl::StorageClear { identifier } => generate_clear_impl(m, identifier),
        AutoImpl::ProxyGetter => generate_proxy_getter_impl(m),
    }
}
