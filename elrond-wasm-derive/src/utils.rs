
macro_rules! format_ident {
    ($ident:expr, $fstr:expr) => {
        syn::Ident::new(&format!($fstr, $ident), $ident.span())
    };
}

static ATTR_PRIVATE: &str = "private";
static ATTR_EVENT: &str = "event";

pub struct Contract {
    pub trait_name: proc_macro2::Ident,
    pub struct_name: proc_macro2::Ident,
    pub debugger_name: proc_macro2::Ident,
    implemented_methods: Vec<syn::TraitItemMethod>,
    public_methods: Vec<syn::TraitItemMethod>,
    event_methods: Vec<syn::TraitItemMethod>,
}

impl Contract {
    pub fn new(contract_trait: &syn::ItemTrait) -> Self {
        let trait_name =  format_ident!(contract_trait.ident, "{}");
        let struct_name = format_ident!(contract_trait.ident, "{}Inst");
        let debugger_name = format_ident!(contract_trait.ident, "{}Debug");
        let trait_methods = extract_methods(&contract_trait);
        let implemented_methods = extract_implemented_methods(&trait_methods);
        let public_methods = extract_public_methods(&trait_methods);
        let event_methods = extract_event_methods(&trait_methods);
        Contract {
            trait_name: trait_name,
            struct_name: struct_name,
            debugger_name: debugger_name,
            implemented_methods: implemented_methods,
            public_methods: public_methods,
            event_methods: event_methods,
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

fn has_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
			return first_seg.value().ident == name
		};
		false
	})
}

fn event_id_value(attrs: &[syn::Attribute]) -> Vec<u8>{
    let event_attr = attrs.iter().find(|attr| {
        if let Some(first_seg) = attr.path.segments.first() {
            first_seg.value().ident == ATTR_EVENT
        } else {
            false
        }
    });
    match event_attr {        
        None => panic!("Event not found"),
        Some(attr) => {
            let result_str: String;
            let mut iter = attr.clone().tts.into_iter();
            match iter.next() {
                Some(proc_macro2::TokenTree::Group(group)) => {
                    if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                        panic!("event paranthesis expected");
                    }
                    let mut iter2 = group.stream().into_iter();
                    match iter2.next() {
                        Some(proc_macro2::TokenTree::Literal(lit)) => {
                            let str_val = lit.to_string();
                            if !str_val.starts_with("\"0x") || !str_val.ends_with("\"") {
                                panic!("string literal expected in event id");
                            }
                            if str_val.len() != 64 + 4 {
                                panic!("event id should be 64 characters long");
                            }
                            let substr = &str_val[3..str_val.len()-1];
                            result_str = substr.to_string();
                        },
                        _ => panic!("literal expected as event identifier")
                    }
                },
                _ => panic!("missing event identifier")
            }

            if let Some(_) = iter.next() {
                panic!("event too many tokens in event attribute");
            }
            
            match hex::decode(result_str) {
                Ok(v) => v,
                Err(_) => panic!("could not parse event id"),
            }
        }
    }
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
                        "Vec" =>
                            quote!{
                                let #pat: Vec<u8> = self.api.get_argument_vec(#arg_index_i32);
                            },
                        "BigInt" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                            },
                        "BI" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                            },
                        "BU" =>
                            quote!{
                                let #pat = self.api.get_argument_big_int_unsigned(#arg_index_i32);
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
                                "Address" =>
                                    quote!{
                                        let #pat: Address = self.api.get_argument_address(#arg_index_i32);
                                    },
                                "Vec" =>
                                    quote!{
                                        let #pat: Vec<u8> = self.api.get_argument_vec(#arg_index_i32);
                                    },
                                "BigInt" =>
                                    quote!{
                                        let #pat: BigInt = self.api.get_argument_big_int_signed(#arg_index_i32);
                                    },
                                "BI" =>
                                    quote!{
                                        let #pat = self.api.get_argument_big_int_signed(#arg_index_i32);
                                    },
                                "BU" =>
                                    quote!{
                                        let #pat = self.api.get_argument_big_int_unsigned(#arg_index_i32);
                                    },
                                other_stype_str => {
                                    panic!("Unsupported reference argument type: {:?}", other_stype_str)
                                }
                            }
                        },
                        _ => {
                            panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                        }
                    }
                    
                },
                other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
			}
        }
        other_arg => panic!("Unsupported argument type: {:?}, not captured", other_arg)
    }
}

fn generate_result_finish_snippet(result_ident: &syn::Ident, ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {                
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Result" => {    
                    match &type_path_segment.arguments {
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                            if args.len() != 2 {
                                panic!("Result type must have exactly 2 generic type arguments");
                            }

                            if let (syn::GenericArgument::Type(result_type), syn::GenericArgument::Type(err_type)) =
                                   (args.first().unwrap().into_value(), args.last().unwrap().into_value()) {
                                let ok_res_ident = syn::Ident::new("ok_res", proc_macro2::Span::call_site());
                                let ok_snippet = generate_result_finish_snippet(&ok_res_ident, result_type);
                                let err_res_ident = syn::Ident::new("err_res", proc_macro2::Span::call_site());
                                let err_snippet = generate_result_err_snippet(&err_res_ident, err_type);

                                quote!{
                                    match #result_ident {
                                        Ok(#ok_res_ident) => {
                                            #ok_snippet
                                        },
                                        Err(#err_res_ident) => {
                                            #err_snippet
                                        }
                                    }
                                }                                
                            } else {
                                panic!("Result type arguments must be types")
                            }
                        },
                        _ => panic!("Result angle brackets expected")
                    }
                    
                },
                "Address" =>
                    quote!{
                        self.api.finish(&#result_ident[0], 32);
                    },
                "Vec" => // TODO: better solution here, must capture type argument <u8>
                    quote!{
                        self.api.finish_vec(#result_ident);
                    },
                "BigInt" =>
                    quote!{
                        self.api.finish_big_int_signed(#result_ident);
                    },
                "BI" =>
                    quote!{
                        self.api.finish_big_int_signed(#result_ident);
                    },
                "BU" =>
                    quote!{
                        self.api.finish_big_int_unsigned(#result_ident);
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
        syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
            let mut i = 0;
            let tuple_snippets = elems.iter().map(|t| {
                let tuple_i=syn::Index::from(i);
                let temp_name = format!("tuple_{}", i);
                let temp_ident = syn::Ident::new(temp_name.as_str(), proc_macro2::Span::call_site());
                i = i + 1;
                let snippet = generate_result_finish_snippet(&temp_ident, t);
                quote!{ let #temp_ident = #result_ident.#tuple_i; #snippet }
            });
            quote!{ #(#tuple_snippets)* }
        },
        other_type => panic!("Unsupported return type: {:#?}, not a path", other_type)
    }
}

fn generate_result_err_snippet(err_ident: &syn::Ident, _ty: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        let (message_ptr, message_len) = ErrorMessage::message_ptr_and_len(#err_ident);
        self.api.signal_error_raw(message_ptr, message_len);
    }
}

fn generate_body_with_result(return_type: &syn::ReturnType, mbody: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match return_type.clone() {
        syn::ReturnType::Default => quote!{#mbody;},
        syn::ReturnType::Type(_, ty) => {
            let result_ident = syn::Ident::new("result", proc_macro2::Span::call_site());
            let finish = generate_result_finish_snippet(&result_ident, &ty);
            quote!{
                let #result_ident = { #mbody };
                #finish
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

    fn generate_topic_conversion_code(&self, arg: &syn::FnArg, arg_index: usize) -> proc_macro2::TokenStream {
        match arg {
            syn::FnArg::SelfRef(ref selfref) => {
                if !selfref.mutability.is_none() || arg_index != 0 {
                    panic!("event method must have `&self` as its first argument.");
                }
                quote!{}
            },
            syn::FnArg::Captured(arg_captured) => {
                let pat = &arg_captured.pat;
                let ty = &arg_captured.ty;
                //let arg_index_i32 = arg_index as i32;
                match ty {                
                    syn::Type::Reference(type_reference) => {
                        if type_reference.mutability != None {
                            panic!("[Event topic] Mutable references not supported as contract method arguments");
                        }
                        match &*type_reference.elem {
                            syn::Type::Path(type_path) => {
                                let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                                match type_str.as_str() {
                                    "Address" =>
                                        quote!{
                                            #pat.copy_to_array(&mut topics[#arg_index]);
                                        },
                                    "BI" =>
                                        quote!{
                                            #pat.copy_to_array_big_endian_pad_right(&mut topics[#arg_index]);
                                        },
                                    other_stype_str => {
                                        panic!("[Event topic] Unsupported reference argument type: {:?}", other_stype_str)
                                    }
                                }
                            },
                            _ => {
                                panic!("[Event topic] Unsupported reference argument type: {:?}", type_reference)
                            }
                        }
                        
                    },
                    other_arg => panic!("[Event topic] Unsupported argument type: {:?}, should be reference", other_arg)
                }
            }
            other_arg => panic!("[Event topic] Unsupported argument type: {:?}, not captured", other_arg)
        }
    }

    fn generate_event_data_conversion_code(&self, arg: &syn::FnArg, arg_index: i32) -> proc_macro2::TokenStream {
        match arg {
            syn::FnArg::SelfRef(ref selfref) => {
                if !selfref.mutability.is_none() || arg_index != 0 {
                    panic!("[Event data] method must have `&self` as its first argument.");
                }
                quote!{}
            },
            syn::FnArg::Captured(arg_captured) => {
                let pat = &arg_captured.pat;
                let ty = &arg_captured.ty;
                match ty {                
                    syn::Type::Reference(type_reference) => {
                        if type_reference.mutability != None {
                            panic!("[Event data] Mutable references not supported as contract method arguments");
                        }
                        match &*type_reference.elem {
                            syn::Type::Path(type_path) => {
                                let type_str = type_path.path.segments.last().unwrap().value().ident.to_string();
                                match type_str.as_str() {
                                    "BI" =>
                                        quote!{
                                            #pat.to_bytes_big_endian_pad_right(32)
                                        },
                                    other_stype_str => {
                                        panic!("[Event data] Unsupported reference argument type: {:?}", other_stype_str)
                                    }
                                }
                            },
                            _ => {
                                panic!("[Event data] Unsupported reference argument type: {:?}", type_reference)
                            }
                        }
                        
                    },
                    other_arg => panic!("[Event data] Unsupported argument type: {:?}, should be reference", other_arg)
                }
            }
            other_arg => panic!("[Event data] Unsupported argument type: {:?}, not captured", other_arg)
        }
    }

    fn generate_event_impl(&self, m: &syn::TraitItemMethod) -> proc_macro2::TokenStream {
        let msig = &m.sig;
        let nr_args_no_self = msig.decl.inputs.len() - 1;
        if nr_args_no_self == 0 {
            panic!("events need at least 1 argument, for the data");
        }
        let nr_topics = nr_args_no_self as usize; // -1 data, +1 event id

        let mut arg_index: usize = 0;
        let topic_conv_snippets: Vec<proc_macro2::TokenStream> = 
            msig.decl.inputs
                .iter()
                .map(|arg| {
                    let result =
                        if arg_index < nr_args_no_self {
                            let conversion = self.generate_topic_conversion_code(arg, arg_index);
                            quote! {
                                #conversion
                            }
                        } else {
                            let conversion = self.generate_event_data_conversion_code(arg, arg_index as i32);
                            quote! {
                                let data_vec = #conversion;
                            }
                        };
                    arg_index=arg_index+1;
                    result
                })
                .collect();
        let event_id_bytes = event_id_value(&m.attrs);
        quote! {
            #msig {
                let mut topics = [[0u8; 32]; #nr_topics];
                topics[0] =  [ #(#event_id_bytes),* ];
                #(#topic_conv_snippets)*
                self.write_log(&topics[..], &data_vec.as_slice());
            }
        }
    }

    pub fn generate_event_impls(&self) -> Vec<proc_macro2::TokenStream> {
        self.event_methods.iter().map(|m|
            self.generate_event_impl(m)
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
