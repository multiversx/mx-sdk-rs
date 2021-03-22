use super::attr_names::*;
use super::util::*;

pub struct StorageGetAttribute {
	pub identifier: String,
}

impl StorageGetAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		find_attr_one_string_arg(m, ATTR_STORAGE_GET).map(|arg_str| StorageGetAttribute {
			identifier: arg_str,
		})
	}
}

pub struct StorageSetAttribute {
	pub identifier: String,
}

impl StorageSetAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		find_attr_one_string_arg(m, ATTR_STORAGE_SET).map(|arg_str| StorageSetAttribute {
			identifier: arg_str,
		})
	}
}

pub struct StorageMapperAttribute {
	pub identifier: String,
}

impl StorageMapperAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		find_attr_one_string_arg(m, ATTR_STORAGE_MAPPER).map(|arg_str| StorageMapperAttribute {
			identifier: arg_str,
		})
	}
}

pub struct StorageIsEmptyAttribute {
	pub identifier: String,
}

impl StorageIsEmptyAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		find_attr_one_string_arg(m, ATTR_STORAGE_IS_EMPTY).map(|arg_str| StorageIsEmptyAttribute {
			identifier: arg_str,
		})
	}
}

pub struct StorageClearAttribute {
	pub identifier: String,
}

impl StorageClearAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		find_attr_one_string_arg(m, ATTR_STORAGE_CLEAR).map(|arg_str| StorageClearAttribute {
			identifier: arg_str,
		})
	}
}
