use super::arg_def::*;
use super::arg_extract::*;
use super::arg_str_serialize::*;
use super::contract_gen_method::*;
use super::parse_attr::*;
use super::util::*;

#[derive(Clone, Debug)]
pub struct CallableMethod {
	pub name: syn::Ident,
	pub payable: bool,
	pub callback: Option<CallbackCallAttribute>,
	pub method_args: Vec<MethodArg>,
}

impl CallableMethod {
	pub fn parse(m: &syn::TraitItemMethod) -> CallableMethod {
		let payable = process_payable(m);
		if let MethodPayableMetadata::SingleEsdtToken(_) | MethodPayableMetadata::AnyToken = payable
		{
			panic!("payable methods in async call proxies currently only accept EGLD");
		}

		let callback_opt = CallbackCallAttribute::parse(m);
		let method_args = extract_method_args(m, callback_opt.is_some());
		CallableMethod {
			name: m.sig.ident.clone(),
			payable: payable.is_payable(),
			callback: callback_opt,
			method_args,
		}
	}

	// TODO: deduplicate
	pub fn generate_sig(&self) -> proc_macro2::TokenStream {
		let method_name = &self.name;
		let arg_decl = arg_declarations(&self.method_args);
		let result = quote! {
			#[allow(non_snake_case)]
			fn #method_name ( &self , #(#arg_decl),* ) -> ()
		};
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

		//let trait_methods = extract_methods(&contract_trait);
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
				let sig = m.generate_sig();
				quote! { #sig ; }
			})
			.collect()
	}

	pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
		self.methods
			.iter()
			.map(|m| {
				let msig = m.generate_sig();

				let mut payment_count = 0;
				let mut amount_snippet = quote! { BigUint::zero() };
				let arg_push_snippets: Vec<proc_macro2::TokenStream> = m
					.method_args
					.iter()
					.map(|arg| {
						let arg_accumulator = if arg.is_callback_arg {
							quote! { callback_data_ser }
						} else {
							quote! { call_data_ser }
						};

						match &arg.metadata {
							ArgMetadata::Single | ArgMetadata::VarArgs => {
								arg_serialize_push(arg, &arg_accumulator)
							},
							ArgMetadata::Payment => {
								// #[payment]
								payment_count += 1;
								let pat = &arg.pat;
								amount_snippet = quote! { #pat };

								quote! {}
							},
							ArgMetadata::PaymentToken => panic!("callable payment token not yet supported"),
						}
					})
					.collect();

				if payment_count > 1 {
					panic!("Only one payment argument allowed in call proxy");
				}

				let (callback_init, callback_store) = if let Some(callback_ident) = &m.callback {
					let cb_name_str = &callback_ident.arg.to_string();
					let cb_name_literal = array_literal(cb_name_str.as_bytes());
					let callback_init = quote! {
						let mut callback_data_ser = elrond_wasm::hex_call_data::HexCallDataSerializer::new( & #cb_name_literal );
					};
					let callback_store = quote! {
						self.api.storage_store_slice_u8(&self.api.get_tx_hash().as_ref(), callback_data_ser.as_slice());
					};
					(callback_init, callback_store)
				} else {
					(quote! {}, quote! {})
				};

				let m_name_literal = array_literal(m.name.to_string().as_bytes());
				let sig = quote! {
					#msig {
						let mut call_data_ser = elrond_wasm::hex_call_data::HexCallDataSerializer::new( & #m_name_literal );
						#callback_init
						#(#arg_push_snippets)*
						#callback_store
						self.api.send().async_call_raw(&self.address, &#amount_snippet, call_data_ser.as_slice());
					}
				};
				sig
			})
			.collect()
	}
}
