use crate::model::{AutoImpl, Method, MethodImpl};

use super::attributes::*;

fn assert_no_other_auto_impl(method: &Method) {
    assert!(
		method.implementation.is_no_implementation(),
		"Only one auto-implementation can be specified at one time. Auto-implementations are: {}{}{}{}{}{}{}{}",
		"`#[storage_get]`, ",
		"`#[storage_set]`, ",
		"`#[storage_mapper]`, ",
		"`#[storage_is_empty]`, ",
		"`#[storage_clear]`, ",
		"`#[proxy]`, ",
		"`#[module]`, ",
		"`#[event]`."
	)
}

pub fn process_event_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    EventAttribute::parse(attr)
        .map(|event_attr| {
            assert_no_other_auto_impl(&*method);
            let event_identifier = event_attr.identifier;
            method.implementation = MethodImpl::Generated(AutoImpl::Event {
                identifier: event_identifier,
            });
        })
        .is_some()
}

pub fn process_proxy_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    if is_proxy(attr) {
        assert_no_other_auto_impl(&*method);
        method.implementation = MethodImpl::Generated(AutoImpl::ProxyGetter);
        true
    } else {
        false
    }
}

pub fn process_storage_get_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    StorageGetAttribute::parse(attr)
        .map(|storage_get| {
            assert_no_other_auto_impl(&*method);
            method.implementation = MethodImpl::Generated(AutoImpl::StorageGetter {
                identifier: storage_get.identifier,
            });
        })
        .is_some()
}

pub fn process_storage_set_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    StorageSetAttribute::parse(attr)
        .map(|storage_set| {
            assert_no_other_auto_impl(&*method);
            method.implementation = MethodImpl::Generated(AutoImpl::StorageSetter {
                identifier: storage_set.identifier,
            });
        })
        .is_some()
}

pub fn process_storage_mapper_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    StorageMapperAttribute::parse(attr)
        .map(|storage_mapper| {
            assert_no_other_auto_impl(&*method);
            method.implementation = MethodImpl::Generated(AutoImpl::StorageMapper {
                identifier: storage_mapper.identifier,
            });
        })
        .is_some()
}

pub fn process_storage_is_empty_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    StorageIsEmptyAttribute::parse(attr)
        .map(|storage_is_empty| {
            assert_no_other_auto_impl(&*method);
            method.implementation = MethodImpl::Generated(AutoImpl::StorageIsEmpty {
                identifier: storage_is_empty.identifier,
            });
        })
        .is_some()
}

pub fn process_storage_clear_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    StorageClearAttribute::parse(attr)
        .map(|storage_clear| {
            assert_no_other_auto_impl(&*method);
            method.implementation = MethodImpl::Generated(AutoImpl::StorageClear {
                identifier: storage_clear.identifier,
            });
        })
        .is_some()
}
