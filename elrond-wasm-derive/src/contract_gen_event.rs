
use super::contract_gen::ATTR_EVENT;

// parses "event attribute"
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

fn generate_topic_conversion_code(arg: &syn::FnArg, arg_index: usize) -> proc_macro2::TokenStream {
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
                                "BigInt" =>
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

fn generate_event_data_conversion_code(arg: &syn::FnArg, arg_index: i32) -> proc_macro2::TokenStream {
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
                                "BigInt" =>
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

pub fn generate_event_impl(m: &syn::TraitItemMethod) -> proc_macro2::TokenStream {
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
                        let conversion = generate_topic_conversion_code(arg, arg_index);
                        quote! {
                            #conversion
                        }
                    } else {
                        let conversion = generate_event_data_conversion_code(arg, arg_index as i32);
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
            self.api.write_log(&topics[..], &data_vec.as_slice());
        }
    }
}