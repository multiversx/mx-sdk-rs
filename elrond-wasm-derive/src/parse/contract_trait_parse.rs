use super::attributes::extract_doc;
use super::method_parse::process_method;
use super::parse_util::extract_struct_name;
use crate::model::{ContractTrait, Method};

pub fn parse_contract_trait(
	args: syn::AttributeArgs,
	contract_trait: &syn::ItemTrait,
) -> ContractTrait {
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
			syn::TraitItem::Method(m) => process_method(m),
			_ => panic!("Only methods allowed in contract traits"),
		})
		.collect();

	ContractTrait {
		docs,
		trait_name: contract_trait.ident.clone(),
		contract_impl_name,
		supertrait_paths,
		methods,
	}
}
