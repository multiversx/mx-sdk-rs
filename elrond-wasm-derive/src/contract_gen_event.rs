
use super::parse_attr::*;
use super::util::*;

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
    let event_id_bytes = EventAttribute::parse(m).unwrap().identifier;
    let event_id_literal = array_literal(event_id_bytes.as_slice());
    quote! {
        #msig {
            let mut topics = [[0u8; 32]; #nr_topics];
            topics[0] = #event_id_literal;
            #(#topic_conv_snippets)*
            self.api.write_log(&topics[..], &data_vec.as_slice());
        }
    }
}