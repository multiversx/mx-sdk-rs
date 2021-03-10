use super::Method;

/// Models a contract or module trait.
pub struct ContractTrait {
	pub docs: Vec<String>,
	pub trait_name: proc_macro2::Ident,
	pub contract_impl_name: syn::Path,
	pub supertrait_paths: Vec<syn::Path>,
	pub methods: Vec<Method>,
}
