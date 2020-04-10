
use super::arg_def::*;
use super::arg_regular::*;
use super::arg_str_deserialize::*;
use super::contract_gen_method::*;
use super::util::*;

pub fn generate_callback_body(methods: &Vec<Method>) -> proc_macro2::TokenStream {
    let raw_decl = find_raw_callback(methods);
    if let Some(raw) = raw_decl {
        generate_callback_body_raw(&raw)
    } else {
        generate_callback_body_regular(methods)
    }
}

fn find_raw_callback(methods: &Vec<Method>) -> Option<Method> {
    methods.iter()
        .find(|m| {
            match m.metadata {
                MethodMetadata::CallbackRaw() => true,
                _ => false
            }
        })
        .map(|m| m.clone())
}

fn generate_callback_body_raw(raw_callback: &Method) -> proc_macro2::TokenStream {
    let fn_ident = &raw_callback.name;
    quote! {
        let nr_args = self.api.get_num_arguments();
        let mut args: Vec<Vec<u8>> = Vec::with_capacity(nr_args as usize);
        for i in 0..nr_args {
            args.push(self.api.get_argument_vec(i));
        }
        self.#fn_ident (args);
    }
}

fn generate_callback_body_regular(methods: &Vec<Method>) -> proc_macro2::TokenStream {
    let match_arms: Vec<proc_macro2::TokenStream> = 
        methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Callback() => {
                        let mut arg_index = -1i32;
                        let mut nr_returned_args = 0i32;

                        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
                            m.method_args
                                .iter()
                                .map(|arg| {
                                    if arg.is_callback_arg {
                                        match &arg.metadata {
                                            ArgMetadata::Single => {
                                                let pat = &arg.pat;
                                                let arg_get = arg_deserialize_next(arg);
                                                quote! {
                                                    let #pat = #arg_get; 
                                                }
                                            },
                                            ArgMetadata::Payment =>
                                                panic!("payment args not allowed in callbacks"),
                                            ArgMetadata::Multi(_) =>
                                                panic!("callback multi args not yet supported"),
                                        }
                                    } else {
                                        nr_returned_args += 1;
                                        match &arg.metadata {
                                            ArgMetadata::Single => {
                                                arg_index += 1;
                                                let pat = &arg.pat;
                                                let arg_get = arg_regular(arg, &quote!{ #arg_index });
                                                quote! {
                                                    let #pat = #arg_get; 
                                                }
                                            },
                                            ArgMetadata::Payment =>
                                                panic!("payment args not allowed in callbacks"),
                                            ArgMetadata::Multi(_) =>
                                                panic!("multi-args not allowed in callbacks"),
                                        }
                                    }
                                })
                                .collect();

                        let fn_ident = &m.name;
                        let fn_name_str = &fn_ident.to_string();
                        let fn_name_literal = array_literal(fn_name_str.as_bytes());
                        let call = m.generate_call_to_method();

                        let match_arm = quote! {                     
                            #fn_name_literal =>
                            {
                                if nr_args != #nr_returned_args {
                                    self.api.signal_error(err_msg::ARG_ASYNC_RETURN_WRONG_NUMBER);
                                }
                                #(#arg_init_snippets)*
                                #call ;
                            },
                        };
                        Some(match_arm)
                    },
                    _ => None
                }
            })
            .collect();
    quote! {
        let cb_data_raw = self.api.storage_load(&self.api.get_tx_hash());
        let cb_data = elrond_wasm::CallData::from_raw_data(cb_data_raw);
        let mut cb_data_deserializer = cb_data.deserializer();
        let cb_name = match cb_data_deserializer.next_raw_bytes() {
            elrond_wasm::DeserializerResult::NoMore => self.api.signal_error(err_msg::ARG_CALLBACK_TOO_FEW), // actually unreachable
            elrond_wasm::DeserializerResult::Err(e) => self.api.signal_error(e), // also unreachable
            elrond_wasm::DeserializerResult::Res(cb_name) => cb_name,
        };
        let nr_args = self.api.get_num_arguments();
        match cb_name {
            [] => {
                if nr_args != 0i32 {
                    self.api.signal_error(err_msg::ARG_ASYNC_RETURN_WRONG_NUMBER);
                }
            }
            #(#match_arms)*
            other => panic!("No callback function with that name exists in contract.")
        }
        match cb_data_deserializer.next_raw_bytes() {
            elrond_wasm::DeserializerResult::NoMore => {
                self.api.storage_store(&self.api.get_tx_hash(), &[]); // cleanup
            },
            _ => {
                self.api.signal_error(err_msg::ARG_CALLBACK_TOO_MANY);
            }
        };
        
    }
}

