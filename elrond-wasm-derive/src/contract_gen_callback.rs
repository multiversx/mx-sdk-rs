
use super::contract_gen_arg::*;
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
                        let mut arg_index = 0i32; // first argument is the function name

                        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
                            m.method_args
                                .iter()
                                .map(|arg| {
                                    match &arg.metadata {
                                        ArgMetadata::None => {
                                            arg_index += 1;
                                            let pat = &arg.pat;
                                            let arg_get = generate_get_arg_snippet(arg, &quote!{ #arg_index });
                                            quote! {
                                                let #pat = #arg_get; 
                                            }
                                        },
                                        ArgMetadata::Payment =>
                                            panic!("payment args not allowed in callbacks"),
                                        ArgMetadata::Multi(_) =>
                                            panic!("multi-args not allowed in callbacks"),
                                    }
                                })
                                .collect();

                        let fn_ident = &m.name;
                        let fn_name_str = &fn_ident.to_string();
                        let fn_name_literal = array_literal(fn_name_str.as_bytes());
                        let expected_num_args = (m.method_args.len() + 1) as i32;
                        let call = m.generate_call_to_method();

                        let match_arm = quote! {                     
                            #fn_name_literal =>
                            {
                                if nr_args != #expected_num_args {
                                    self.api.signal_error("wrong number of callback arguments");
                                    return;
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
        let nr_args = self.api.get_num_arguments();
        if nr_args == 0 {
            return;
        }
        let cb_name = self.api.get_argument_vec(0i32);
        match cb_name.as_slice() {
            [] => {
                if nr_args != 1i32 {
                    self.api.signal_error("wrong number of callback arguments");
                    return;
                }
            }
            #(#match_arms)*
            other => panic!("No callback function with that name exists in contract.")
        }
    }
}

