use super::contract_gen_method::*;
use super::contract_gen_arg::*;
//use super::parse_attr::*;
use super::util::*;

fn generate_topic_conversion_code(topic_index: usize, arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("[Event topic] Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().ident.to_string();
                    match type_str.as_str() {
                        "Address" =>
                            quote!{
                                #pat.copy_to_array(&mut topics[#topic_index]);
                            },
                        "BigInt" =>
                            panic!("[Event data] BigInt argument type currently not supported"),
                        "BigUint" =>
                            quote!{
                                #pat.copy_to_array_big_endian_pad_right(&mut topics[#topic_index]);
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

fn generate_event_data_conversion_code(arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {            
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("[Event data] Mutable references not supported as event arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().ident.to_string();
                    match type_str.as_str() {
                        "BigInt" =>
                            panic!("[Event data] BigInt argument type currently not supported"),
                        "BigUint" =>
                            quote!{
                                #pat.to_bytes_be_pad_right(32).unwrap()
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
        syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
            // allow empty tuple as event data
            if elems.len() == 0 {
                quote! {
                    Vec::with_capacity(0)
                }
            } else {
                panic!("Only empty tuples accepted as event data")
            }
        },
        other_arg => panic!("[Event data] Unsupported argument type: {:?}, should be reference", other_arg)
    }
}

pub fn generate_event_impl(m: &Method, event_id_bytes: Vec<u8>) -> proc_macro2::TokenStream {
    let nr_args_no_self = m.method_args.len();
    if nr_args_no_self == 0 {
        panic!("events need at least 1 argument, for the data");
    }
    let nr_topics = nr_args_no_self as usize; // -1 data, +1 event id

    let mut topic_index: usize = 1;
    let topic_conv_snippets: Vec<proc_macro2::TokenStream> = 
        m.method_args
            .iter()
            .map(|arg| {
                let result =
                    if topic_index < nr_args_no_self {
                        let conversion = generate_topic_conversion_code(topic_index, arg);
                        quote! {
                            #conversion
                        }
                    } else {
                        let conversion = generate_event_data_conversion_code(arg);
                        quote! {
                            let data_vec = #conversion;
                        }
                    };
                topic_index += 1;
                result
            })
            .collect();
    let msig = m.generate_sig();
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
