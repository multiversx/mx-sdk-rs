use crate::model::Method;

use super::{
	attributes::{extract_doc, find_output_names},
	extract_method_args,
	method_impl_parse::process_method_impl,
	process_public_role,
};

pub fn process_method(m: &syn::TraitItemMethod) -> Method {
	let public_role = process_public_role(m);
	let method_args = extract_method_args(m);
	let output_names = find_output_names(m);
	let implementation = process_method_impl(m);

	Method {
		docs: extract_doc(m.attrs.as_slice()),
		public_role,
		name: m.sig.ident.clone(),
		generics: m.sig.generics.clone(),
		remaining_attributes: Vec::new(), // TODO: keep unprocessed attributes (e.g. `#[inline]` here)
		method_args,
		output_names,
		return_type: m.sig.output.clone(),
		implementation,
	}
}
