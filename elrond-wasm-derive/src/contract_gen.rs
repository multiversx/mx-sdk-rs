use super::contract_gen_callback::*;
use super::contract_gen_event::*;
use super::contract_gen_method::*;
use super::contract_gen_module::*;
use super::contract_gen_storage::*;
use super::parse_attr::*;
use super::snippets;
use super::util::*;

pub struct Contract {
	pub docs: Vec<String>,
	pub trait_name: proc_macro2::Ident,
	pub contract_impl_name: syn::Path,
	pub supertrait_paths: Vec<syn::Path>,
	pub methods: Vec<Method>,
}

impl Contract {
	pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
		let contract_impl_name = extract_struct_name(args);

		let docs = extract_doc(contract_trait.attrs.as_slice());

		let supertrait_paths: Vec<syn::Path> = contract_trait
			.supertraits
			.iter()
			.map(|supertrait| match supertrait {
				syn::TypeParamBound::Trait(t) => t.path.clone(),
				_ => panic!("Contract trait can only extend other traits."),
			})
			.collect();

		let methods: Vec<Method> = contract_trait
			.items
			.iter()
			.map(|itm| match itm {
				syn::TraitItem::Method(m) => Method::parse(m),
				_ => panic!("Only methods allowed in contract traits"),
			})
			.collect();

		Contract {
			docs,
			trait_name: contract_trait.ident.clone(),
			contract_impl_name,
			supertrait_paths,
			methods,
		}
	}

	pub fn extract_pub_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| {
				if m.metadata.endpoint_name().is_some() {
					Some(m.generate_sig())
				} else {
					None
				}
			})
			.collect()
	}

	pub fn extract_method_impls(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| {
				if m.metadata.has_implementation() {
					let body = match m.body {
						Some(ref mbody) => {
							let msig = m.generate_sig();
							quote! {
								#msig
								#mbody
							}
						},
						None => quote! {},
					};
					Some(body)
				} else {
					None
				}
			})
			.collect()
	}

	pub fn generate_call_methods(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| {
				if m.metadata.endpoint_name().is_some() {
					Some(m.generate_call_method())
				} else {
					None
				}
			})
			.collect()
	}

	/// Definitions for methods that get auto-generated implementations: events, getters, setters
	pub fn generate_auto_impl_defs(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| match m.metadata {
				MethodMetadata::Event { .. }
				| MethodMetadata::StorageGetter { .. }
				| MethodMetadata::StorageSetter { .. }
				| MethodMetadata::StorageGetMut { .. }
				| MethodMetadata::StorageIsEmpty { .. }
				| MethodMetadata::StorageClear { .. }
				| MethodMetadata::Module { .. } => {
					let sig = m.generate_sig();
					Some(quote! { #sig ; })
				},
				_ => None,
			})
			.collect()
	}

	/// Implementations for methods that get auto-generated implementations: events, getters, setters
	pub fn generate_auto_impls(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| match &m.metadata {
				MethodMetadata::Event { identifier } => {
					Some(generate_event_impl(&m, identifier.clone()))
				},
				MethodMetadata::StorageGetter {
					visibility: _,
					identifier,
				} => Some(generate_getter_impl(&m, identifier.clone())),
				MethodMetadata::StorageSetter {
					visibility: _,
					identifier,
				} => Some(generate_setter_impl(&m, identifier.clone())),
				MethodMetadata::StorageGetMut {
					visibility: _,
					identifier,
				} => Some(generate_borrow_impl(&m, identifier.clone())),
				MethodMetadata::StorageIsEmpty {
					visibility: _,
					identifier,
				} => Some(generate_is_empty_impl(&m, identifier.clone())),
				MethodMetadata::StorageClear {
					visibility: _,
					identifier,
				} => Some(generate_clear_impl(&m, identifier.clone())),
				MethodMetadata::Module { impl_path } => {
					Some(generate_module_getter_impl(&m, &impl_path))
				},
				_ => None,
			})
			.collect()
	}

	pub fn generate_storage_impls(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| match &m.metadata {
				MethodMetadata::Event { identifier } => {
					Some(generate_event_impl(&m, identifier.clone()))
				},
				_ => None,
			})
			.collect()
	}

	pub fn generate_supertrait_impls(&self) -> Vec<proc_macro2::TokenStream> {
		let contract_impl_ident = self.contract_impl_name.clone();
		let api_where = snippets::api_where();
		self.supertrait_paths
			.iter()
			.map(|supertrait_path| {
				quote! {
					impl <T, BigInt, BigUint> #supertrait_path<T, BigInt, BigUint> for #contract_impl_ident<T, BigInt, BigUint>
					#api_where
					{}

				}
			})
			.collect()
	}

	pub fn generate_endpoints(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.filter_map(|m| {
				if let Some(endpoint_name) = m.metadata.endpoint_name() {
					let fn_ident = &m.name;
					let call_method_ident = generate_call_method_name(&fn_ident);
					let endpoint = quote! {
						#[no_mangle]
						pub fn #endpoint_name ()
						{
							let inst = new_arwen_instance();
							inst.#call_method_ident();
						}
					};
					Some(endpoint)
				} else {
					None
				}
			})
			.collect()
	}

	pub fn generate_callback_body(&self) -> proc_macro2::TokenStream {
		generate_callback_body(&self.methods)
	}
}
