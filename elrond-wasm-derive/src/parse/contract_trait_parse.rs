use super::attributes::extract_doc;
use super::method_parse::process_method;
use super::parse_util::validate_attribute_args;
use super::supertrait_parse::parse_supertrait;
use crate::model::{ContractTrait, Method, Supertrait};

pub fn parse_contract_trait(
	args: syn::AttributeArgs,
	contract_trait: &syn::ItemTrait,
) -> ContractTrait {
	validate_attribute_args(args);

	let docs = extract_doc(contract_trait.attrs.as_slice());

	let supertraits: Vec<Supertrait> = contract_trait
		.supertraits
		.iter()
		.map(parse_supertrait)
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
		supertraits,
		auto_inheritance_modules: Vec::new(),
		methods,
	}
}
