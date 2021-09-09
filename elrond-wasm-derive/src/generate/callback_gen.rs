use super::{
    arg_regular::*,
    method_call_gen::{
        generate_body_with_result, generate_call_method_body, generate_call_to_method_expr,
    },
    payable_gen::*,
    util::*,
};
use crate::model::{ContractTrait, Method, PublicRole, Supertrait};

pub fn generate_callback_selector_and_main(
    contract: &ContractTrait,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let raw_decl = find_raw_callback(&contract.methods);
    if let Some(raw) = raw_decl {
        let as_call_method = generate_call_method_body(&raw);
        let cb_selector_body = quote! {
            #as_call_method
            elrond_wasm::types::CallbackSelectorResult::Processed
        };
        let cb_main_body = quote! {
            let _ = self.callback_selector(elrond_wasm::hex_call_data::HexCallDataDeserializer::new(&[]));
        };
        (cb_selector_body, cb_main_body)
    } else {
        let match_arms: Vec<proc_macro2::TokenStream> = match_arms(contract.methods.as_slice());
        let module_calls: Vec<proc_macro2::TokenStream> =
            module_calls(contract.supertraits.as_slice());
        if match_arms.is_empty() && module_calls.is_empty() {
            let cb_selector_body = quote! {
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
            };
            let cb_main_body = quote! {};
            (cb_selector_body, cb_main_body)
        } else {
            let cb_selector_body = callback_selector_body(match_arms, module_calls);
            let cb_main_body = quote! {
                let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.raw_vm_api());
                let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(&self.get_storage_raw(), &___tx_hash___.as_bytes());
                elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(&self.get_storage_raw(), &___tx_hash___.as_bytes(), &[]); // cleanup
                let mut ___cb_data_deserializer___ = elrond_wasm::hex_call_data::HexCallDataDeserializer::new(___cb_data_raw___.as_slice());
                if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                    self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)	{
                    elrond_wasm::api::ErrorApi::signal_error(&self.raw_vm_api(), err_msg::CALLBACK_BAD_FUNC);
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
        let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.raw_vm_api());
        match ___cb_data_deserializer___.get_func_name() {
            [] => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            #(#match_arms)*
            _ => {},
        }
        #(#module_calls)*
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
    }
}

fn match_arms(methods: &[Method]) -> Vec<proc_macro2::TokenStream> {
    methods
        .iter()
        .filter_map(|m| {
            if let PublicRole::Callback(callback) = &m.public_role {
                let payable_snippet = generate_payable_snippet(m);
                let mut has_call_result = false;
                let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
                    .method_args
                    .iter()
                    .map(|arg| {
                        if arg.metadata.payment.is_payment_arg() {
                            quote! {}
                        } else if arg.metadata.callback_call_result {
                            has_call_result = true;

                            // Should be an AsyncCallResult argument that wraps what comes from the async call.
                            // But in principle, one can express it it any way.
                            generate_load_dyn_arg(arg, &quote! { &mut ___call_result_loader___ })
                        } else {
                            // callback args, loaded from storage via the tx hash
                            generate_load_dyn_arg(arg, &quote! { &mut ___cb_closure_loader___ })
                        }
                    })
                    .collect();

                let callback_name_str = &callback.callback_name.to_string();
                let callback_name_literal = array_literal(callback_name_str.as_bytes());
                let call = generate_call_to_method_expr(m);
                let call_result_assert_no_more_args = if has_call_result {
                    quote! {
                        ___call_result_loader___.assert_no_more_args();
                    }
                } else {
                    quote! {}
                };
                let body_with_result = generate_body_with_result(&m.return_type, &call);

                let match_arm = quote! {
                    #callback_name_literal =>
                    {
                        #payable_snippet
                        let mut ___cb_closure_loader___ = CallDataArgLoader::new(
                            ___cb_data_deserializer___,
                            self.raw_vm_api(),
                        );
                        #(#arg_init_snippets)*
                        ___cb_closure_loader___.assert_no_more_args();
                        #call_result_assert_no_more_args
                        #body_with_result ;
                        return elrond_wasm::types::CallbackSelectorResult::Processed;
                    },
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
				match #module_path EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
					elrond_wasm::types::CallbackSelectorResult::Processed => {
						return elrond_wasm::types::CallbackSelectorResult::Processed;
					},
					elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
						___cb_data_deserializer___ = recovered_deser;
					},
				}
			}
		})
		.collect()
}
