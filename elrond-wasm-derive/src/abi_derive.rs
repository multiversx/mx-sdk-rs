use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn type_abi_derive(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let name_str = name.to_string();
	let gen = match &ast.data {
		syn::Data::Struct(_) | syn::Data::Enum(_) => {
			let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
			quote! {
				impl #impl_generics elrond_wasm::abi::TypeAbi for #name #ty_generics #where_clause {
					fn type_name() -> elrond_wasm::String {
						#name_str.into()
					}
				}
			}
		},
		syn::Data::Union(_) => panic!("Union not supported!"),
	};

	gen.into()
}
