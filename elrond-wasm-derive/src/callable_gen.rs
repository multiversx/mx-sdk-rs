use super::arg_def::*;
use super::arg_extract::*;
use super::arg_str_serialize::*;
use super::contract_gen_method::*;
// use super::parse_attr::*;
use super::util::*;

#[derive(Clone, Debug)]
pub struct CallableMethod {
	pub name: syn::Ident,
	pub payable: bool,
	pub generics: syn::Generics,
	pub method_args: Vec<MethodArg>,
	pub return_type: syn::ReturnType,
}

impl CallableMethod {
	pub fn parse(m: &syn::TraitItemMethod) -> CallableMethod {
		let payable = process_payable(m);
		if let MethodPayableMetadata::SingleEsdtToken(_) | MethodPayableMetadata::AnyToken = payable
		{
			panic!("payable methods in async call proxies currently only accept EGLD");
		}

		let method_args = extract_method_args(m);
		CallableMethod {
			name: m.sig.ident.clone(),
			payable: payable.is_payable(),
			generics: m.sig.generics.clone(),
			method_args,
			return_type: m.sig.output.clone(),
		}
	}

	pub fn generate_proxy_sig(&self) -> proc_macro2::TokenStream {
		let method_name = &self.name;
		let generics = &self.generics;
		let generics_where = &self.generics.where_clause;
		let arg_decl = arg_declarations(&self.method_args);
		let ret_tok = match &self.return_type {
			syn::ReturnType::Default => quote! {},
			syn::ReturnType::Type(_, ty) => quote! { -> #ty },
		};
		let result =
			quote! { fn #method_name #generics ( self , #(#arg_decl),* ) #ret_tok #generics_where };
		result
	}
}

#[derive(Clone, Debug)]
pub struct Callable {
	pub trait_name: proc_macro2::Ident,
	pub callable_impl_name: proc_macro2::Ident,
	pub contract_impl_name: syn::Path,
	methods: Vec<CallableMethod>,
}

impl Callable {
	pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
		let callable_impl_name =
			generate_callable_interface_impl_struct_name(&contract_trait.ident);
		let contract_impl_name = extract_struct_name(args);

		let methods: Vec<CallableMethod> = contract_trait
			.items
			.iter()
			.map(|itm| match itm {
				syn::TraitItem::Method(m) => CallableMethod::parse(m),
				_ => panic!("Only methods allowed in callable traits"),
			})
			.collect();

		Callable {
			trait_name: contract_trait.ident.clone(),
			callable_impl_name,
			contract_impl_name,
			methods,
		}
	}
}

impl Callable {
	pub fn extract_pub_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.map(|m| {
				let sig = m.generate_proxy_sig();
				quote! { #sig ; }
			})
			.collect()
	}

	pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.map(|m| {
				let msig = m.generate_proxy_sig();

				let mut payment_count = 0;
				let mut payment_expr = quote! { self.payment };
				let mut token_count = 0;
				let mut token_expr = quote! { self.token };
				let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						let arg_accumulator = quote! { &mut async_call.hex_data };

						match &arg.metadata {
							ArgMetadata::Single | ArgMetadata::VarArgs => {
								arg_serialize_push(arg, &arg_accumulator)
							},
							ArgMetadata::Payment => {
								payment_count += 1;
								let pat = &arg.pat;
								payment_expr = quote! { #pat };

								quote! {}
							},
							ArgMetadata::PaymentToken => {
								token_count += 1;
								let pat = &arg.pat;
								token_expr = quote! { #pat };

								quote! {}
							},
							ArgMetadata::AsyncCallResultArg => panic!("async call result arg not allowed in call proxy"),
						}
					})
					.collect();

				if payment_count > 1 {
					panic!("No more than one payment argument allowed in call proxy");
				}
				if token_count > 1 {
					panic!("No more than one payment token argument allowed in call proxy");
				}

				let m_name_literal = ident_str_literal(&m.name);
				let sig = quote! {
					#msig {
						let mut async_call = AsyncCall::<BigUint>::new(
							self.address,
							#token_expr,
							#payment_expr,
							#m_name_literal);
						#(#arg_push_snippets)*
						async_call
					}
				};
				sig
			})
			.collect()
	}
}
