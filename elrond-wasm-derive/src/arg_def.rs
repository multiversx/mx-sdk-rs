#[derive(Clone, Debug)]
pub struct MethodArg {
	pub index: i32,
	pub pat: syn::Pat,
	pub ty: syn::Type,
	pub metadata: ArgMetadata,
	pub event_topic: bool, // TODO: reorganize arg metadata
}

#[derive(Clone, Debug)]
pub enum ArgMetadata {
	Payment,
	PaymentToken,
	Single,
	VarArgs,
	AsyncCallResultArg,
}

pub fn generate_arg_call_name(arg: &MethodArg) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	match &arg.ty {
		syn::Type::Reference(_) => quote! { &#pat },
		_ => quote! { #pat },
	}
}
