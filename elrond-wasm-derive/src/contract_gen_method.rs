use super::contract_gen_arg::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::parse_attr::*;
use super::util::*;
use super::reserved;

#[derive(Clone, Debug)]
pub enum MethodMetadata {
    Public(PublicMethodMetadata),
    Private(),
    Event(Vec<u8>),
    Callback(),
    CallbackRaw(),
}

#[derive(Clone, Debug)]
pub struct PublicMethodMetadata {
    pub payable: bool,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub metadata: MethodMetadata,
    pub name: syn::Ident,
    pub method_args: Vec<MethodArg>,
    pub return_type: syn::ReturnType,
    pub body: Option<syn::Block>,
}

fn extract_metadata(m: &syn::TraitItemMethod) -> MethodMetadata {
    let payable = is_payable(m);
    let private = is_private(m);
    let callback = is_callback_decl(m);
    let callback_raw = is_callback_raw_decl(m);
    let event_opt = EventAttribute::parse(m);

    if let Some(event_attr) = event_opt {
        if payable {
            panic!("Events cannot be payable.");
        }
        if private {
            panic!("Events cannot be marked private, they are private by definition.");
        }
        if callback {
            panic!("Events cannot be callbacks.");
        }
        if let Some(_) = m.default {
            panic!("Events cannot have provided implementations in the trait.");
        }
        MethodMetadata::Event(event_attr.identifier)
    } else if callback || callback_raw {
        if payable {
            panic!("Callback methods cannot be marked payable.");
        }
        if private {
            panic!("Callbacks cannot be marked private, they are private by definition.");
        }
        if m.default == None {
            panic!("Callback methods need an implementation.");
        }
        if callback && callback_raw {
            panic!("It is either the default callback, or regular callback, not both.");
        }
        if callback_raw {
            MethodMetadata::CallbackRaw()
        } else {
            MethodMetadata::Callback()
        }
    } else if private {
        if payable {
            panic!("Private methods cannot be marked payable.");
        }
        if m.default == None {
            panic!("Private methods need an implementation.");
        }
        MethodMetadata::Private()
    } else {
        if m.default == None {
            panic!("Public methods need an implementation.");
        }
        let fn_name_str = &m.sig.ident.to_string();
        if reserved::is_reserved(fn_name_str) {
            panic!("Cannot declare public method with name '{}', because that name is reserved by the Arwen API.", fn_name_str);
        }

        MethodMetadata::Public(PublicMethodMetadata{
            payable: payable,
        })
    }
}

impl Method {
    pub fn parse(m: &syn::TraitItemMethod) -> Method {
        let metadata = extract_metadata(m);
        let method_args = extract_method_args(m, is_payable(m));
        Method {
            metadata: metadata,
            name: m.sig.ident.clone(),
            method_args: method_args,
            return_type: m.sig.output.clone(),
            body: m.default.clone(),
        }
    }
}

pub fn arg_declarations(method_args: &Vec<MethodArg>) -> Vec<proc_macro2::TokenStream>  {
    method_args
        .iter()
        .map(|arg| {
            let pat = &arg.pat;
            let ty = &arg.ty;
            quote!{#pat : #ty }
        })
        .collect()
}

impl Method {
    pub fn generate_sig(&self) -> proc_macro2::TokenStream {
        let method_name = &self.name;
        let arg_decl = arg_declarations(&self.method_args);
        let ret_tok = match &self.return_type {
            syn::ReturnType::Default => quote!{},
            syn::ReturnType::Type(_, ty) => quote!{ -> #ty },
        };
        let result = quote!{ fn #method_name ( &self , #(#arg_decl),* ) #ret_tok };
        result
    }

    pub fn generate_call_to_method(&self) -> proc_macro2::TokenStream {
        let fn_ident = &self.name;
        let arg_values: Vec<proc_macro2::TokenStream> = self.method_args
            .iter()
            .map(|arg| generate_arg_call_name(arg))
            .collect();
        quote! {
            self.#fn_ident (#(#arg_values),*)
        }
    }

    pub fn generate_call_method(&self) -> proc_macro2::TokenStream {
        let has_variable_nr_args = 
            self.method_args.iter()
                .any(|arg| {
                    match &arg.metadata {
                        ArgMetadata::Multi(_) => true,
                        _ => false,
                    }
                });
        if has_variable_nr_args {
            self.generate_call_method_variable_nr_args()
        } else {
            self.generate_call_method_fixed_args()
        }
    }

    pub fn generate_call_method_fixed_args(&self) -> proc_macro2::TokenStream {
        let payable_snippet = generate_payable_snippet(self);

        let mut arg_index = -1i32;
        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.method_args
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
                            generate_payment_snippet(arg), // #[payment]
                        ArgMetadata::Multi(_) =>
                            panic!("multi-args not accepted in function generate_call_method_fixed_args"),
                    }
                })
                .collect();

        let call_method_ident = generate_call_method_name(&self.name);
        let call = self.generate_call_to_method();
        let body_with_result = generate_body_with_result(&self.return_type, &call);
        let nr_args = arg_index + 1;

        quote! {
            #[inline]
            fn #call_method_ident (&self) {
                #payable_snippet
                if !self.api.check_num_arguments(#nr_args) {
                    return;
                }
                #(#arg_init_snippets)*
                #body_with_result
            }
        }
    }


    fn generate_call_method_variable_nr_args(&self) -> proc_macro2::TokenStream {
        let payable_snippet = generate_payable_snippet(self);

        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.method_args
                .iter()
                .map(|arg| {
                    match &arg.metadata {
                        ArgMetadata::None => {
                            let pat = &arg.pat;
                            let arg_get = generate_get_arg_snippet(arg, &quote!{ ___current_arg });
                            quote! {
                                if ___current_arg >= ___nr_args {
                                    self.api.signal_error("wrong number of arguments");
                                }
                                let #pat = #arg_get;
                                ___current_arg += 1;
                            }
                        },
                        ArgMetadata::Payment => generate_payment_snippet(arg), // #[payment]
                        ArgMetadata::Multi(multi_attr) => { // #[multi(...)]
                            let pat = &arg.pat;
                            let count_expr = &multi_attr.count_expr; // TODO: parse count_expr and make sure it is a an expression in parantheses
                            let push_snippet = generate_multi_arg_push_snippet(&arg, &quote!{ ___current_arg });
                            quote! {
                                let mut #pat = Vec::with_capacity #count_expr ;
                                for _ in 0..#pat.capacity() {
                                    if ___current_arg >= ___nr_args {
                                        self.api.signal_error("wrong number of arguments");
                                    }
                                    #push_snippet
                                    ___current_arg += 1;
                                }
                            }
                        }
                    }
                })
                .collect();

        let call_method_ident = generate_call_method_name(&self.name);
        let call = self.generate_call_to_method();
        let body_with_result = generate_body_with_result(&self.return_type, &call);

        quote! {
            #[inline]
            fn #call_method_ident (&self) {
                #payable_snippet

                let ___nr_args = self.api.get_num_arguments();
                let mut ___current_arg = 0i32;

                #(#arg_init_snippets)*

                match ___nr_args - ___current_arg {
                    0 => {},
                    1 => {
                        let callback_name_arg = self.api.get_argument_vec(___nr_args - 1);
                        self.api.finish_slice_u8(&callback_name_arg.as_slice()); // callback method argument
                    },
                    _ => {
                        self.api.signal_error("wrong number of arguments");
                    }
                }

                #body_with_result
            }
        }
    }

}