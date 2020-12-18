use proc_macro::TokenStream;
use quote::quote;
use syn;
use super::parse_attr::extract_doc;

pub fn type_abi_derive(ast: &syn::DeriveInput) -> TokenStream {
	let type_description_impl = match &ast.data {
		syn::Data::Struct(_) => {
			quote! {}
		},
		syn::Data::Enum(data_enum) => {
			let enum_variant_snippets: Vec<proc_macro2::TokenStream> = data_enum
				.variants
				.iter()
				.map(|v| {
					let variant_docs = extract_doc(v.attrs.as_slice());
					let variant_name_str = v.ident.to_string();
					quote! {
						variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
							docs: &[ #(#variant_docs),* ],
							name: #variant_name_str
						});
					}
				})
				.collect();
			let type_docs = extract_doc(ast.attrs.as_slice());
			quote! {
				fn type_description() -> Option<elrond_wasm::abi::TypeDescription> {
					let mut variant_descriptions = elrond_wasm::Vec::new();
					#(#enum_variant_snippets)*
					Some(elrond_wasm::abi::TypeDescription {
						docs: &[ #(#type_docs),* ],
						name: Self::type_name(),
						contents: elrond_wasm::abi::TypeContents::Enum(variant_descriptions),
					})
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported!"),
	};

	let name = &ast.ident;
	let name_str = name.to_string();
	let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
	let type_abi_impl = quote! {
		impl #impl_generics elrond_wasm::abi::TypeAbi for #name #ty_generics #where_clause {
			fn type_name() -> elrond_wasm::String {
				#name_str.into()
			}

			#type_description_impl
		}
	};
	type_abi_impl.into()
}
