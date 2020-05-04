
use super::contract_gen_event::*;
use super::contract_gen_storage::*;
use super::contract_gen_method::*;
use super::contract_gen_callback::*;
use super::util::*;

pub struct Contract {
    pub trait_name: proc_macro2::Ident,
    pub contract_impl_name: syn::Path,
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

    pub fn extract_pub_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                if m.metadata.is_public() {
                    Some(m.generate_sig())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn extract_method_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
        .filter_map(|m| {
            if m.metadata.has_implementation() {
                let body = match m.body {
                    Some(ref mbody) => {
                        let msig = m.generate_sig();
                        quote! { 
                            #msig { 
                                #mbody 
                            } 
                        }
                    }
                    None => quote! {},
                };
                Some(body)
            } else {
                None
            }
        })
        .collect()
    }

    pub fn generate_call_methods(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                if m.metadata.is_public() {
                    Some(m.generate_call_method())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Definitions for methods that get auto-generated implementations: events, getters, setters
    pub fn generate_auto_impl_defs(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match m.metadata {
                    MethodMetadata::Event{ .. } |
                    MethodMetadata::StorageGetter{ .. } |
                    MethodMetadata::StorageSetter{ .. } => {
                        let sig = m.generate_sig();
                        Some(quote! { #sig ; })
                    },
                    _ => None
                }
            })
            .collect()
    }

    /// Implementations for methods that get auto-generated implementations: events, getters, setters
    pub fn generate_auto_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match &m.metadata {
                    MethodMetadata::Event{ identifier } => 
                        Some(generate_event_impl(&m, identifier.clone())),
                    MethodMetadata::StorageGetter{ visibility: _, identifier } =>
                        Some(generate_getter_impl(&m, identifier.clone())),
                    MethodMetadata::StorageSetter{ visibility: _, identifier } =>
                        Some(generate_setter_impl(&m, identifier.clone())),
                    _ => None
                }
            })
            .collect()
    }

    pub fn generate_storage_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                match &m.metadata {
                    MethodMetadata::Event{ identifier } => {
                        Some(generate_event_impl(&m, identifier.clone()))
                    },
                    _ => None
                }
            })
            .collect()
    }

    pub fn generate_endpoints(&self) -> Vec<proc_macro2::TokenStream> {
        self.methods.iter()
            .filter_map(|m| {
                if m.metadata.is_public() {
                    let fn_ident = &m.name;
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
                } else {
                    None
                }
            })
            .collect()
    }
    
    pub fn generate_function_selector_body(&self) -> proc_macro2::TokenStream {
        let match_arms: Vec<proc_macro2::TokenStream> = 
            self.methods.iter()
                .filter_map(|m| {
                    if m.metadata.is_public() {
                        let fn_ident = &m.name;
                        let fn_name_str = &fn_ident.to_string();
                        let call_method_ident = generate_call_method_name(&fn_ident);
                        let match_arm = quote! {                     
                            #fn_name_str =>
                            {
                                self.#call_method_ident();
                            },
                        };
                        Some(match_arm)
                    } else {
                        None
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
        generate_callback_body(&self.methods)
    }
}
