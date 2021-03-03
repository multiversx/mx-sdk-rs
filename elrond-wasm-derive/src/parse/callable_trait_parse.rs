use super::parse_util::*;
use super::{extract_method_args, process_payable};
use crate::generate::util::generate_callable_interface_impl_struct_name; // TODO: remove dependency
use crate::model::{CallableMethod, CallableTrait};

pub fn parse_callable_trait(
	args: syn::AttributeArgs,
	contract_trait: &syn::ItemTrait,
) -> CallableTrait {
	let callable_impl_name = generate_callable_interface_impl_struct_name(&contract_trait.ident);
	let contract_impl_name = extract_struct_name(args);

	let methods: Vec<CallableMethod> = contract_trait
		.items
		.iter()
		.map(|itm| match itm {
			syn::TraitItem::Method(m) => parse_callable_method(m),
			_ => panic!("Only methods allowed in callable traits"),
		})
		.collect();

	CallableTrait {
		trait_name: contract_trait.ident.clone(),
		callable_impl_name,
		contract_impl_name,
		methods,
	}
}

fn parse_callable_method(m: &syn::TraitItemMethod) -> CallableMethod {
	let payable = process_payable(m);
	let method_args = extract_method_args(m);
	CallableMethod {
		name: m.sig.ident.clone(),
		payable,
		generics: m.sig.generics.clone(),
		method_args,
		return_type: m.sig.output.clone(),
	}
}
