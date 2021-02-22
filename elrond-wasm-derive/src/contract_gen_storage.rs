use super::arg_def::*;
use super::contract_gen_method::*;
use super::util::*;

fn generate_key_snippet(key_args: &[MethodArg], identifier: String) -> proc_macro2::TokenStream {
	let id_literal = byte_str_literal(identifier.as_bytes());
	if key_args.is_empty() {
		// hardcode key
		quote! {
			let key: &'static [u8] = #id_literal;
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
		syn::ReturnType::Type(_, _ty) => {
			quote! {
				#msig {
					#key_snippet
					elrond_wasm::storage_get(self.get_storage_raw(), &key[..])
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
	let pat = &value_arg.pat;
	quote! {
		#msig {
			#key_snippet
			elrond_wasm::storage_set(self.get_storage_raw(), &key[..], & #pat);
		}
	}
}

pub fn generate_mapper_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	match m.return_type.clone() {
		syn::ReturnType::Default => panic!("getter should return some value"),
		syn::ReturnType::Type(_, ty) => {
			quote! {
				#msig {
					#key_snippet
					<#ty as elrond_wasm::storage::mappers::StorageMapper<Self::Storage>>::new(
						self.get_storage_raw(),
						elrond_wasm::types::BoxedBytes::from(key),
					)
				}
			}
		},
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
				BorrowedMutStorage::with_generated_key(self.get_storage_raw(), key)
			}
		}
	}
}

pub fn generate_is_empty_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	quote! {
		#msig {
			#key_snippet
			elrond_wasm::api::StorageReadApi::storage_load_len(&self.get_storage_raw(), &key[..]) == 0
		}
	}
}

pub fn generate_clear_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
	let msig = m.generate_sig();
	if m.return_type != syn::ReturnType::Default {
		panic!("storage clear should not return anything");
	}
	let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
	quote! {
		#msig {
			#key_snippet
			elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(&self.get_storage_raw(), &key[..], &[]);
		}
	}
}
