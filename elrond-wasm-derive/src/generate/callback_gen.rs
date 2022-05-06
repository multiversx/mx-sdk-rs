use super::{
    method_call_gen::{
        generate_body_with_result, generate_endpoint_call_method_body, generate_call_to_method_expr,
    },
    payable_gen::*,
    util::*,
};
use crate::{
    generate::method_call_gen::generate_arg_nested_tuples,
    model::{ContractTrait, Method, PublicRole, Supertrait},
};

/// Callback name max length is checked during derive,
/// so as not to burden the contract at runtime.
pub const CALLBACK_NAME_MAX_LENGTH: usize = 32;

pub fn generate_callback_selector_and_main(
    contract: &ContractTrait,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let raw_decl = find_raw_callback(&contract.methods);
    if let Some(raw) = raw_decl {
        let as_call_method = generate_endpoint_call_method_body(&raw);
        let cb_selector_body = quote! {
            #as_call_method
            elrond_wasm::types::CallbackSelectorResult::Processed
        };
        let cb_main_body = quote! {
            let _ = self::EndpointWrappers::callback_selector(
                self,
                elrond_wasm::types::CallbackClosureForDeser::new_empty(),
            );
        };
        (cb_selector_body, cb_main_body)
    } else {
        let match_arms: Vec<proc_macro2::TokenStream> = match_arms(contract.methods.as_slice());
        let module_calls: Vec<proc_macro2::TokenStream> =
            module_calls(contract.supertraits.as_slice());
        if match_arms.is_empty() && module_calls.is_empty() {
            let cb_selector_body = quote! {
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
            };
            let cb_main_body = quote! {};
            (cb_selector_body, cb_main_body)
        } else {
            let cb_selector_body = callback_selector_body(match_arms, module_calls);
            let cb_main_body = quote! {
                if let Some(___cb_closure___) = elrond_wasm::types::CallbackClosureForDeser::storage_load_and_clear::<Self::Api>() {
                    if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                        self::EndpointWrappers::callback_selector(self, ___cb_closure___)	{
                        elrond_wasm::api::ErrorApiImpl::signal_error(
                            &Self::Api::error_api_impl(),
                            err_msg::CALLBACK_BAD_FUNC,
                        );
                    }
                }
            };
            (cb_selector_body, cb_main_body)
        }
    }
}

fn find_raw_callback(methods: &[Method]) -> Option<Method> {
    methods
        .iter()
        .find(|m| matches!(m.public_role, PublicRole::CallbackRaw))
        .cloned()
}

fn callback_selector_body(
    match_arms: Vec<proc_macro2::TokenStream>,
    module_calls: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        let ___cb_closure_matcher___ = ___cb_closure___.matcher::<#CALLBACK_NAME_MAX_LENGTH>();
        if ___cb_closure_matcher___.matches_empty() {
            return elrond_wasm::types::CallbackSelectorResult::Processed;
        }
        #(#match_arms)*
        #(#module_calls)*
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
    }
}

fn load_call_result_args_snippet(m: &Method) -> proc_macro2::TokenStream {
    // if no `#[call_result]` present, ignore altogether
    let has_call_result = m
        .method_args
        .iter()
        .any(|arg| arg.metadata.callback_call_result);
    if !has_call_result {
        return quote! {};
    }

    let (call_result_var_names, call_result_var_types, call_result_var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| {
            arg.metadata.callback_call_result
        });
    quote! {
        let #call_result_var_names = elrond_wasm::io::load_endpoint_args::<Self::Api,#call_result_var_types>(
            #call_result_var_names_str,
        );
    }
}

fn load_cb_closure_args_snippet(m: &Method) -> proc_macro2::TokenStream {
    let (closure_var_names, closure_var_types, closure_var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| {
            arg.is_endpoint_arg() && !arg.metadata.callback_call_result
        });
    quote! {
        let #closure_var_names = elrond_wasm::io::load_multi_args_custom_loader::<Self::Api, _, #closure_var_types>(
            ___cb_closure___.into_arg_loader(),
            #closure_var_names_str,
        );
    }
}

fn match_arms(methods: &[Method]) -> Vec<proc_macro2::TokenStream> {
    methods
        .iter()
        .filter_map(|m| {
            if let PublicRole::Callback(callback) = &m.public_role {
                let payable_snippet = generate_payable_snippet(m);
                let callback_name_str = &callback.callback_name.to_string();
                assert!(
                    callback_name_str.len() <= CALLBACK_NAME_MAX_LENGTH,
                    "Callback name `{}` is too long, it cannot exceed {} characters",
                    callback_name_str,
                    CALLBACK_NAME_MAX_LENGTH
                );
                let callback_name_literal = byte_str_literal(callback_name_str.as_bytes());
                let load_call_result_args = load_call_result_args_snippet(m);
                let load_cb_closure_args = load_cb_closure_args_snippet(m);
                let call = generate_call_to_method_expr(m);
                let body_with_result = generate_body_with_result(&m.return_type, &call);

                let match_arm = quote! {
                    else if ___cb_closure_matcher___.name_matches(#callback_name_literal) {
                        #payable_snippet
                        #load_call_result_args
                        #load_cb_closure_args
                        #body_with_result ;
                        return elrond_wasm::types::CallbackSelectorResult::Processed;
                    }
                };
                Some(match_arm)
            } else {
                None
            }
        })
        .collect()
}

pub fn module_calls(supertraits: &[Supertrait]) -> Vec<proc_macro2::TokenStream> {
    supertraits
		.iter()
		.map(|supertrait| {
			let module_path = &supertrait.module_path;
			quote! {
				match #module_path EndpointWrappers::callback_selector(self, ___cb_closure___) {
					elrond_wasm::types::CallbackSelectorResult::Processed => {
						return elrond_wasm::types::CallbackSelectorResult::Processed;
					},
					elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_cb_closure) => {
						___cb_closure___ = recovered_cb_closure;
					},
				}
			}
		})
		.collect()
}
