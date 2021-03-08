use crate::model::{Method, MethodArgument};

pub fn arg_declarations(method_args: &[MethodArgument]) -> Vec<proc_macro2::TokenStream> {
	method_args
		.iter()
		.map(|arg| {
			let pat = &arg.pat;
			let ty = &arg.ty;
			quote! {#pat : #ty }
		})
		.collect()
}

pub fn generate_sig(m: &Method) -> proc_macro2::TokenStream {
	let method_name = &m.name;
	let generics = &m.generics;
	let generics_where = &m.generics.where_clause;
	let arg_decl = arg_declarations(&m.method_args);
	let ret_tok = match &m.return_type {
		syn::ReturnType::Default => quote! {},
		syn::ReturnType::Type(_, ty) => quote! { -> #ty },
	};
	let result =
		quote! { fn #method_name #generics ( &self , #(#arg_decl),* ) #ret_tok #generics_where };
	result
}

pub fn generate_arg_call_name(arg: &MethodArgument) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	match &arg.ty {
		syn::Type::Reference(_) => quote! { &#pat },
		_ => quote! { #pat },
	}
}
