use crate::model::{AutoImpl, MethodImpl};

use super::attributes::*;

pub fn process_method_impl(m: &syn::TraitItemMethod) -> MethodImpl {
	if let Some(auto_impl) = extract_auto_impl(m) {
		assert!(
			m.default.is_none(),
			"method cannot have both an auto-implementation and a default implementation"
		);
		MethodImpl::Generated(auto_impl)
	} else if let Some(body) = m.default.clone() {
		MethodImpl::Explicit(body)
	} else {
		MethodImpl::NoImplementation
	}
}

fn assert_no_other_auto_impl(auto_impl: &Option<AutoImpl>) {
	assert!(
		auto_impl.is_none(),
		"Only one auto-implementation can be specified at one time. Auto-implementations are: {}{}{}{}{}{}{}{}{}",
		"`#[storage_get]`, ",
		"`#[storage_set]`, ",
		"`#[storage_mapper]`, ",
		"`#[storage_is_empty]`, ",
		"`#[storage_clear]`, ",
		"`#[proxy]`, ",
		"`#[module]`, ",
		"`#[event]`, ",
		"`#[legacy-event]`."
	)
}

fn extract_auto_impl(m: &syn::TraitItemMethod) -> Option<AutoImpl> {
	let legacy_event_opt = LegacyEventAttribute::parse(m);
	let event_opt = EventAttribute::parse(m);
	let storage_get_opt = StorageGetAttribute::parse(m);
	let storage_set_opt = StorageSetAttribute::parse(m);
	let storage_mapper_opt = StorageMapperAttribute::parse(m);
	let storage_is_empty_opt = StorageIsEmptyAttribute::parse(m);
	let storage_clear_opt = StorageClearAttribute::parse(m);
	let is_proxy = is_proxy(m);
	let module_opt = ModuleAttribute::parse(m);

	let mut result = None;

	if let Some(event_attr) = legacy_event_opt {
		result = Some(AutoImpl::LegacyEvent {
			identifier: event_attr.identifier,
		})
	}

	if let Some(event_attr) = event_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::Event {
			identifier: event_attr.identifier,
		});
	}

	if let Some(storage_get) = storage_get_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::StorageGetter {
			identifier: storage_get.identifier,
		});
	}

	if let Some(storage_set) = storage_set_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::StorageSetter {
			identifier: storage_set.identifier,
		});
	}

	if let Some(storage_mapper) = storage_mapper_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::StorageMapper {
			identifier: storage_mapper.identifier,
		});
	}

	if let Some(storage_is_empty) = storage_is_empty_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::StorageIsEmpty {
			identifier: storage_is_empty.identifier,
		});
	}

	if let Some(storage_clear) = storage_clear_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::StorageClear {
			identifier: storage_clear.identifier,
		});
	}

	if is_proxy {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::ProxyGetter);
	}

	if let Some(module_attr) = module_opt {
		assert_no_other_auto_impl(&result);
		result = Some(AutoImpl::Module {
			impl_path: module_attr.arg,
		});
	}

	result
}
