use super::attr_names::*;
use super::util::*;

pub struct StorageGetAttribute {
	pub identifier: String,
}

impl StorageGetAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_GET) {
			None => None,
			Some(arg_str) => Some(StorageGetAttribute {
				identifier: arg_str,
			}),
		}
	}
}

pub struct StorageSetAttribute {
	pub identifier: String,
}

impl StorageSetAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_SET) {
			None => None,
			Some(arg_str) => Some(StorageSetAttribute {
				identifier: arg_str,
			}),
		}
	}
}

pub struct StorageMapperAttribute {
	pub identifier: String,
}

impl StorageMapperAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_MAPPER) {
			None => None,
			Some(arg_str) => Some(StorageMapperAttribute {
				identifier: arg_str,
			}),
		}
	}
}

pub struct StorageGetMutAttribute {
	pub identifier: String,
}

impl StorageGetMutAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_GET_MUT) {
			None => None,
			Some(arg_str) => Some(StorageGetMutAttribute {
				identifier: arg_str,
			}),
		}
	}
}

pub struct StorageIsEmptyAttribute {
	pub identifier: String,
}

impl StorageIsEmptyAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_IS_EMPTY) {
			None => None,
			Some(arg_str) => Some(StorageIsEmptyAttribute {
				identifier: arg_str,
			}),
		}
	}
}

pub struct StorageClearAttribute {
	pub identifier: String,
}

impl StorageClearAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<Self> {
		match find_attr_one_string_arg(m, ATTR_STORAGE_CLEAR) {
			None => None,
			Some(arg_str) => Some(StorageClearAttribute {
				identifier: arg_str,
			}),
		}
	}
}
