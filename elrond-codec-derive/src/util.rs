use quote::quote;
use syn;

pub fn extract_field_names(data: &syn::Data) -> Vec<syn::Ident> {
	match data {
		syn::Data::Struct(s) => match &s.fields {
			syn::Fields::Named(fields) => fields
				.named
				.iter()
				.map(|f| f.clone().ident.unwrap())
				.collect(),
			syn::Fields::Unnamed(_) => Vec::new(),
			syn::Fields::Unit => panic!("unit not supported"),
		},
		syn::Data::Enum(e) => e.variants.iter().map(|v| v.clone().ident).collect(),
		syn::Data::Union(_) => panic!("unions not supported"),
	}
}

pub fn extract_struct_field_types(data: &syn::Data) -> Vec<syn::Type> {
	match data {
		syn::Data::Struct(s) => match &s.fields {
			syn::Fields::Named(fields) => fields.named.iter().map(|f| f.ty.clone()).collect(),
			syn::Fields::Unnamed(fields) => fields.unnamed.iter().map(|f| f.ty.clone()).collect(),
			syn::Fields::Unit => panic!("unit not supported"),
		},
		_ => panic!("only structs supported"),
	}
}

pub fn extract_enum_field_types(data: &syn::Data) -> Vec<Vec<syn::Type>> {
	match data {
		syn::Data::Enum(e) => e
			.variants
			.iter()
			.map(|v| {
				let mut field_types = Vec::new();
				for field in &v.fields {
					field_types.push(field.ty.clone());
				}

				field_types
			})
			.collect(),
		_ => panic!("only enums supported"),
	}
}

pub fn is_simple_enum(data: &syn::Data) -> bool {
	let types = extract_enum_field_types(data);

	for type_list in &types {
		if type_list.len() > 0 {
			return false;
		}
	}

	return true;
}

pub fn fields_snippets<F>(fields: &syn::Fields, field_snippet: F) -> Vec<proc_macro2::TokenStream>
where
	F: Fn(usize, &syn::Field) -> proc_macro2::TokenStream,
{
	match fields {
		syn::Fields::Named(fields_named) => fields_named
			.named
			.iter()
			.enumerate()
			.map(|(index, field)| field_snippet(index, field))
			.collect(),
		syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
			.unnamed
			.iter()
			.enumerate()
			.map(|(index, field)| field_snippet(index, field))
			.collect(),
		syn::Fields::Unit => Vec::new(),
	}
}

pub fn self_field_expr(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
	if let Some(ident) = &field.ident {
		quote! {
			self.#ident
		}
	} else {
		quote! {
			self.#index
		}
	}
}

pub fn local_variable_for_field(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
	if let Some(ident) = &field.ident {
		quote! {
			#ident
		}
	} else {
		let local_var_name = format!("unnamed_{}", index);
		let local_var_ident = syn::Ident::new(&local_var_name, proc_macro2::Span::call_site());
		quote! {
			#local_var_ident
		}
	}
}

pub fn match_local_var_declarations(fields: &syn::Fields) -> proc_macro2::TokenStream {
	match fields {
		syn::Fields::Named(fields_named) => {
			let local_variables: Vec<proc_macro2::TokenStream> = fields_named
				.named
				.iter()
				.enumerate()
				.map(|(index, field)| local_variable_for_field(index, field))
				.collect();
			quote! {
				{ #(#local_variables),* }
			}
		},
		syn::Fields::Unnamed(fields_unnamed) => {
			let local_variables: Vec<proc_macro2::TokenStream> = fields_unnamed
				.unnamed
				.iter()
				.enumerate()
				.map(|(index, field)| local_variable_for_field(index, field))
				.collect();
			quote! {
				( #(#local_variables),* )
			}
		},
		syn::Fields::Unit => quote! {},
	}
}
