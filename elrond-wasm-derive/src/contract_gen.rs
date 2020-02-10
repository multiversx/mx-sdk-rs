use super::contract_gen_arg::*;
use super::contract_gen_event::*;
use super::contract_gen_finish::*;
use super::contract_gen_payable::*;
use super::util::*;

pub static ATTR_PRIVATE: &str = "private";
pub static ATTR_EVENT: &str = "event";

pub struct Contract {
    pub trait_name: proc_macro2::Ident,
    pub contract_impl_name: proc_macro2::Ident,
    implemented_methods: Vec<syn::TraitItemMethod>,
    public_methods: Vec<syn::TraitItemMethod>,
    event_methods: Vec<syn::TraitItemMethod>,
}

impl Contract {
    pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
        let contract_impl_name = extract_struct_name(args);
        let trait_methods = extract_methods(&contract_trait);
        let implemented_methods = extract_implemented_methods(&trait_methods);
        let public_methods = extract_public_methods(&trait_methods);
        let event_methods = extract_event_methods(&trait_methods);
        Contract {
            trait_name: contract_trait.ident.clone(),
            contract_impl_name: contract_impl_name,
            implemented_methods: implemented_methods,
            public_methods: public_methods,
            event_methods: event_methods,
        }
    }
}

fn extract_public_methods(trait_methods: &Vec<syn::TraitItemMethod>) -> Vec<syn::TraitItemMethod> {
    trait_methods
        .iter()
        .filter(|m| !has_attribute(&m.attrs, ATTR_EVENT) && !has_attribute(&m.attrs, ATTR_PRIVATE))
        .cloned()
        .collect()
}

fn extract_implemented_methods(trait_methods: &Vec<syn::TraitItemMethod>) -> Vec<syn::TraitItemMethod> {
    trait_methods
        .iter()
        .filter(|m| !has_attribute(&m.attrs, ATTR_EVENT))
        .cloned()
        .collect()
}

fn extract_event_methods(trait_methods: &Vec<syn::TraitItemMethod>) -> Vec<syn::TraitItemMethod> {
    trait_methods
        .iter()
        .filter(|m| has_attribute(&m.attrs, ATTR_EVENT))
        .cloned()
        .collect()
}

impl Contract {
    // can extract trait method signatures
    // currently not used
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.public_methods.iter().map(|m| {
            let mattrs = &m.attrs;
            let msig = &m.sig;
            let sig = quote! {
                #(#mattrs)*
                #msig;
            };
            sig
        }).collect()
    }

    pub fn extract_method_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.implemented_methods.iter().map(|m| {
            let msig = &m.sig;
            let body = match m.default {
                Some(ref mbody) => {
                    quote! { 
                        #msig { 
                            #mbody 
                        } 
                    }
                }
                None => quote! {},
            };
            body        
        }).collect()
    }
}

impl Contract {
    fn generate_call_method(&self, m: &syn::TraitItemMethod) -> proc_macro2::TokenStream {
        let msig = &m.sig;
        let mut arg_index: isize = -1; // ignore the first argument, which is &self
        let arg_names: Vec<proc_macro2::TokenStream> =  
            msig.decl.inputs
                .iter()
                .filter_map(|arg| {
                    let call_name = generate_arg_call_name(arg, arg_index);
                    arg_index=arg_index+1;
                    call_name
                })
                .collect();

        arg_index = -1;
        let arg_init_snippets: Vec<proc_macro2::TokenStream> = 
            msig.decl.inputs
                .iter()
                .map(|arg| {
                    let snippet = generate_arg_init_snippet(arg, arg_index);
                    arg_index=arg_index+1;
                    snippet
                })
                .collect();

        let nr_args = (msig.decl.inputs.len() - 1) as i32; // ignore the first argument, which is &self

        let payable_snippet = generate_payable_snippet(&m);

        if m.default == None {
            panic!("Methods without implementation (other than events) not allowed in contract trait");
        }

        let fn_ident = &m.sig.ident;
        let call_method_ident = generate_call_method_name(&msig.ident);
        
        let call = quote! {
            self.#fn_ident (#(#arg_names),*)
        };
        let body_with_result = generate_body_with_result(&msig.decl.output, &call);

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

    pub fn generate_call_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.public_methods.iter().map(|m| 
            self.generate_call_method(m)
        ).collect()
    }

    pub fn generate_event_defs(&self) -> Vec<proc_macro2::TokenStream> {
        self.event_methods.iter().map(|m| {
            let msig = &m.sig;
            quote! {
                #msig ;
            }
        }).collect()
    }

    pub fn generate_event_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.event_methods.iter().map(|m|
            generate_event_impl(m)
        ).collect()
    }

    pub fn generate_endpoints(&self) -> Vec<proc_macro2::TokenStream> {
        self.public_methods.iter().map(|m| {
            let fn_ident = &m.sig.ident;
            let call_method_ident = generate_call_method_name(&m.sig.ident);
            quote! { 
                #[no_mangle]
                pub fn #fn_ident ()
                {
                    let mut inst = new_arwen_instance();
                    inst.#call_method_ident();
                }
            }
        }).collect()
    }
    
    pub fn generate_function_selector_body(&self) -> proc_macro2::TokenStream {
        let match_arms: Vec<proc_macro2::TokenStream> = 
            self.public_methods.iter().map(|m| {
                let fn_name_str = &m.sig.ident.to_string();
                let call_method_ident = generate_call_method_name(&m.sig.ident);
                quote! {                     
                    #fn_name_str =>
                    {
                        self.#call_method_ident();
                    },
                }
            }).collect();
        quote! {      
            match fn_name {
                #(#match_arms)*
                other => panic!("No function named `{}` exists in contract.", other)
            }
        }
    }
}
