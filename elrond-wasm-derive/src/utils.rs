
macro_rules! format_ident {
    ($ident:expr, $fstr:expr) => {
        syn::Ident::new(&format!($fstr, $ident), $ident.span())
    };
}

pub struct Contract {
    pub trait_name: proc_macro2::Ident,
    pub struct_name: proc_macro2::Ident,
    pub debugger_name: proc_macro2::Ident,
    trait_methods: Vec<syn::TraitItemMethod>,
}

impl Contract {
    pub fn new(contract_trait: &syn::ItemTrait) -> Self {
        let trait_name =  format_ident!(contract_trait.ident, "{}");
        let struct_name = format_ident!(contract_trait.ident, "{}Inst");
        let debugger_name = format_ident!(contract_trait.ident, "{}Debug");
        let trait_methods = extract_methods(&contract_trait);
        Contract {
            trait_name: trait_name,
            struct_name: struct_name,
            debugger_name: debugger_name,
            trait_methods: trait_methods,
        }
    }
}

fn extract_methods(contract_trait: &syn::ItemTrait) -> Vec<syn::TraitItemMethod> {
    contract_trait
        .items
        .iter()
        .filter_map(|itm| match itm {
            syn::TraitItem::Method(m) => {
                let msig = &m.sig;
                let bad_self_ref = format!(
                    "ABI function `{}` must have `&self` as its first argument.",
                    msig.ident.to_string()
                );
                match msig.decl.inputs[0] {
                    syn::FnArg::SelfRef(ref selfref) => {
                        if !selfref.mutability.is_none() {
                            panic!(bad_self_ref)
                        }
                    }
                    _ => panic!(bad_self_ref),
                }

                Some(m.clone())
            }
            _ => None,
        }).collect()
}

impl Contract {
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
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
        self.trait_methods.iter().map(|m| {
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

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| {
		if let Some(first_seg) = attr.path.segments.first() {
			return first_seg.value().ident == name
		};
		false
	})
}

fn generate_arg_call_name(arg: &syn::FnArg, arg_index: isize) -> Option<proc_macro2::TokenStream> {
    match arg {
        syn::FnArg::SelfRef(ref selfref) => {
            if !selfref.mutability.is_none() || arg_index != -1 {
                panic!("ABI function must have `&self` as its first argument.");
            }
            None
        },
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            match ty {                
                syn::Type::Path(_) => Some(quote!{ #pat }),
                syn::Type::Reference(_) => Some(quote!{ &#pat }),
                other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name", other_arg),
            }            
        },
        other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name, neither self, nor captured", other_arg)
    }
}

fn generate_call_method_name(method_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    let call_method_name = format_ident!(method_ident, "call_{}");
    call_method_name
}

fn generate_arg_init_snippet(arg: &syn::FnArg, arg_index: isize) -> proc_macro2::TokenStream {
    match arg {
        syn::FnArg::SelfRef(ref selfref) => {
            if !selfref.mutability.is_none() || arg_index != -1 {
                panic!("ABI function must have `&self` as its first argument.");
            }
            quote!{}
        },
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            let arg_index_i32 = arg_index as i32;
            match ty {                
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                    match type_str.as_str() {
                        "Address" =>
                            quote!{
                                let #pat: Address = self.api.get_argument_address(#arg_index_i32);
                            },
                        "BigInt" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int(#arg_index_i32);
                            },
                        "BI" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int(#arg_index_i32);
                            },
                        "i64" =>
                            quote!{
                                let #pat: i64 = self.api.get_argument_i64(#arg_index_i32);
                            },
                        other_stype_str => {
                            panic!("Unsupported argument type {:?} for arg init snippet", other_stype_str)
                        }
                    }
                },             
                syn::Type::Reference(type_reference) => {
                    if type_reference.mutability != None {
                        panic!("Mutable references not supported as contract method arguments");
                    }
                    match &*type_reference.elem {
                        syn::Type::Path(type_path) => {
                            let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                            match type_str.as_str() {
                                "BigInt" =>
                                    quote!{
                                        let #pat: BigInt = self.api.get_argument_big_int(#arg_index_i32);
                                    },
                                "BI" =>
                                    quote!{
                                        let #pat = self.api.get_argument_big_int(#arg_index_i32);
                                    },
                                other_stype_str => {
                                    panic!("Unsupported reference argument type: {:?}", other_stype_str)
                                }
                            }
                        },
                        _ => {
                            panic!("Unsupported reference argument type: {:?}", type_reference)
                        }
                    }
                    
                },
                other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
			}
        }
        other_arg => panic!("Unsupported argument type: {:?}, not captured", other_arg)
    }
}

fn generate_result_finish_snippet(result_name: &str, ty: &syn::Type) -> proc_macro2::TokenStream {
    let result_ident = syn::Ident::new(result_name, proc_macro2::Span::call_site());
    match ty {                
        syn::Type::Path(type_path) => {
            let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
            match type_str.as_str() {
                "Address" =>
                    quote!{
                        self.api.finish(&#result_ident[0], 32);
                    },
                "BigInt" =>
                    quote!{
                        self.api.finish_big_int(#result_ident);
                    },
                "BI" =>
                    quote!{
                        self.api.finish_big_int(#result_ident);
                    },
                "i64" =>
                    quote!{
                        self.api.finish_i64(#result_ident);
                    },
                "bool" =>
                    quote!{
                        self.api.finish_i64( if #result_ident { 1i64 } else { 0i64 });
                    },
                other_stype_str => {
                    panic!("Unsupported return type: {:?}", other_stype_str)
                }
            }
        },
        other_type => panic!("Unsupported return type: {:#?}, not a path", other_type)
    }
}

fn generate_body_with_result(return_type: &syn::ReturnType, mbody: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!{#mbody;},
        syn::ReturnType::Type(_, ty) => {
            match *ty {
                syn::Type::Tuple(_) => {
                    panic!("Tuple result types not yet supported")
                },
                other_ty => {
                    let finish = generate_result_finish_snippet("result", &other_ty);
                    quote!{
                        let result = { #mbody };
                        #finish
                    }
                }
            }
        },
    }
}

fn generate_payable_snippet(m: &syn::TraitItemMethod) -> proc_macro2::TokenStream {
    let payable = has_attribute(&m.attrs, "payable");
    if payable {
        quote!{}
    } else {
        quote!{
            if !self.api.check_not_payable() {
                return;
            }
        }
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
            panic!("Methods without implementation not allowed in contract trait");
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
        self.trait_methods.iter().map(|m| 
            self.generate_call_method(m)
        ).collect()
    }

    pub fn generate_endpoints(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
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
            self.trait_methods.iter().map(|m| {
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
