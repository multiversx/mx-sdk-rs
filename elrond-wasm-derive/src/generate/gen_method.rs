use super::arg_def::*;
use super::arg_extract::*;
use super::arg_regular::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::parse_attr::*;
use super::reserved;
use super::util::*;




// #[derive(Clone, Debug)]
// pub enum MethodMetadata {
// 	Regular {
// 		visibility: Visibility,
// 		payable: MethodPayableMetadata,
// 	},
// 	LegacyEvent {
// 		identifier: Vec<u8>,
// 	},
// 	Event {
// 		identifier: String,
// 	},
// 	Callback,
// 	CallbackRaw,
// 	StorageGetter {
// 		visibility: Visibility,
// 		identifier: String,
// 	},
// 	StorageSetter {
// 		visibility: Visibility,
// 		identifier: String,
// 	},
// 	StorageMapper {
// 		visibility: Visibility,
// 		identifier: String,
// 	},
// 	StorageIsEmpty {
// 		visibility: Visibility,
// 		identifier: String,
// 	},
// 	StorageClear {
// 		visibility: Visibility,
// 		identifier: String,
// 	},
// 	Module {
// 		impl_path: proc_macro2::TokenTree,
// 	},
// }

impl MethodMetadata {
	// pub fn endpoint_name(&self) -> Option<&syn::Ident> {
	// 	match self {
	// 		MethodMetadata::Regular {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		}
	// 		| MethodMetadata::StorageGetter {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		}
	// 		| MethodMetadata::StorageSetter {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		}
	// 		| MethodMetadata::StorageMapper {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		}
	// 		| MethodMetadata::StorageIsEmpty {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		}
	// 		| MethodMetadata::StorageClear {
	// 			visibility: Visibility::Endpoint(e),
	// 			..
	// 		} => Some(e),
	// 		_ => None,
	// 	}
	// }

	// pub fn has_implementation(&self) -> bool {
	// 	matches!(
	// 		self,
	// 		MethodMetadata::Regular { .. } | MethodMetadata::Callback | MethodMetadata::CallbackRaw
	// 	)
	// }

	// pub fn payable_metadata(&self) -> MethodPayableMetadata {
	// 	match self {
	// 		MethodMetadata::Regular { payable, .. } => payable.clone(),
	// 		MethodMetadata::Callback | MethodMetadata::CallbackRaw => {
	// 			MethodPayableMetadata::AnyToken
	// 		},
	// 		MethodMetadata::StorageGetter { .. } => MethodPayableMetadata::NotPayable,
	// 		MethodMetadata::StorageSetter { .. } => MethodPayableMetadata::NotPayable,
	// 		MethodMetadata::StorageIsEmpty { .. } => MethodPayableMetadata::NotPayable,
	// 		MethodMetadata::StorageClear { .. } => MethodPayableMetadata::NotPayable,
	// 		_ => MethodPayableMetadata::NoMetadata,
	// 	}
	// }
}

// #[derive(Clone, Debug)]
// pub struct Method {
// 	pub docs: Vec<String>,
// 	pub metadata: MethodMetadata,
// 	pub name: syn::Ident,
// 	pub generics: syn::Generics,
// 	pub method_args: Vec<MethodArg>,
// 	pub payment_arg: Option<MethodArg>,
// 	pub token_arg: Option<MethodArg>,
// 	pub output_names: Vec<String>,
// 	pub return_type: syn::ReturnType,
// 	pub body: Option<syn::Block>,
// }







pub fn arg_declarations(method_args: &[MethodArg]) -> Vec<proc_macro2::TokenStream> {
	method_args
		.iter()
		.map(|arg| {
			let pat = &arg.pat;
			let ty = &arg.ty;
			quote! {#pat : #ty }
		})
		.collect()
}

impl Method {
	pub fn generate_sig(&self) -> proc_macro2::TokenStream {
		let method_name = &self.name;
		let generics = &self.generics;
		let generics_where = &self.generics.where_clause;
		let arg_decl = arg_declarations(&self.method_args);
		let ret_tok = match &self.return_type {
			syn::ReturnType::Default => quote! {},
			syn::ReturnType::Type(_, ty) => quote! { -> #ty },
		};
		let result = quote! { fn #method_name #generics ( &self , #(#arg_decl),* ) #ret_tok #generics_where };
		result
	}

	pub fn generate_call_to_method(&self) -> proc_macro2::TokenStream {
		let fn_ident = &self.name;
		let arg_values: Vec<proc_macro2::TokenStream> = self
			.method_args
			.iter()
			.map(|arg| generate_arg_call_name(arg))
			.collect();
		quote! {
			self.#fn_ident (#(#arg_values),*)
		}
	}

	pub fn has_variable_nr_args(&self) -> bool {
		self.method_args
			.iter()
			.any(|arg| matches!(&arg.metadata, ArgMetadata::VarArgs))
	}

	pub fn generate_call_method(&self) -> proc_macro2::TokenStream {
		let call_method_ident = generate_call_method_name(&self.name);
		let call_method_body = self.generate_call_method_body();
		quote! {
			#[inline]
			fn #call_method_ident (&self) {
				#call_method_body
			}
		}
	}

	pub fn generate_call_method_body(&self) -> proc_macro2::TokenStream {
		if self.has_variable_nr_args() {
			self.generate_call_method_body_variable_nr_args()
		} else {
			self.generate_call_method_body_fixed_args()
		}
	}

	pub fn generate_call_method_body_fixed_args(&self) -> proc_macro2::TokenStream {
		let payable_snippet = generate_payable_snippet(self);

		let mut arg_index = -1i32;
		let arg_init_snippets: Vec<proc_macro2::TokenStream> = self
			.method_args
			.iter()
			.map(|arg| match &arg.metadata {
				ArgMetadata::Single => {
					arg_index += 1;
					let pat = &arg.pat;
					let arg_get = generate_load_single_arg(arg, &quote! { #arg_index });
					quote! {
						let #pat = #arg_get;
					}
				},
				ArgMetadata::Payment | ArgMetadata::PaymentToken => quote! {},
				ArgMetadata::VarArgs => {
					panic!("var_args not accepted in function generate_call_method_fixed_args")
				},
				ArgMetadata::AsyncCallResultArg => {
					panic!("async call result arg not allowed here")
				},
			})
			.collect();

		let call = self.generate_call_to_method();
		let body_with_result = generate_body_with_result(&self.return_type, &call);
		let nr_args = arg_index + 1;

		quote! {
			#payable_snippet
			self.api.check_num_arguments(#nr_args);
			#(#arg_init_snippets)*
			#body_with_result
		}
	}

	fn generate_call_method_body_variable_nr_args(&self) -> proc_macro2::TokenStream {
		let payable_snippet = generate_payable_snippet(self);

		let arg_init_snippets: Vec<proc_macro2::TokenStream> = self
			.method_args
			.iter()
			.map(|arg| match &arg.metadata {
				ArgMetadata::Single | ArgMetadata::VarArgs => {
					generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
				},
				ArgMetadata::Payment | ArgMetadata::PaymentToken => quote! {},
				ArgMetadata::AsyncCallResultArg => panic!("async call result arg npt allowed here"),
			})
			.collect();

		let call = self.generate_call_to_method();
		let body_with_result = generate_body_with_result(&self.return_type, &call);

		quote! {
			#payable_snippet

			let mut ___arg_loader = EndpointDynArgLoader::new(self.api.clone());

			#(#arg_init_snippets)*

			___arg_loader.assert_no_more_args();

			#body_with_result
		}
	}
}
