use super::arg_def::*;
use super::contract_gen_method::*;
use super::util::*;

fn storage_store_snippet(arg: &MethodArg) -> proc_macro2::TokenStream {
	let pat = &arg.pat;
	quote! {
		elrond_wasm::storage_set(self.api.clone(), &key[..], & #pat);
	}
}

fn storage_load_snippet(_ty: &syn::Type) -> proc_macro2::TokenStream {
	quote! {
		elrond_wasm::storage_get(self.api.clone(), &key[..])
	}
}

fn storage_clear_snippet() -> proc_macro2::TokenStream {
	quote! {
		elrond_wasm::storage_set(self.api.clone(), &key[..], &Vec::<u8>::new());
	}
}

fn generate_key_snippet(key_args: &[MethodArg], identifier: String) -> proc_macro2::TokenStream {
	let id_literal = array_literal(identifier.as_bytes());
	if key_args.is_empty() {
		// hardcode key
		quote! {
			let key: &'static [u8] = &#id_literal;
		}
	} else {
		// build key from arguments
		let key_appends: Vec<proc_macro2::TokenStream> = key_args
			.iter()
			.map(|arg| {
				let arg_pat = &arg.pat;
				quote! {
					if let Result::Err(encode_error) = #arg_pat.dep_encode(&mut key) {
						self.api.signal_error(encode_error.message_bytes());
					}
				}
			})
			.collect();
		quote! {
			let mut key: Vec<u8> = #id_literal.to_vec();
			#(#key_appends)*
		}
	}
}

pub fn generate_getter_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	match m.return_type.clone() {
		syn::ReturnType::Default => panic!("getter should return some value"),
		syn::ReturnType::Type(_, ty) => {
			let load_snippet = storage_load_snippet(&ty);
			quote! {
				#msig {
					#key_snippet
					#load_snippet
				}
			}
		},
	}
}

pub fn generate_setter_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	if m.method_args.is_empty() {
		panic!("setter must have at least one argument, for the value");
	}
	if m.return_type != syn::ReturnType::Default {
		panic!("setter should not return anything");
	}
	let key_args = &m.method_args[..m.method_args.len() - 1];
	let key_snippet = generate_key_snippet(key_args, identifier);
	let value_arg = &m.method_args[m.method_args.len() - 1];
	let store_snippet = storage_store_snippet(value_arg);
	quote! {
		#msig {
			#key_snippet
			#store_snippet
		}
	}
}

pub fn generate_borrow_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	if m.method_args.is_empty() {
		// const key
		quote! {
			#msig {
				#key_snippet
				BorrowedMutStorage::with_const_key(self.api.clone(), key)
			}
		}
	} else {
		// generated key
		quote! {
			#msig {
				#key_snippet
				BorrowedMutStorage::with_generated_key(self.api.clone(), key)
			}
		}
	}
}

pub fn generate_getter_or_default_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	match m.return_type.clone() {
		syn::ReturnType::Default => panic!("getter should return some value"),
		syn::ReturnType::Type(_, ty) => {
			let load_snippet = storage_load_snippet(&ty);
			quote! {
				#msig {
					#key_snippet
					if self.api.storage_load_len(&key[..]) > 0 {
						#load_snippet
					}
					else {
						<#ty>::default()
					}
				}
			}
		},
	}
}

pub fn generate_is_empty_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	quote! {
		#msig {
			#key_snippet
			self.api.storage_load_len(&key[..]) == 0
		}
	}
}

pub fn generate_clear_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	if m.return_type != syn::ReturnType::Default {
		panic!("storage clear should not return anything");
	}
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	let clear_snippet = storage_clear_snippet();
	quote! {
		#msig {
			#key_snippet
			#clear_snippet
		}
	}
}
