use super::{
    attributes::extract_doc, method_parse::process_method, parse_util::validate_attribute_args,
    supertrait_parse::parse_supertrait,
};
use crate::{
    model::{ContractTrait, Method, Supertrait, TraitProperties},
    parse::{is_contract_base, process_trait_arguments},
};

pub fn parse_contract_trait(
    args: syn::AttributeArgs,
    contract_trait: &syn::ItemTrait,
) -> ContractTrait {
    validate_attribute_args(args);

    let docs = extract_doc(contract_trait.attrs.as_slice());

    let mut trait_attributes = TraitProperties::default();
    let mut unprocessed_attributes = Vec::new();
    process_trait_arguments(
        contract_trait.attrs.as_slice(),
        &mut trait_attributes,
        &mut unprocessed_attributes,
    );

    let supertraits: Vec<Supertrait> = contract_trait
        .supertraits
        .iter()
        .filter(|supertrait| !is_contract_base(supertrait))
        .map(parse_supertrait)
        .collect();

    let methods: Vec<Method> = contract_trait
        .items
        .iter()
        .map(|itm| match itm {
            syn::TraitItem::Method(m) => process_method(m, &trait_attributes),
            _ => panic!("Only methods allowed in contract traits"),
        })
        .collect();

    ContractTrait {
        docs,
        original_attributes: unprocessed_attributes,
        trait_name: contract_trait.ident.clone(),
        supertraits,
        auto_inheritance_modules: Vec::new(),
        methods,
        trait_attributes,
    }
}
