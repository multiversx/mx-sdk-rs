use super::contract_gen_arg::*;
use super::contract_gen_event::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::parse_attr::*;
use super::util::*;

//use super::parse_attr::*;

pub struct Contract {
    pub trait_name: proc_macro2::Ident,
    pub contract_impl_name: proc_macro2::Ident,
    methods: Vec<Method>,
}

impl Contract {
    pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
        let contract_impl_name = extract_struct_name(args);
        //let trait_methods = extract_methods(&contract_trait);

        let methods: Vec<Method> = contract_trait
            .items
            .iter()
            .map(|itm| match itm {
                syn::TraitItem::Method(m) => Method::parse(m),
                _ => panic!("Only methods allowed in contract traits")
            })
            .collect();
        
        Contract {
            trait_name: contract_trait.ident.clone(),
            contract_impl_name: contract_impl_name,
            methods: methods,
        }
    }
}

#[derive(Clone, Debug)]
pub enum MethodMetadata {
    Public(Option<PayableAttribute>),
    Private(),
    Event(Vec<u8>),
    Callback(),
}

#[derive(Clone, Debug)]
pub struct PublicArg {
    pub index: i32,
    pub syn_arg: syn::FnArg,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub metadata: MethodMetadata,
    pub public_args: Vec<PublicArg>,
    pub syn_m: syn::TraitItemMethod,
}

impl Method {
    pub fn parse(m: &syn::TraitItemMethod) -> Method {
        let payable_opt = PayableAttribute::parse(m);
        let private = is_private(m);
        let callback = is_callback_decl(m);
        let event_opt = EventAttribute::parse(m);

        let metadata: MethodMetadata;
        if let Some(event_attr) = event_opt {
            if let Some(_) = payable_opt {
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
            metadata = MethodMetadata::Event(event_attr.identifier);
        } else if callback {
            if let Some(_) = payable_opt {
                panic!("Callback methods cannot be marked payable.");
            }
            if private {
                panic!("Callbacks cannot be marked private, they are private by definition.");
            }
            if m.default == None {
                panic!("Callback methods need an implementation.");
            }
            metadata = MethodMetadata::Callback();
        } else if private {
            if let Some(_) = payable_opt {
                panic!("Private methods cannot be marked payable.");
            }
            if m.default == None {
                panic!("Private methods need an implementation.");
            }
            metadata = MethodMetadata::Private();
        } else {
            if m.default == None {
                panic!("Public methods need an implementation.");
            }
            metadata = MethodMetadata::Public(payable_opt);
        }
        
        let payable_opt: Option<PayableAttribute> = if let MethodMetadata::Public(payable_opt) = &metadata {
            payable_opt.clone()
        } else {
            None
        };
        let public_args = extract_public_args(m, &payable_opt);
        
        Method {
            metadata: metadata.clone(),
            public_args: public_args,
            syn_m: m.clone(),
        }
    }
}

pub fn extract_public_args(m: &syn::TraitItemMethod, payable_opt: &Option<PayableAttribute>) -> Vec<PublicArg> {
    let mut arg_index: isize = -1; // ignore the first argument, which is &self
    m.sig.decl.inputs
        .iter()
        .filter_map(|arg| {
            let arg_opt = match arg {
                syn::FnArg::SelfRef(ref selfref) => {
                    if !selfref.mutability.is_none() || arg_index != -1 {
                        panic!("Trait method must have `&self` as its first argument.");
                    }
                    None
                },
                captured @ syn::FnArg::Captured(_) => {
                    let mut is_payment_arg = false;
                    if let Some(PayableAttribute{ payment_arg: Some(p_attr) }) = &payable_opt {
                        if p_attr == captured {
                            is_payment_arg = true;
                        }
                    }
                    if is_payment_arg {
                        None // do not add payment arg to public args
                    } else {
                        arg_index=arg_index+1;
                        Some(PublicArg{
                            index: arg_index as i32,
                            syn_arg: captured.clone()
                        })
                    }
                },
                other_arg => panic!("Unsupported argument type {:?}, nor captured", other_arg),
            };
            
            arg_opt
        })
        .collect()
}

impl Contract {
    // can extract trait method signatures
    // currently not used
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Public(_) => {
                        let mattrs = &m.syn_m.attrs;
                        let msig = &m.syn_m.sig;
                        let sig = quote! {
                            #(#mattrs)*
                            #msig;
                        };
                        Some(sig)
                    },
                    _ => None
                }
            })
            .collect()
    }

    pub fn extract_method_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
        .filter_map(|m| {
            match m.metadata {
                MethodMetadata::Public(_) | MethodMetadata::Private() | MethodMetadata::Callback() => {
                    let msig = &m.syn_m.sig;
                    let body = match m.syn_m.default {
                        Some(ref mbody) => {
                            quote! { 
                                #msig { 
                                    #mbody 
                                } 
                            }
                        }
                        None => quote! {},
                    };
                    Some(body)
                },
                _ => None
            }
        })
        .collect()
    }
}

impl Method {
    pub fn generate_call_method(&self) -> proc_macro2::TokenStream {
        let msig = &self.syn_m.sig;
        let all_arg_names: Vec<proc_macro2::TokenStream> =  
            msig.decl.inputs
                .iter()
                .filter_map(|arg| generate_arg_call_name(arg))
                .collect();

        let pub_arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            self.public_args
                .iter()
                .map(|arg| generate_arg_init_snippet(arg, 0))
                .collect();

        let nr_args = self.public_args.len() as i32;

        let payable_snippet = generate_payable_snippet(self);

        let fn_ident = &self.syn_m.sig.ident;
        let call_method_ident = generate_call_method_name(&msig.ident);
        
        let call = quote! {
            self.#fn_ident (#(#all_arg_names),*)
        };
        let body_with_result = generate_body_with_result(&msig.decl.output, &call);

        quote! {
            #[inline]
            fn #call_method_ident (&self) {
                #payable_snippet
                if !self.api.check_num_arguments(#nr_args) {
                    return;
                }
                #(#pub_arg_init_snippets)*
                #body_with_result
            }
        }
    }
}

impl Contract {

    pub fn generate_call_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Public(_) => {
                        Some(m.generate_call_method())
                    },
                    _ => None
                }
            })
            .collect()
    }

    pub fn generate_event_defs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Event(_) => {
                        let msig = &m.syn_m.sig;
                        let event_sig = quote! {
                            #msig ;
                        };
                        Some(event_sig)
                    },
                    _ => None
                }
            })
            .collect()
    }

    pub fn generate_event_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
        .filter_map(|m| {
            match m.metadata {
                MethodMetadata::Event(_) => {
                    Some(generate_event_impl(&m.syn_m))
                },
                _ => None
            }
        })
        .collect()
    }

    pub fn generate_endpoints(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Public(_) => {
                        let fn_ident = &m.syn_m.sig.ident;
                        let call_method_ident = generate_call_method_name(&fn_ident);
                        let endpoint = quote! { 
                            #[no_mangle]
                            pub fn #fn_ident ()
                            {
                                let inst = new_arwen_instance();
                                inst.#call_method_ident();
                            }
                        }  ;  
                        Some(endpoint)
                    },
                    _ => None
                }
            })
            .collect()
    }
    
    pub fn generate_function_selector_body(&self) -> proc_macro2::TokenStream {
        let match_arms: Vec<proc_macro2::TokenStream> = 
            self.methods.iter()
                .filter_map(|m| {
                    match m.metadata {
                        MethodMetadata::Public(_) => {
                            let fn_ident = &m.syn_m.sig.ident;
                            let fn_name_str = &fn_ident.to_string();
                            let call_method_ident = generate_call_method_name(&fn_ident);
                            let match_arm = quote! {                     
                                #fn_name_str =>
                                {
                                    self.#call_method_ident();
                                },
                            };
                            Some(match_arm)
                        },
                        _ => None
                    }
                })
                .collect();
        quote! {      
            match fn_name {
                #(#match_arms)*
                other => panic!("No function named `{}` exists in contract.", other)
            }
        }
    }

    pub fn generate_callback_body(&self) -> proc_macro2::TokenStream {
        let match_arms: Vec<proc_macro2::TokenStream> = 
            self.methods.iter()
                .filter_map(|m| {
                    let all_arg_names: Vec<proc_macro2::TokenStream> =  
                        m.syn_m.sig.decl.inputs
                            .iter()
                            .filter_map(|arg| generate_arg_call_name(arg))
                            .collect();

                    let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
                        m.public_args
                            .iter()
                            .map(|arg| generate_arg_init_snippet(arg, 1))
                            .collect();

                    match m.metadata {
                        MethodMetadata::Callback() => {
                            let fn_ident = &m.syn_m.sig.ident;
                            let fn_name_str = &fn_ident.to_string();
                            let fn_name_literal = array_literal(fn_name_str.as_bytes());
                            let expected_num_args = (m.public_args.len() + 1) as i32;
                            let match_arm = quote! {                     
                                #fn_name_literal =>
                                {
                                    if nr_args != #expected_num_args {
                                        self.api.signal_error("wrong number of callback arguments");
                                        return;
                                    }
                                    #(#arg_init_snippets)*
                                    self.#fn_ident (#(#all_arg_names),*);
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
}
