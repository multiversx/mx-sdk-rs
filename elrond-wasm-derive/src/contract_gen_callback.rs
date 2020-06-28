
use super::arg_def::*;
use super::arg_regular::*;
// use super::arg_str_deserialize::*;
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
                MethodMetadata::CallbackRaw => true,
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
                    MethodMetadata::Callback => {
                        // let mut nr_regular_args = 0i32;

                        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
                            m.method_args
                                .iter()
                                .map(|arg| {
                                    if arg.is_callback_arg {
                                        // callback args, loaded from storage via the tx hash
                                        match &arg.metadata {
                                            ArgMetadata::Single => {
                                                // let pat = &arg.pat;
                                                // let arg_get = arg_deserialize_next(
                                                //     &quote!{ cb_data_deserializer },
                                                //     arg);
                                                // quote! {
                                                //     let #pat = #arg_get;
                                                // }
                                                dyn_endpoint_args_init(arg,
                                                    &quote! { &mut ___cb_arg_loader },
                                                    &quote! { &___err_handler })
                                            },
                                            ArgMetadata::Payment =>
                                                panic!("payment args not allowed in callbacks"),
                                            ArgMetadata::Multi(_) =>
                                                panic!("callback multi args not yet supported"),
                                            ArgMetadata::VarArgs =>
                                                panic!("callback var_args not yet supported"),
                                        }
                                    } else {
                                        // AsyncCallResult argument, wraps what comes from the async call
                                        // nr_regular_args += 1;
                                        
                                        // let arg_index_expr = quote!{ ___async_res_arg };
                                        // let nr_args_expr = quote! { ___nr_args };

                                        match &arg.metadata {
                                            ArgMetadata::Single | ArgMetadata::VarArgs => {
                                                dyn_endpoint_args_init(arg,
                                                    &quote! { &mut ___arg_loader },
                                                    &quote! { &___err_handler })
                                            },
                                            // ArgMetadata::Single => {
                                            //     let pat = &arg.pat;
                                            //     let arg_get = arg_regular_callback_new(arg, &arg_index_expr, &nr_args_expr);
                                            //     quote! {
                                            //         let #pat = #arg_get;
                                            //     }
                                            // },
                                            ArgMetadata::Payment =>
                                                panic!("payment args not allowed in callbacks"),
                                            ArgMetadata::Multi(_) =>
                                                panic!("multi args not allowed in callbacks"),
                                            // ArgMetadata::VarArgs =>
                                            //     panic!("var_args annotation not allowed in callbacks, callbacks always have variable number of arguments"),
                                        }
                                    }
                                })
                                .collect();

                        // if nr_regular_args != 1 {
                        //     panic!("Callback method exactly 1 AsyncCallResult regular arg.");
                        // }

                        let fn_ident = &m.name;
                        let fn_name_str = &fn_ident.to_string();
                        let fn_name_literal = array_literal(fn_name_str.as_bytes());
                        let call = m.generate_call_to_method();

                        let match_arm = quote! {                     
                            #fn_name_literal =>
                            {
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
    if match_arms.len() == 0 {
        quote! {
            self.api.signal_error(err_msg::CALLBACK_NONE)
        }
    } else {
        quote! {
            let cb_data_raw = self.api.storage_load(&self.api.get_tx_hash().as_ref());
            let mut cb_data_deserializer = elrond_wasm::call_data::CallDataDeserializer::new(cb_data_raw.as_slice());
            // let ___nr_args = self.api.get_num_arguments();
            // if ___nr_args == 0 {
            //     self.api.signal_error(err_msg::ARG_ASYNC_RETURN_WRONG_NUMBER);
            // }

            let mut ___cb_arg_loader = DynEndpointArgLoader::new(&self.api);
            let mut ___arg_loader = DynEndpointArgLoader::new(&self.api);
            let ___err_handler = DynEndpointErrHandler::new(&self.api);

            match cb_data_deserializer.get_func_name() {
                [] => {}
                #(#match_arms)*
                other => self.api.signal_error(err_msg::CALLBACK_BAD_FUNC)
            }
            if cb_data_deserializer.has_next() {
                self.api.signal_error(err_msg::ARG_CALLBACK_TOO_MANY);
            }

            // cleanup
            self.api.storage_store(&self.api.get_tx_hash().as_ref(), &[]); 
        }
    }
}

