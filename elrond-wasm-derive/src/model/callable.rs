use super::{MethodArgument, MethodPayableMetadata};

#[derive(Clone, Debug)]
pub struct CallableMethod {
	pub name: syn::Ident,
	pub payable: MethodPayableMetadata,
	pub generics: syn::Generics,
	pub method_args: Vec<MethodArgument>,
	pub return_type: syn::ReturnType,
}

#[derive(Clone, Debug)]
pub struct CallableTrait {
	pub trait_name: proc_macro2::Ident,
	pub callable_impl_name: proc_macro2::Ident,
	pub contract_impl_name: syn::Path,
	pub methods: Vec<CallableMethod>,
}
