use super::attributes::*;
use crate::model::TraitProperties;

pub fn process_trait_arguments(
    attrs: &[syn::Attribute],
    trait_arg_metadata: &mut TraitProperties,
    unprocessed_attributes: &mut Vec<syn::Attribute>,
) {
    for attr in attrs {
        let processed = process_trait_attribute(attr, trait_arg_metadata);
        if !processed {
            unprocessed_attributes.push(attr.clone())
        }
    }
}

fn process_trait_attribute(
    attr: &syn::Attribute,
    trait_arg_metadata: &mut TraitProperties,
) -> bool {
    process_only_owner_argument(attr, trait_arg_metadata)
        || process_only_admin_argument(attr, trait_arg_metadata)
}

fn process_only_owner_argument(attr: &syn::Attribute, arg_metadata: &mut TraitProperties) -> bool {
    let has_attr = is_only_owner_prop(attr);
    if has_attr {
        arg_metadata.only_owner = true;
    }
    has_attr
}

fn process_only_admin_argument(attr: &syn::Attribute, arg_metadata: &mut TraitProperties) -> bool {
    let has_attr = is_only_admin_prop(attr);
    if has_attr {
        arg_metadata.only_admin = true;
    }
    has_attr
}
