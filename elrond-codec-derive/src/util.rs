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
