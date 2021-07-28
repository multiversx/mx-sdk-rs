use crate::model::{Method, MethodImpl, MethodPayableMetadata, PublicRole};

use super::{
    attributes::extract_doc,
    auto_impl_parse::{
        process_event_attribute, process_legacy_event_attribute, process_proxy_attribute,
        process_storage_clear_attribute, process_storage_get_attribute,
        process_storage_is_empty_attribute, process_storage_mapper_attribute,
        process_storage_set_attribute,
    },
    extract_method_args, process_callback_attribute, process_callback_raw_attribute,
    process_endpoint_attribute, process_init_attribute, process_only_owner_attribute,
    process_output_names_attribute, process_payable_attribute, process_view_attribute,
};
pub struct MethodAttributesPass1 {
    pub method_name: String,
    pub payable: MethodPayableMetadata,
    pub only_owner: bool,
}

pub fn process_method(m: &syn::TraitItemMethod) -> Method {
    let method_args = extract_method_args(m);

    let implementation = if let Some(body) = m.default.clone() {
        MethodImpl::Explicit(body)
    } else {
        MethodImpl::NoImplementation
    };

    let mut first_pass_data = MethodAttributesPass1 {
        method_name: m.sig.ident.to_string(),
        payable: MethodPayableMetadata::NotPayable,
        only_owner: false,
    };
    let mut first_pass_unprocessed_attributes = Vec::new();

    process_attributes_first_pass(
        &m.attrs,
        &mut first_pass_data,
        &mut first_pass_unprocessed_attributes,
    );

    let mut method = Method {
        docs: extract_doc(m.attrs.as_slice()),
        public_role: PublicRole::Private,
        name: m.sig.ident.clone(),
        generics: m.sig.generics.clone(),
        unprocessed_attributes: Vec::new(),
        method_args,
        output_names: Vec::new(),
        return_type: m.sig.output.clone(),
        implementation,
    };

    process_attributes_second_pass(
        &first_pass_unprocessed_attributes,
        &first_pass_data,
        &mut method,
    );

    method
}

fn process_attributes_first_pass(
    attrs: &[syn::Attribute],
    first_pass_data: &mut MethodAttributesPass1,
    first_pass_unprocessed_attributes: &mut Vec<syn::Attribute>,
) {
    for attr in attrs {
        let processed = process_attribute_first_pass(attr, first_pass_data);
        if !processed {
            first_pass_unprocessed_attributes.push(attr.clone());
        }
    }
}

fn process_attribute_first_pass(
    attr: &syn::Attribute,
    first_pass_data: &mut MethodAttributesPass1,
) -> bool {
    process_payable_attribute(attr, first_pass_data)
        || process_only_owner_attribute(attr, first_pass_data)
}

fn process_attributes_second_pass(
    attrs: &[syn::Attribute],
    first_pass_data: &MethodAttributesPass1,
    method: &mut Method,
) {
    for attr in attrs {
        let processed = process_attribute_second_pass(attr, first_pass_data, method);
        if !processed {
            method.unprocessed_attributes.push(attr.clone());
        }
    }
}

fn process_attribute_second_pass(
    attr: &syn::Attribute,
    first_pass_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    process_init_attribute(attr, first_pass_data, method)
        || process_endpoint_attribute(attr, first_pass_data, method)
        || process_view_attribute(attr, first_pass_data, method)
        || process_callback_raw_attribute(attr, method)
        || process_callback_attribute(attr, method)
        || process_legacy_event_attribute(attr, method)
        || process_event_attribute(attr, method)
        || process_proxy_attribute(attr, method)
        || process_storage_get_attribute(attr, method)
        || process_storage_set_attribute(attr, method)
        || process_storage_mapper_attribute(attr, method)
        || process_storage_is_empty_attribute(attr, method)
        || process_storage_clear_attribute(attr, method)
        || process_output_names_attribute(attr, method)
}
