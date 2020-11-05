use super::parse_attr::*;

#[derive(Clone, Debug)]
pub struct MethodArg {
	pub index: i32,
	pub pat: syn::Pat,
	pub ty: syn::Type,
	pub is_callback_arg: bool,
	pub metadata: ArgMetadata,
}

#[derive(Clone, Debug)]
pub enum ArgMetadata {
	Payment,
	Single,
	Multi(MultiAttribute),
	VarArgs,
}

pub fn generate_arg_call_name(arg: &MethodArg) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	match &arg.ty {
		syn::Type::Reference(_) => quote! { &#pat },
		_ => quote! { #pat },
	}
}
