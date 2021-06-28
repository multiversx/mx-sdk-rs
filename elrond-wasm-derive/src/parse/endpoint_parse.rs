use crate::model::{CallbackMetadata, EndpointMetadata, InitMetadata, Method, PublicRole};

use super::{
	attributes::{
		is_callback_raw, is_init, CallbackAttribute, EndpointAttribute, OutputNameAttribute,
		ViewAttribute,
	},
	MethodAttributesPass1,
};

fn check_single_role(method: &Method) {
	if !matches!(method.public_role, PublicRole::Private) {
		panic!(
			"Can only annotate with one of the following arguments: `#[init]`, `#[endpoint]`, `#[view]`, `#[callback]`, `#[callback_raw]`."
		);
	}
}

pub fn process_init_attribute(
	attr: &syn::Attribute,
	pass_1_data: &MethodAttributesPass1,
	method: &mut Method,
) -> bool {
	if is_init(attr) {
		check_single_role(&*method);
		method.public_role = PublicRole::Init(InitMetadata {
			payable: pass_1_data.payable.clone(),
		});
		true
	} else {
		false
	}
}

pub fn process_endpoint_attribute(
	attr: &syn::Attribute,
	pass_1_data: &MethodAttributesPass1,
	method: &mut Method,
) -> bool {
	EndpointAttribute::parse(attr)
		.map(|endpoint_attr| {
			check_single_role(&*method);
			let endpoint_ident = match endpoint_attr.endpoint_name {
				Some(ident) => ident,
				None => method.name.clone(),
			};
			method.public_role = PublicRole::Endpoint(EndpointMetadata {
				public_name: endpoint_ident,
				payable: pass_1_data.payable.clone(),
			});
		})
		.is_some()
}

pub fn process_view_attribute(
	attr: &syn::Attribute,
	pass_1_data: &MethodAttributesPass1,
	method: &mut Method,
) -> bool {
	ViewAttribute::parse(attr)
		.map(|view_attribute| {
			check_single_role(&*method);
			let view_ident = match view_attribute.view_name {
				Some(ident) => ident,
				None => method.name.clone(),
			};
			method.public_role = PublicRole::Endpoint(EndpointMetadata {
				public_name: view_ident,
				payable: pass_1_data.payable.clone(),
			});
		})
		.is_some()
}

pub fn process_callback_raw_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
	if is_callback_raw(attr) {
		check_single_role(&*method);
		method.public_role = PublicRole::CallbackRaw;
		true
	} else {
		false
	}
}

pub fn process_callback_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
	CallbackAttribute::parse(attr)
		.map(|callback_attr| {
			check_single_role(&*method);
			let callback_ident = match callback_attr.callback_name {
				Some(ident) => ident,
				None => method.name.clone(),
			};
			method.public_role = PublicRole::Callback(CallbackMetadata {
				callback_name: callback_ident,
			});
		})
		.is_some()
}

pub fn process_output_names_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
	OutputNameAttribute::parse(attr)
		.map(|output_name_attr| {
			method.output_names.push(output_name_attr.output_name);
		})
		.is_some()
}
